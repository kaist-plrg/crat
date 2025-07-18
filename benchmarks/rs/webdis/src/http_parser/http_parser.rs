use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uchar", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uchar", bits = "2..=7")]
    pub type_0_flags: [u8; 1],
    pub state: libc::c_uchar,
    pub header_state: libc::c_uchar,
    pub index: libc::c_uchar,
    pub nread: uint32_t,
    pub content_length: int64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    pub status_code: libc::c_ushort,
    pub method: libc::c_uchar,
    pub upgrade: libc::c_char,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_path: http_data_cb,
    pub on_query_string: http_data_cb,
    pub on_url: http_data_cb,
    pub on_fragment: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
}
pub type http_cb = Option::<unsafe extern "C" fn(*mut http_parser) -> libc::c_int>;
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type http_method = libc::c_uint;
pub const HTTP_UNSUBSCRIBE: http_method = 22;
pub const HTTP_SUBSCRIBE: http_method = 21;
pub const HTTP_NOTIFY: http_method = 20;
pub const HTTP_MSEARCH: http_method = 19;
pub const HTTP_MERGE: http_method = 18;
pub const HTTP_CHECKOUT: http_method = 17;
pub const HTTP_MKACTIVITY: http_method = 16;
pub const HTTP_REPORT: http_method = 15;
pub const HTTP_UNLOCK: http_method = 14;
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
pub const F_SKIPBODY: flags = 32;
pub const F_UPGRADE: flags = 16;
pub const F_TRAILING: flags = 8;
pub const F_CONNECTION_CLOSE: flags = 4;
pub const F_CONNECTION_KEEP_ALIVE: flags = 2;
pub const F_CHUNKED: flags = 1;
pub const s_start_req_or_res: state = 2;
pub const s_start_res: state = 4;
pub const s_start_req: state = 17;
pub const s_dead: state = 1;
pub type header_states = libc::c_uint;
pub const h_connection_close: header_states = 18;
pub const h_connection_keep_alive: header_states = 17;
pub const h_transfer_encoding_chunked: header_states = 16;
pub const h_matching_connection_close: header_states = 15;
pub const h_matching_connection_keep_alive: header_states = 14;
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
pub type state = libc::c_uint;
pub const s_body_identity_eof: state = 54;
pub const s_body_identity: state = 53;
pub const s_chunk_data_done: state = 52;
pub const s_chunk_data_almost_done: state = 51;
pub const s_chunk_data: state = 50;
pub const s_chunk_parameters: state = 49;
pub const s_chunk_size_almost_done: state = 48;
pub const s_chunk_size: state = 47;
pub const s_chunk_size_start: state = 46;
pub const s_headers_almost_done: state = 45;
pub const s_header_almost_done: state = 44;
pub const s_header_value: state = 43;
pub const s_header_value_start: state = 42;
pub const s_header_field: state = 41;
pub const s_header_field_start: state = 40;
pub const s_req_line_almost_done: state = 39;
pub const s_req_http_minor: state = 38;
pub const s_req_first_http_minor: state = 37;
pub const s_req_http_major: state = 36;
pub const s_req_first_http_major: state = 35;
pub const s_req_http_HTTP: state = 34;
pub const s_req_http_HTT: state = 33;
pub const s_req_http_HT: state = 32;
pub const s_req_http_H: state = 31;
pub const s_req_http_start: state = 30;
pub const s_req_fragment: state = 29;
pub const s_req_fragment_start: state = 28;
pub const s_req_query_string: state = 27;
pub const s_req_query_string_start: state = 26;
pub const s_req_path: state = 25;
pub const s_req_port: state = 24;
pub const s_req_host: state = 23;
pub const s_req_schema_slash_slash: state = 22;
pub const s_req_schema_slash: state = 21;
pub const s_req_schema: state = 20;
pub const s_req_spaces_before_url: state = 19;
pub const s_req_method: state = 18;
pub const s_res_line_almost_done: state = 16;
pub const s_res_status: state = 15;
pub const s_res_status_code: state = 14;
pub const s_res_first_status_code: state = 13;
pub const s_res_http_minor: state = 12;
pub const s_res_first_http_minor: state = 11;
pub const s_res_http_major: state = 10;
pub const s_res_first_http_major: state = 9;
pub const s_res_HTTP: state = 8;
pub const s_res_HTT: state = 7;
pub const s_res_HT: state = 6;
pub const s_res_H: state = 5;
pub const s_res_or_resp_H: state = 3;
static mut method_strings: [*const libc::c_char; 23] = [
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
    b"UNLOCK\0" as *const u8 as *const libc::c_char,
    b"REPORT\0" as *const u8 as *const libc::c_char,
    b"MKACTIVITY\0" as *const u8 as *const libc::c_char,
    b"CHECKOUT\0" as *const u8 as *const libc::c_char,
    b"MERGE\0" as *const u8 as *const libc::c_char,
    b"M-SEARCH\0" as *const u8 as *const libc::c_char,
    b"NOTIFY\0" as *const u8 as *const libc::c_char,
    b"SUBSCRIBE\0" as *const u8 as *const libc::c_char,
    b"UNSUBSCRIBE\0" as *const u8 as *const libc::c_char,
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
    ' ' as i32 as libc::c_char,
    '!' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
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
    '/' as i32 as libc::c_char,
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
    '}' as i32 as libc::c_char,
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
static mut normal_url_char: [uint8_t; 256] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
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
pub unsafe extern "C" fn http_parser_execute(
    mut parser: *mut http_parser,
    mut settings: *const http_parser_settings,
    mut data: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut ch: libc::c_char = 0;
    let mut p: *const libc::c_char = data;
    let mut pe: *const libc::c_char = 0 as *const libc::c_char;
    let mut to_read: int64_t = 0;
    let mut state: state = (*parser).state as state;
    let mut header_state: header_states = (*parser).header_state as header_states;
    let mut index: uint64_t = (*parser).index as uint64_t;
    let mut nread: uint64_t = (*parser).nread as uint64_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        if state as libc::c_uint == s_body_identity_eof as libc::c_int as libc::c_uint {
            if ((*settings).on_message_complete).is_some() {
                if 0 as libc::c_int != ((*settings).on_message_complete).unwrap()(parser)
                {
                    return p.offset_from(data) as libc::c_long as size_t;
                }
            }
        }
        return 0 as libc::c_int as size_t;
    }
    let mut header_field_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut header_value_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut fragment_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut query_string_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut url_mark: *const libc::c_char = 0 as *const libc::c_char;
    if state as libc::c_uint == s_header_field as libc::c_int as libc::c_uint {
        header_field_mark = data;
    }
    if state as libc::c_uint == s_header_value as libc::c_int as libc::c_uint {
        header_value_mark = data;
    }
    if state as libc::c_uint == s_req_fragment as libc::c_int as libc::c_uint {
        fragment_mark = data;
    }
    if state as libc::c_uint == s_req_query_string as libc::c_int as libc::c_uint {
        query_string_mark = data;
    }
    if state as libc::c_uint == s_req_path as libc::c_int as libc::c_uint {
        path_mark = data;
    }
    if state as libc::c_uint == s_req_path as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_schema as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_schema_slash as libc::c_int as libc::c_uint
        || state as libc::c_uint
            == s_req_schema_slash_slash as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_port as libc::c_int as libc::c_uint
        || state as libc::c_uint
            == s_req_query_string_start as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_query_string as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_host as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_fragment_start as libc::c_int as libc::c_uint
        || state as libc::c_uint == s_req_fragment as libc::c_int as libc::c_uint
    {
        url_mark = data;
    }
    p = data;
    pe = data.offset(len as isize);
    loop {
        if !(p != pe) {
            current_block = 10962706839408844679;
            break;
        }
        ch = *p;
        if state as libc::c_uint <= s_headers_almost_done as libc::c_int as libc::c_uint
            && 0 as libc::c_int
                == (*parser).flags() as libc::c_int & F_TRAILING as libc::c_int
        {
            nread = nread.wrapping_add(1);
            nread;
            if nread > (80 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
                current_block = 6821533312007271109;
                break;
            }
        }
        match state as libc::c_uint {
            1 => {
                current_block = 6821533312007271109;
                break;
            }
            2 => {
                if ch as libc::c_int == '\r' as i32 || ch as libc::c_int == '\n' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    (*parser).set_flags(0 as libc::c_int as libc::c_uchar);
                    (*parser).content_length = -(1 as libc::c_int) as int64_t;
                    if ((*settings).on_message_begin).is_some() {
                        if 0 as libc::c_int
                            != ((*settings).on_message_begin).unwrap()(parser)
                        {
                            return p.offset_from(data) as libc::c_long as size_t;
                        }
                    }
                    if ch as libc::c_int == 'H' as i32 {
                        state = s_res_or_resp_H;
                        current_block = 13472856163611868459;
                    } else {
                        (*parser)
                            .set_type_0(HTTP_REQUEST as libc::c_int as libc::c_uchar);
                        current_block = 3610007317786028845;
                    }
                }
            }
            3 => {
                if ch as libc::c_int == 'T' as i32 {
                    (*parser).set_type_0(HTTP_RESPONSE as libc::c_int as libc::c_uchar);
                    state = s_res_HT;
                } else {
                    if ch as libc::c_int != 'E' as i32 {
                        current_block = 6821533312007271109;
                        break;
                    }
                    (*parser).set_type_0(HTTP_REQUEST as libc::c_int as libc::c_uchar);
                    (*parser).method = HTTP_HEAD as libc::c_int as libc::c_uchar;
                    index = 2 as libc::c_int as uint64_t;
                    state = s_req_method;
                }
                current_block = 13472856163611868459;
            }
            4 => {
                (*parser).set_flags(0 as libc::c_int as libc::c_uchar);
                (*parser).content_length = -(1 as libc::c_int) as int64_t;
                if ((*settings).on_message_begin).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_message_begin).unwrap()(parser)
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                match ch as libc::c_int {
                    72 => {
                        state = s_res_H;
                        current_block = 13472856163611868459;
                    }
                    13 | 10 => {
                        current_block = 13472856163611868459;
                    }
                    _ => {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
            }
            5 => {
                if ch as libc::c_int != 'T' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_res_HT;
                current_block = 13472856163611868459;
            }
            6 => {
                if ch as libc::c_int != 'T' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_res_HTT;
                current_block = 13472856163611868459;
            }
            7 => {
                if ch as libc::c_int != 'P' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_res_HTTP;
                current_block = 13472856163611868459;
            }
            8 => {
                if ch as libc::c_int != '/' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_res_first_http_major;
                current_block = 13472856163611868459;
            }
            9 => {
                if (ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                (*parser)
                    .http_major = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                state = s_res_http_major;
                current_block = 13472856163611868459;
            }
            10 => {
                if ch as libc::c_int == '.' as i32 {
                    state = s_res_first_http_minor;
                } else {
                    if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32
                    {
                        current_block = 6821533312007271109;
                        break;
                    }
                    (*parser)
                        .http_major = ((*parser).http_major as libc::c_int
                        * 10 as libc::c_int) as libc::c_ushort;
                    (*parser)
                        .http_major = ((*parser).http_major as libc::c_int
                        + (ch as libc::c_int - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_major as libc::c_int > 999 as libc::c_int {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            11 => {
                if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                (*parser)
                    .http_minor = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                state = s_res_http_minor;
                current_block = 13472856163611868459;
            }
            12 => {
                if ch as libc::c_int == ' ' as i32 {
                    state = s_res_first_status_code;
                } else {
                    if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32
                    {
                        current_block = 6821533312007271109;
                        break;
                    }
                    (*parser)
                        .http_minor = ((*parser).http_minor as libc::c_int
                        * 10 as libc::c_int) as libc::c_ushort;
                    (*parser)
                        .http_minor = ((*parser).http_minor as libc::c_int
                        + (ch as libc::c_int - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_minor as libc::c_int > 999 as libc::c_int {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            13 => {
                if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                    if !(ch as libc::c_int == ' ' as i32) {
                        current_block = 6821533312007271109;
                        break;
                    }
                } else {
                    (*parser)
                        .status_code = (ch as libc::c_int - '0' as i32)
                        as libc::c_ushort;
                    state = s_res_status_code;
                }
                current_block = 13472856163611868459;
            }
            14 => {
                if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                    match ch as libc::c_int {
                        32 => {
                            state = s_res_status;
                        }
                        13 => {
                            state = s_res_line_almost_done;
                        }
                        10 => {
                            state = s_header_field_start;
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                } else {
                    (*parser)
                        .status_code = ((*parser).status_code as libc::c_int
                        * 10 as libc::c_int) as libc::c_ushort;
                    (*parser)
                        .status_code = ((*parser).status_code as libc::c_int
                        + (ch as libc::c_int - '0' as i32)) as libc::c_ushort;
                    if (*parser).status_code as libc::c_int > 999 as libc::c_int {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            15 => {
                if ch as libc::c_int == '\r' as i32 {
                    state = s_res_line_almost_done;
                } else if ch as libc::c_int == '\n' as i32 {
                    state = s_header_field_start;
                }
                current_block = 13472856163611868459;
            }
            16 => {
                if ch as libc::c_int != '\n' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_header_field_start;
                current_block = 13472856163611868459;
            }
            17 => {
                if ch as libc::c_int == '\r' as i32 || ch as libc::c_int == '\n' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    (*parser).set_flags(0 as libc::c_int as libc::c_uchar);
                    (*parser).content_length = -(1 as libc::c_int) as int64_t;
                    if ((*settings).on_message_begin).is_some() {
                        if 0 as libc::c_int
                            != ((*settings).on_message_begin).unwrap()(parser)
                        {
                            return p.offset_from(data) as libc::c_long as size_t;
                        }
                    }
                    if (ch as libc::c_int) < 'A' as i32
                        || ('Z' as i32) < ch as libc::c_int
                    {
                        current_block = 6821533312007271109;
                        break;
                    }
                    current_block = 3610007317786028845;
                }
            }
            18 => {
                if ch as libc::c_int == '\0' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                let mut matcher: *const libc::c_char = method_strings[(*parser).method
                    as usize];
                if ch as libc::c_int == ' ' as i32
                    && *matcher.offset(index as isize) as libc::c_int == '\0' as i32
                {
                    state = s_req_spaces_before_url;
                } else if !(ch as libc::c_int
                    == *matcher.offset(index as isize) as libc::c_int)
                {
                    if (*parser).method as libc::c_int == HTTP_CONNECT as libc::c_int {
                        if index == 1 as libc::c_int as libc::c_ulong
                            && ch as libc::c_int == 'H' as i32
                        {
                            (*parser)
                                .method = HTTP_CHECKOUT as libc::c_int as libc::c_uchar;
                        } else if index == 2 as libc::c_int as libc::c_ulong
                            && ch as libc::c_int == 'P' as i32
                        {
                            (*parser).method = HTTP_COPY as libc::c_int as libc::c_uchar;
                        }
                    } else if (*parser).method as libc::c_int
                        == HTTP_MKCOL as libc::c_int
                    {
                        if index == 1 as libc::c_int as libc::c_ulong
                            && ch as libc::c_int == 'O' as i32
                        {
                            (*parser).method = HTTP_MOVE as libc::c_int as libc::c_uchar;
                        } else if index == 1 as libc::c_int as libc::c_ulong
                            && ch as libc::c_int == 'E' as i32
                        {
                            (*parser)
                                .method = HTTP_MERGE as libc::c_int as libc::c_uchar;
                        } else if index == 1 as libc::c_int as libc::c_ulong
                            && ch as libc::c_int == '-' as i32
                        {
                            (*parser)
                                .method = HTTP_MSEARCH as libc::c_int as libc::c_uchar;
                        } else if index == 2 as libc::c_int as libc::c_ulong
                            && ch as libc::c_int == 'A' as i32
                        {
                            (*parser)
                                .method = HTTP_MKACTIVITY as libc::c_int as libc::c_uchar;
                        }
                    } else if index == 1 as libc::c_int as libc::c_ulong
                        && (*parser).method as libc::c_int == HTTP_POST as libc::c_int
                        && ch as libc::c_int == 'R' as i32
                    {
                        (*parser).method = HTTP_PROPFIND as libc::c_int as libc::c_uchar;
                    } else if index == 1 as libc::c_int as libc::c_ulong
                        && (*parser).method as libc::c_int == HTTP_POST as libc::c_int
                        && ch as libc::c_int == 'U' as i32
                    {
                        (*parser).method = HTTP_PUT as libc::c_int as libc::c_uchar;
                    } else if index == 2 as libc::c_int as libc::c_ulong
                        && (*parser).method as libc::c_int == HTTP_UNLOCK as libc::c_int
                        && ch as libc::c_int == 'S' as i32
                    {
                        (*parser)
                            .method = HTTP_UNSUBSCRIBE as libc::c_int as libc::c_uchar;
                    } else {
                        if !(index == 4 as libc::c_int as libc::c_ulong
                            && (*parser).method as libc::c_int
                                == HTTP_PROPFIND as libc::c_int
                            && ch as libc::c_int == 'P' as i32)
                        {
                            current_block = 6821533312007271109;
                            break;
                        }
                        (*parser)
                            .method = HTTP_PROPPATCH as libc::c_int as libc::c_uchar;
                    }
                }
                index = index.wrapping_add(1);
                index;
                current_block = 13472856163611868459;
            }
            19 => {
                if ch as libc::c_int == ' ' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    if ch as libc::c_int == '/' as i32 || ch as libc::c_int == '*' as i32
                    {
                        url_mark = p;
                        path_mark = p;
                        state = s_req_path;
                    } else {
                        c = (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                            as libc::c_char;
                        if !(c as libc::c_int >= 'a' as i32
                            && c as libc::c_int <= 'z' as i32)
                        {
                            current_block = 6821533312007271109;
                            break;
                        }
                        url_mark = p;
                        state = s_req_schema;
                    }
                    current_block = 13472856163611868459;
                }
            }
            20 => {
                c = (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_char;
                if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    if ch as libc::c_int == ':' as i32 {
                        state = s_req_schema_slash;
                    } else if ch as libc::c_int == '.' as i32 {
                        state = s_req_host;
                    } else {
                        if !('0' as i32 <= ch as libc::c_int
                            && ch as libc::c_int <= '9' as i32)
                        {
                            current_block = 6821533312007271109;
                            break;
                        }
                        state = s_req_host;
                    }
                    current_block = 13472856163611868459;
                }
            }
            21 => {
                if ch as libc::c_int != '/' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_req_schema_slash_slash;
                current_block = 13472856163611868459;
            }
            22 => {
                if ch as libc::c_int != '/' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_req_host;
                current_block = 13472856163611868459;
            }
            23 => {
                c = (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_char;
                if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
                    current_block = 13472856163611868459;
                } else if ch as libc::c_int >= '0' as i32
                    && ch as libc::c_int <= '9' as i32 || ch as libc::c_int == '.' as i32
                    || ch as libc::c_int == '-' as i32
                {
                    current_block = 13472856163611868459;
                } else {
                    match ch as libc::c_int {
                        58 => {
                            state = s_req_port;
                        }
                        47 => {
                            path_mark = p;
                            state = s_req_path;
                        }
                        32 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_url)
                                            .unwrap()(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const libc::c_char;
                            state = s_req_http_start;
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            24 => {
                if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as libc::c_int {
                        47 => {
                            path_mark = p;
                            state = s_req_path;
                        }
                        32 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_url)
                                            .unwrap()(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const libc::c_char;
                            state = s_req_http_start;
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            25 => {
                if normal_url_char[ch as libc::c_uchar as usize] != 0 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as libc::c_int {
                        32 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_url)
                                            .unwrap()(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const libc::c_char;
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_path)
                                            .unwrap()(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const libc::c_char;
                            state = s_req_http_start;
                        }
                        13 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_url)
                                            .unwrap()(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const libc::c_char;
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_path)
                                            .unwrap()(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const libc::c_char;
                            (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                            (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                            state = s_req_line_almost_done;
                        }
                        10 => {
                            if !url_mark.is_null() {
                                if ((*settings).on_url).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_url)
                                            .unwrap()(
                                            parser,
                                            url_mark,
                                            p.offset_from(url_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            url_mark = 0 as *const libc::c_char;
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_path)
                                            .unwrap()(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const libc::c_char;
                            (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                            (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                            state = s_header_field_start;
                        }
                        63 => {
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_path)
                                            .unwrap()(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const libc::c_char;
                            state = s_req_query_string_start;
                        }
                        35 => {
                            if !path_mark.is_null() {
                                if ((*settings).on_path).is_some() {
                                    if 0 as libc::c_int
                                        != ((*settings).on_path)
                                            .unwrap()(
                                            parser,
                                            path_mark,
                                            p.offset_from(path_mark) as libc::c_long as size_t,
                                        )
                                    {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                            }
                            path_mark = 0 as *const libc::c_char;
                            state = s_req_fragment_start;
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            26 => {
                if normal_url_char[ch as libc::c_uchar as usize] != 0 {
                    query_string_mark = p;
                    state = s_req_query_string;
                } else {
                    match ch as libc::c_int {
                        63 => {}
                        32 => {
                            current_block = 18252126646788143160;
                            match current_block {
                                13693545634078317383 => {
                                    state = s_req_fragment_start;
                                }
                                17705882666338532246 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                18252126646788143160 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        13 => {
                            current_block = 17705882666338532246;
                            match current_block {
                                13693545634078317383 => {
                                    state = s_req_fragment_start;
                                }
                                17705882666338532246 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                18252126646788143160 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        10 => {
                            current_block = 3527918385265024866;
                            match current_block {
                                13693545634078317383 => {
                                    state = s_req_fragment_start;
                                }
                                17705882666338532246 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                18252126646788143160 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        35 => {
                            current_block = 13693545634078317383;
                            match current_block {
                                13693545634078317383 => {
                                    state = s_req_fragment_start;
                                }
                                17705882666338532246 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                18252126646788143160 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                }
                current_block = 13472856163611868459;
            }
            27 => {
                if normal_url_char[ch as libc::c_uchar as usize] != 0 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as libc::c_int {
                        63 => {
                            current_block = 13472856163611868459;
                        }
                        32 => {
                            current_block = 13068256419887880769;
                            match current_block {
                                2939615421586548314 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_fragment_start;
                                }
                                663624886940844279 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13068256419887880769 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        13 => {
                            current_block = 663624886940844279;
                            match current_block {
                                2939615421586548314 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_fragment_start;
                                }
                                663624886940844279 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13068256419887880769 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        10 => {
                            current_block = 11849595666524850565;
                            match current_block {
                                2939615421586548314 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_fragment_start;
                                }
                                663624886940844279 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13068256419887880769 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        35 => {
                            current_block = 2939615421586548314;
                            match current_block {
                                2939615421586548314 => {
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_fragment_start;
                                }
                                663624886940844279 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13068256419887880769 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !query_string_mark.is_null() {
                                        if ((*settings).on_query_string).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_query_string)
                                                    .unwrap()(
                                                    parser,
                                                    query_string_mark,
                                                    p.offset_from(query_string_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    query_string_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                }
            }
            28 => {
                if normal_url_char[ch as libc::c_uchar as usize] != 0 {
                    fragment_mark = p;
                    state = s_req_fragment;
                } else {
                    match ch as libc::c_int {
                        32 => {
                            current_block = 13088904190973355423;
                            match current_block {
                                16523802078244709528 => {
                                    fragment_mark = p;
                                    state = s_req_fragment;
                                }
                                5598348517352324960 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13088904190973355423 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        13 => {
                            current_block = 5598348517352324960;
                            match current_block {
                                16523802078244709528 => {
                                    fragment_mark = p;
                                    state = s_req_fragment;
                                }
                                5598348517352324960 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13088904190973355423 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        10 => {
                            current_block = 1965917335645526817;
                            match current_block {
                                16523802078244709528 => {
                                    fragment_mark = p;
                                    state = s_req_fragment;
                                }
                                5598348517352324960 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13088904190973355423 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        63 => {
                            current_block = 16523802078244709528;
                            match current_block {
                                16523802078244709528 => {
                                    fragment_mark = p;
                                    state = s_req_fragment;
                                }
                                5598348517352324960 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                13088904190973355423 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                        }
                        35 => {}
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                }
                current_block = 13472856163611868459;
            }
            29 => {
                if normal_url_char[ch as libc::c_uchar as usize] != 0 {
                    current_block = 13472856163611868459;
                } else {
                    match ch as libc::c_int {
                        32 => {
                            current_block = 9923390962991019052;
                            match current_block {
                                9923390962991019052 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                10477942228951044767 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        13 => {
                            current_block = 10477942228951044767;
                            match current_block {
                                9923390962991019052 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                10477942228951044767 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        10 => {
                            current_block = 8753412356531521447;
                            match current_block {
                                9923390962991019052 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    state = s_req_http_start;
                                }
                                10477942228951044767 => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_req_line_almost_done;
                                }
                                _ => {
                                    if !url_mark.is_null() {
                                        if ((*settings).on_url).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_url)
                                                    .unwrap()(
                                                    parser,
                                                    url_mark,
                                                    p.offset_from(url_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    url_mark = 0 as *const libc::c_char;
                                    if !fragment_mark.is_null() {
                                        if ((*settings).on_fragment).is_some() {
                                            if 0 as libc::c_int
                                                != ((*settings).on_fragment)
                                                    .unwrap()(
                                                    parser,
                                                    fragment_mark,
                                                    p.offset_from(fragment_mark) as libc::c_long as size_t,
                                                )
                                            {
                                                return p.offset_from(data) as libc::c_long as size_t;
                                            }
                                        }
                                    }
                                    fragment_mark = 0 as *const libc::c_char;
                                    (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                                    (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                                    state = s_header_field_start;
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        63 | 35 => {
                            current_block = 13472856163611868459;
                        }
                        _ => {
                            current_block = 6821533312007271109;
                            break;
                        }
                    }
                }
            }
            30 => {
                match ch as libc::c_int {
                    72 => {
                        state = s_req_http_H;
                        current_block = 13472856163611868459;
                    }
                    32 => {
                        current_block = 13472856163611868459;
                    }
                    _ => {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
            }
            31 => {
                if ch as libc::c_int != 'T' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_req_http_HT;
                current_block = 13472856163611868459;
            }
            32 => {
                if ch as libc::c_int != 'T' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_req_http_HTT;
                current_block = 13472856163611868459;
            }
            33 => {
                if ch as libc::c_int != 'P' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_req_http_HTTP;
                current_block = 13472856163611868459;
            }
            34 => {
                if ch as libc::c_int != '/' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_req_first_http_major;
                current_block = 13472856163611868459;
            }
            35 => {
                if (ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                (*parser)
                    .http_major = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                state = s_req_http_major;
                current_block = 13472856163611868459;
            }
            36 => {
                if ch as libc::c_int == '.' as i32 {
                    state = s_req_first_http_minor;
                } else {
                    if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32
                    {
                        current_block = 6821533312007271109;
                        break;
                    }
                    (*parser)
                        .http_major = ((*parser).http_major as libc::c_int
                        * 10 as libc::c_int) as libc::c_ushort;
                    (*parser)
                        .http_major = ((*parser).http_major as libc::c_int
                        + (ch as libc::c_int - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_major as libc::c_int > 999 as libc::c_int {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            37 => {
                if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                (*parser)
                    .http_minor = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                state = s_req_http_minor;
                current_block = 13472856163611868459;
            }
            38 => {
                if ch as libc::c_int == '\r' as i32 {
                    state = s_req_line_almost_done;
                } else if ch as libc::c_int == '\n' as i32 {
                    state = s_header_field_start;
                } else {
                    if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32
                    {
                        current_block = 6821533312007271109;
                        break;
                    }
                    (*parser)
                        .http_minor = ((*parser).http_minor as libc::c_int
                        * 10 as libc::c_int) as libc::c_ushort;
                    (*parser)
                        .http_minor = ((*parser).http_minor as libc::c_int
                        + (ch as libc::c_int - '0' as i32)) as libc::c_ushort;
                    if (*parser).http_minor as libc::c_int > 999 as libc::c_int {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
                current_block = 13472856163611868459;
            }
            39 => {
                if ch as libc::c_int != '\n' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_header_field_start;
                current_block = 13472856163611868459;
            }
            40 => {
                if ch as libc::c_int == '\r' as i32 {
                    state = s_headers_almost_done;
                    current_block = 13472856163611868459;
                } else if ch as libc::c_int == '\n' as i32 {
                    state = s_headers_almost_done;
                    current_block = 6436225509474489267;
                } else {
                    c = tokens[ch as libc::c_uchar as usize];
                    if c == 0 {
                        current_block = 6821533312007271109;
                        break;
                    }
                    header_field_mark = p;
                    index = 0 as libc::c_int as uint64_t;
                    state = s_header_field;
                    match c as libc::c_int {
                        99 => {
                            header_state = h_C;
                        }
                        112 => {
                            header_state = h_matching_proxy_connection;
                        }
                        116 => {
                            header_state = h_matching_transfer_encoding;
                        }
                        117 => {
                            header_state = h_matching_upgrade;
                        }
                        _ => {
                            header_state = h_general;
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            41 => {
                c = tokens[ch as libc::c_uchar as usize];
                if c != 0 {
                    match header_state as libc::c_uint {
                        0 => {}
                        1 => {
                            index = index.wrapping_add(1);
                            index;
                            header_state = (if c as libc::c_int == 'o' as i32 {
                                h_CO as libc::c_int
                            } else {
                                h_general as libc::c_int
                            }) as header_states;
                        }
                        2 => {
                            index = index.wrapping_add(1);
                            index;
                            header_state = (if c as libc::c_int == 'n' as i32 {
                                h_CON as libc::c_int
                            } else {
                                h_general as libc::c_int
                            }) as header_states;
                        }
                        3 => {
                            index = index.wrapping_add(1);
                            index;
                            match c as libc::c_int {
                                110 => {
                                    header_state = h_matching_connection;
                                }
                                116 => {
                                    header_state = h_matching_content_length;
                                }
                                _ => {
                                    header_state = h_general;
                                }
                            }
                        }
                        4 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::std::mem::size_of::<[libc::c_char; 11]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                || c as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 11],
                                        &[libc::c_char; 11],
                                    >(b"connection\0"))[index as usize] as libc::c_int
                            {
                                header_state = h_general;
                            } else if index
                                == (::std::mem::size_of::<[libc::c_char; 11]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            {
                                header_state = h_connection;
                            }
                        }
                        5 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::std::mem::size_of::<[libc::c_char; 17]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                || c as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 17],
                                        &[libc::c_char; 17],
                                    >(b"proxy-connection\0"))[index as usize] as libc::c_int
                            {
                                header_state = h_general;
                            } else if index
                                == (::std::mem::size_of::<[libc::c_char; 17]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            {
                                header_state = h_connection;
                            }
                        }
                        6 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::std::mem::size_of::<[libc::c_char; 15]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                || c as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 15],
                                        &[libc::c_char; 15],
                                    >(b"content-length\0"))[index as usize] as libc::c_int
                            {
                                header_state = h_general;
                            } else if index
                                == (::std::mem::size_of::<[libc::c_char; 15]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            {
                                header_state = h_content_length;
                            }
                        }
                        7 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::std::mem::size_of::<[libc::c_char; 18]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                || c as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 18],
                                        &[libc::c_char; 18],
                                    >(b"transfer-encoding\0"))[index as usize] as libc::c_int
                            {
                                header_state = h_general;
                            } else if index
                                == (::std::mem::size_of::<[libc::c_char; 18]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            {
                                header_state = h_transfer_encoding;
                            }
                        }
                        8 => {
                            index = index.wrapping_add(1);
                            index;
                            if index
                                > (::std::mem::size_of::<[libc::c_char; 8]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                || c as libc::c_int
                                    != (*::std::mem::transmute::<
                                        &[u8; 8],
                                        &[libc::c_char; 8],
                                    >(b"upgrade\0"))[index as usize] as libc::c_int
                            {
                                header_state = h_general;
                            } else if index
                                == (::std::mem::size_of::<[libc::c_char; 8]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            {
                                header_state = h_upgrade;
                            }
                        }
                        9 | 10 | 11 | 12 => {
                            if ch as libc::c_int != ' ' as i32 {
                                header_state = h_general;
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
                                    b"src/http-parser/http_parser.c\0" as *const u8
                                        as *const libc::c_char,
                                    1153 as libc::c_int as libc::c_uint,
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
                } else if ch as libc::c_int == ':' as i32 {
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_header_field)
                                    .unwrap()(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as libc::c_long as size_t,
                                )
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    }
                    header_field_mark = 0 as *const libc::c_char;
                    state = s_header_value_start;
                } else if ch as libc::c_int == '\r' as i32 {
                    state = s_header_almost_done;
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_header_field)
                                    .unwrap()(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as libc::c_long as size_t,
                                )
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    }
                    header_field_mark = 0 as *const libc::c_char;
                } else {
                    if !(ch as libc::c_int == '\n' as i32) {
                        current_block = 6821533312007271109;
                        break;
                    }
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_header_field)
                                    .unwrap()(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as libc::c_long as size_t,
                                )
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    }
                    header_field_mark = 0 as *const libc::c_char;
                    state = s_header_field_start;
                }
                current_block = 13472856163611868459;
            }
            42 => {
                if ch as libc::c_int == ' ' as i32 {
                    current_block = 13472856163611868459;
                } else {
                    header_value_mark = p;
                    state = s_header_value;
                    index = 0 as libc::c_int as uint64_t;
                    c = (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                        as libc::c_char;
                    if ch as libc::c_int == '\r' as i32 {
                        if !header_value_mark.is_null() {
                            if ((*settings).on_header_value).is_some() {
                                if 0 as libc::c_int
                                    != ((*settings).on_header_value)
                                        .unwrap()(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as libc::c_long as size_t,
                                    )
                                {
                                    return p.offset_from(data) as libc::c_long as size_t;
                                }
                            }
                        }
                        header_value_mark = 0 as *const libc::c_char;
                        header_state = h_general;
                        state = s_header_almost_done;
                    } else if ch as libc::c_int == '\n' as i32 {
                        if !header_value_mark.is_null() {
                            if ((*settings).on_header_value).is_some() {
                                if 0 as libc::c_int
                                    != ((*settings).on_header_value)
                                        .unwrap()(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as libc::c_long as size_t,
                                    )
                                {
                                    return p.offset_from(data) as libc::c_long as size_t;
                                }
                            }
                        }
                        header_value_mark = 0 as *const libc::c_char;
                        state = s_header_field_start;
                    } else {
                        match header_state as libc::c_uint {
                            12 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_UPGRADE as libc::c_int as libc::c_uchar,
                                    );
                                header_state = h_general;
                            }
                            11 => {
                                if 'c' as i32 == c as libc::c_int {
                                    header_state = h_matching_transfer_encoding_chunked;
                                } else {
                                    header_state = h_general;
                                }
                            }
                            10 => {
                                if (ch as libc::c_int) < '0' as i32
                                    || ch as libc::c_int > '9' as i32
                                {
                                    current_block = 6821533312007271109;
                                    break;
                                }
                                (*parser)
                                    .content_length = (ch as libc::c_int - '0' as i32)
                                    as int64_t;
                            }
                            9 => {
                                if c as libc::c_int == 'k' as i32 {
                                    header_state = h_matching_connection_keep_alive;
                                } else if c as libc::c_int == 'c' as i32 {
                                    header_state = h_matching_connection_close;
                                } else {
                                    header_state = h_general;
                                }
                            }
                            _ => {
                                header_state = h_general;
                            }
                        }
                    }
                    current_block = 13472856163611868459;
                }
            }
            43 => {
                c = (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_char;
                if ch as libc::c_int == '\r' as i32 {
                    if !header_value_mark.is_null() {
                        if ((*settings).on_header_value).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_header_value)
                                    .unwrap()(
                                    parser,
                                    header_value_mark,
                                    p.offset_from(header_value_mark) as libc::c_long as size_t,
                                )
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    }
                    header_value_mark = 0 as *const libc::c_char;
                    state = s_header_almost_done;
                    current_block = 13472856163611868459;
                } else if ch as libc::c_int == '\n' as i32 {
                    if !header_value_mark.is_null() {
                        if ((*settings).on_header_value).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_header_value)
                                    .unwrap()(
                                    parser,
                                    header_value_mark,
                                    p.offset_from(header_value_mark) as libc::c_long as size_t,
                                )
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    }
                    header_value_mark = 0 as *const libc::c_char;
                    current_block = 11020582689779776601;
                } else {
                    match header_state as libc::c_uint {
                        0 => {
                            current_block = 13472856163611868459;
                        }
                        9 | 11 => {
                            current_block = 13580586057089936640;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        10 => {
                            current_block = 17332133338080947575;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        13 => {
                            current_block = 14152827358935435035;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        14 => {
                            current_block = 1133543008428547523;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        15 => {
                            current_block = 5475236905426780108;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        16 | 17 | 18 => {
                            current_block = 10808458787776969121;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                        _ => {
                            current_block = 7466475382891248240;
                            match current_block {
                                10808458787776969121 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        header_state = h_general;
                                    }
                                }
                                1133543008428547523 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_keep_alive;
                                    }
                                }
                                14152827358935435035 => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_transfer_encoding_chunked;
                                    }
                                }
                                17332133338080947575 => {
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if (ch as libc::c_int) < '0' as i32
                                            || ch as libc::c_int > '9' as i32
                                        {
                                            current_block = 6821533312007271109;
                                            break;
                                        }
                                        (*parser).content_length
                                            *= 10 as libc::c_int as libc::c_long;
                                        (*parser).content_length
                                            += (ch as libc::c_int - '0' as i32) as libc::c_long;
                                    }
                                }
                                7466475382891248240 => {
                                    state = s_header_value;
                                    header_state = h_general;
                                }
                                13580586057089936640 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http-parser/http_parser.c\0" as *const u8
                                                as *const libc::c_char,
                                            1264 as libc::c_int as libc::c_uint,
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
                                _ => {
                                    index = index.wrapping_add(1);
                                    index;
                                    if index
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[index as usize] as libc::c_int
                                    {
                                        header_state = h_general;
                                    } else if index
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        header_state = h_connection_close;
                                    }
                                }
                            }
                            current_block = 13472856163611868459;
                        }
                    }
                }
            }
            44 => {
                current_block = 11020582689779776601;
            }
            45 => {
                current_block = 6436225509474489267;
            }
            53 => {
                to_read = if (pe.offset_from(p) as libc::c_long)
                    < (*parser).content_length
                {
                    pe.offset_from(p) as libc::c_long
                } else {
                    (*parser).content_length
                };
                if to_read > 0 as libc::c_int as libc::c_long {
                    if ((*settings).on_body).is_some() {
                        ((*settings).on_body).unwrap()(parser, p, to_read as size_t);
                    }
                    p = p.offset((to_read - 1 as libc::c_int as libc::c_long) as isize);
                    (*parser).content_length -= to_read;
                    if (*parser).content_length == 0 as libc::c_int as libc::c_long {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_message_complete).unwrap()(parser)
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
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
                    }
                }
                current_block = 13472856163611868459;
            }
            54 => {
                to_read = pe.offset_from(p) as libc::c_long;
                if to_read > 0 as libc::c_int as libc::c_long {
                    if ((*settings).on_body).is_some() {
                        ((*settings).on_body).unwrap()(parser, p, to_read as size_t);
                    }
                    p = p.offset((to_read - 1 as libc::c_int as libc::c_long) as isize);
                }
                current_block = 13472856163611868459;
            }
            46 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1440 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                c = unhex[ch as libc::c_uchar as usize] as libc::c_char;
                if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int
                {
                    current_block = 6821533312007271109;
                    break;
                }
                (*parser).content_length = c as int64_t;
                state = s_chunk_size;
                current_block = 13472856163611868459;
            }
            47 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1451 as libc::c_int as libc::c_uint,
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
                    state = s_chunk_size_almost_done;
                } else {
                    c = unhex[ch as libc::c_uchar as usize] as libc::c_char;
                    if c as libc::c_int
                        == -(1 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        if !(ch as libc::c_int == ';' as i32
                            || ch as libc::c_int == ' ' as i32)
                        {
                            current_block = 6821533312007271109;
                            break;
                        }
                        state = s_chunk_parameters;
                    } else {
                        (*parser).content_length *= 16 as libc::c_int as libc::c_long;
                        (*parser).content_length += c as libc::c_long;
                    }
                }
                current_block = 13472856163611868459;
            }
            49 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1475 as libc::c_int as libc::c_uint,
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
                    state = s_chunk_size_almost_done;
                    current_block = 13472856163611868459;
                } else {
                    current_block = 13472856163611868459;
                }
            }
            48 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1486 as libc::c_int as libc::c_uint,
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
                    current_block = 6821533312007271109;
                    break;
                }
                if (*parser).content_length == 0 as libc::c_int as libc::c_long {
                    (*parser)
                        .set_flags(
                            (*parser).flags()
                                | F_TRAILING as libc::c_int as libc::c_uchar,
                        );
                    state = s_header_field_start;
                } else {
                    state = s_chunk_data;
                }
                current_block = 13472856163611868459;
            }
            50 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1500 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                to_read = if (pe.offset_from(p) as libc::c_long)
                    < (*parser).content_length
                {
                    pe.offset_from(p) as libc::c_long
                } else {
                    (*parser).content_length
                };
                if to_read > 0 as libc::c_int as libc::c_long {
                    if ((*settings).on_body).is_some() {
                        ((*settings).on_body).unwrap()(parser, p, to_read as size_t);
                    }
                    p = p.offset((to_read - 1 as libc::c_int as libc::c_long) as isize);
                }
                if to_read == (*parser).content_length {
                    state = s_chunk_data_almost_done;
                }
                (*parser).content_length -= to_read;
                current_block = 13472856163611868459;
            }
            51 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1518 as libc::c_int as libc::c_uint,
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
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_chunk_data_done;
                current_block = 13472856163611868459;
            }
            52 => {
                if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                {} else {
                    __assert_fail(
                        b"parser->flags & F_CHUNKED\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1524 as libc::c_int as libc::c_uint,
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
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_chunk_size_start;
                current_block = 13472856163611868459;
            }
            _ => {
                if 0 as libc::c_int != 0
                    && !(b"unhandled state\0" as *const u8 as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"0 && \"unhandled state\"\0" as *const u8
                            as *const libc::c_char,
                        b"src/http-parser/http_parser.c\0" as *const u8
                            as *const libc::c_char,
                        1530 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                current_block = 6821533312007271109;
                break;
            }
        }
        match current_block {
            6436225509474489267 => {
                if ch as libc::c_int != '\n' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                if (*parser).flags() as libc::c_int & F_TRAILING as libc::c_int != 0 {
                    if ((*settings).on_message_complete).is_some() {
                        if 0 as libc::c_int
                            != ((*settings).on_message_complete).unwrap()(parser)
                        {
                            return p.offset_from(data) as libc::c_long as size_t;
                        }
                    }
                    state = (if http_should_keep_alive(parser) != 0 {
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
                } else {
                    nread = 0 as libc::c_int as uint64_t;
                    if (*parser).flags() as libc::c_int & F_UPGRADE as libc::c_int != 0
                        || (*parser).method as libc::c_int == HTTP_CONNECT as libc::c_int
                    {
                        (*parser).upgrade = 1 as libc::c_int as libc::c_char;
                    }
                    if ((*settings).on_headers_complete).is_some() {
                        match ((*settings).on_headers_complete).unwrap()(parser) {
                            0 => {}
                            1 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_SKIPBODY as libc::c_int as libc::c_uchar,
                                    );
                            }
                            _ => return p.offset_from(data) as libc::c_long as size_t,
                        }
                    }
                    if (*parser).upgrade != 0 {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_message_complete).unwrap()(parser)
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                    if (*parser).flags() as libc::c_int & F_SKIPBODY as libc::c_int != 0
                    {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_message_complete).unwrap()(parser)
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
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
                    } else if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int
                        != 0
                    {
                        state = s_chunk_size_start;
                    } else if (*parser).content_length
                        == 0 as libc::c_int as libc::c_long
                    {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_message_complete).unwrap()(parser)
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
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
                    } else if (*parser).content_length > 0 as libc::c_int as libc::c_long
                    {
                        state = s_body_identity;
                    } else if (*parser).type_0() as libc::c_int
                        == HTTP_REQUEST as libc::c_int
                        || http_should_keep_alive(parser) != 0
                    {
                        if ((*settings).on_message_complete).is_some() {
                            if 0 as libc::c_int
                                != ((*settings).on_message_complete).unwrap()(parser)
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        state = (if http_should_keep_alive(parser) != 0 {
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
                    } else {
                        state = s_body_identity_eof;
                    }
                }
            }
            11020582689779776601 => {
                if ch as libc::c_int != '\n' as i32 {
                    current_block = 6821533312007271109;
                    break;
                }
                state = s_header_field_start;
                match header_state as libc::c_uint {
                    17 => {
                        (*parser)
                            .set_flags(
                                (*parser).flags()
                                    | F_CONNECTION_KEEP_ALIVE as libc::c_int as libc::c_uchar,
                            );
                    }
                    18 => {
                        (*parser)
                            .set_flags(
                                (*parser).flags()
                                    | F_CONNECTION_CLOSE as libc::c_int as libc::c_uchar,
                            );
                    }
                    16 => {
                        (*parser)
                            .set_flags(
                                (*parser).flags()
                                    | F_CHUNKED as libc::c_int as libc::c_uchar,
                            );
                    }
                    _ => {}
                }
            }
            3610007317786028845 => {
                (*parser).method = HTTP_DELETE as libc::c_uchar;
                index = 1 as libc::c_int as uint64_t;
                match ch as libc::c_int {
                    67 => {
                        (*parser).method = HTTP_CONNECT as libc::c_int as libc::c_uchar;
                    }
                    68 => {
                        (*parser).method = HTTP_DELETE as libc::c_int as libc::c_uchar;
                    }
                    71 => {
                        (*parser).method = HTTP_GET as libc::c_int as libc::c_uchar;
                    }
                    72 => {
                        (*parser).method = HTTP_HEAD as libc::c_int as libc::c_uchar;
                    }
                    76 => {
                        (*parser).method = HTTP_LOCK as libc::c_int as libc::c_uchar;
                    }
                    77 => {
                        (*parser).method = HTTP_MKCOL as libc::c_int as libc::c_uchar;
                    }
                    78 => {
                        (*parser).method = HTTP_NOTIFY as libc::c_int as libc::c_uchar;
                    }
                    79 => {
                        (*parser).method = HTTP_OPTIONS as libc::c_int as libc::c_uchar;
                    }
                    80 => {
                        (*parser).method = HTTP_POST as libc::c_int as libc::c_uchar;
                    }
                    82 => {
                        (*parser).method = HTTP_REPORT as libc::c_int as libc::c_uchar;
                    }
                    83 => {
                        (*parser)
                            .method = HTTP_SUBSCRIBE as libc::c_int as libc::c_uchar;
                    }
                    84 => {
                        (*parser).method = HTTP_TRACE as libc::c_int as libc::c_uchar;
                    }
                    85 => {
                        (*parser).method = HTTP_UNLOCK as libc::c_int as libc::c_uchar;
                    }
                    _ => {
                        current_block = 6821533312007271109;
                        break;
                    }
                }
                state = s_req_method;
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        6821533312007271109 => {
            (*parser).state = s_dead as libc::c_int as libc::c_uchar;
            return p.offset_from(data) as libc::c_long as size_t;
        }
        _ => {
            if !header_field_mark.is_null() {
                if ((*settings).on_header_field).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_header_field)
                            .unwrap()(
                            parser,
                            header_field_mark,
                            p.offset_from(header_field_mark) as libc::c_long as size_t,
                        )
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
            }
            if !header_value_mark.is_null() {
                if ((*settings).on_header_value).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_header_value)
                            .unwrap()(
                            parser,
                            header_value_mark,
                            p.offset_from(header_value_mark) as libc::c_long as size_t,
                        )
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
            }
            if !fragment_mark.is_null() {
                if ((*settings).on_fragment).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_fragment)
                            .unwrap()(
                            parser,
                            fragment_mark,
                            p.offset_from(fragment_mark) as libc::c_long as size_t,
                        )
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
            }
            if !query_string_mark.is_null() {
                if ((*settings).on_query_string).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_query_string)
                            .unwrap()(
                            parser,
                            query_string_mark,
                            p.offset_from(query_string_mark) as libc::c_long as size_t,
                        )
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
            }
            if !path_mark.is_null() {
                if ((*settings).on_path).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_path)
                            .unwrap()(
                            parser,
                            path_mark,
                            p.offset_from(path_mark) as libc::c_long as size_t,
                        )
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
            }
            if !url_mark.is_null() {
                if ((*settings).on_url).is_some() {
                    if 0 as libc::c_int
                        != ((*settings).on_url)
                            .unwrap()(
                            parser,
                            url_mark,
                            p.offset_from(url_mark) as libc::c_long as size_t,
                        )
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
            }
            (*parser).state = state as libc::c_uchar;
            (*parser).header_state = header_state as libc::c_uchar;
            (*parser).index = index as libc::c_uchar;
            (*parser).nread = nread as uint32_t;
            return len;
        }
    };
}
pub unsafe extern "C" fn http_should_keep_alive(
    mut parser: *mut http_parser,
) -> libc::c_int {
    if (*parser).http_major as libc::c_int > 0 as libc::c_int
        && (*parser).http_minor as libc::c_int > 0 as libc::c_int
    {
        if (*parser).flags() as libc::c_int & F_CONNECTION_CLOSE as libc::c_int != 0 {
            return 0 as libc::c_int
        } else {
            return 1 as libc::c_int
        }
    } else if (*parser).flags() as libc::c_int & F_CONNECTION_KEEP_ALIVE as libc::c_int
        != 0
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn http_method_str(mut m: http_method) -> *const libc::c_char {
    return method_strings[m as usize];
}
pub unsafe extern "C" fn http_parser_init(
    mut parser: *mut http_parser,
    mut t: http_parser_type,
) {
    (*parser).set_type_0(t as libc::c_uchar);
    (*parser)
        .state = (if t as libc::c_uint == HTTP_REQUEST as libc::c_int as libc::c_uint {
        s_start_req as libc::c_int
    } else if t as libc::c_uint == HTTP_RESPONSE as libc::c_int as libc::c_uint {
        s_start_res as libc::c_int
    } else {
        s_start_req_or_res as libc::c_int
    }) as libc::c_uchar;
    (*parser).nread = 0 as libc::c_int as uint32_t;
    (*parser).upgrade = 0 as libc::c_int as libc::c_char;
    (*parser).set_flags(0 as libc::c_int as libc::c_uchar);
    (*parser).method = 0 as libc::c_int as libc::c_uchar;
}
