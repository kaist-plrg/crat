use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn osip_hash(str: *const libc::c_char) -> libc::c_ulong;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_message_set_www_authenticate(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_via(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_to(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_route(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_record_route(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_proxy_authorization(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_proxy_authentication_info(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_proxy_authenticate(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_mime_version(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_contact(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_content_length(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_call_id(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_from(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_error_info(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_content_encoding(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_cseq(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_content_type(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_call_info(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_authorization(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_authentication_info(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_allow(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_alert_info(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_accept_language(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_accept_encoding(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_accept(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
pub type _trace_level = libc::c_uint;
pub const END_TRACE_LEVEL: _trace_level = 8;
pub const TRACE_LEVEL7: _trace_level = 7;
pub const TRACE_LEVEL6: _trace_level = 6;
pub const TRACE_LEVEL5: _trace_level = 5;
pub const TRACE_LEVEL4: _trace_level = 4;
pub const TRACE_LEVEL3: _trace_level = 3;
pub const TRACE_LEVEL2: _trace_level = 2;
pub const TRACE_LEVEL1: _trace_level = 1;
pub const TRACE_LEVEL0: _trace_level = 0;
pub type osip_trace_level_t = _trace_level;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri {
    pub scheme: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub url_params: osip_list_t,
    pub url_headers: osip_list_t,
    pub string: *mut libc::c_char,
}
pub type osip_uri_t = osip_uri;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_type {
    pub type_0: *mut libc::c_char,
    pub subtype: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_content_type_t = osip_content_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_length {
    pub value: *mut libc::c_char,
}
pub type osip_content_length_t = osip_content_length;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_call_id {
    pub number: *mut libc::c_char,
    pub host: *mut libc::c_char,
}
pub type osip_call_id_t = osip_call_id;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_from {
    pub displayname: *mut libc::c_char,
    pub url: *mut osip_uri_t,
    pub gen_params: osip_list_t,
}
pub type osip_from_t = osip_from;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_cseq {
    pub method: *mut libc::c_char,
    pub number: *mut libc::c_char,
}
pub type osip_cseq_t = osip_cseq;
pub type osip_mime_version_t = osip_content_length_t;
pub type osip_to_t = osip_from_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_message {
    pub sip_version: *mut libc::c_char,
    pub req_uri: *mut osip_uri_t,
    pub sip_method: *mut libc::c_char,
    pub status_code: libc::c_int,
    pub reason_phrase: *mut libc::c_char,
    pub accepts: osip_list_t,
    pub accept_encodings: osip_list_t,
    pub accept_languages: osip_list_t,
    pub alert_infos: osip_list_t,
    pub allows: osip_list_t,
    pub authentication_infos: osip_list_t,
    pub authorizations: osip_list_t,
    pub call_id: *mut osip_call_id_t,
    pub call_infos: osip_list_t,
    pub contacts: osip_list_t,
    pub content_encodings: osip_list_t,
    pub content_length: *mut osip_content_length_t,
    pub content_type: *mut osip_content_type_t,
    pub cseq: *mut osip_cseq_t,
    pub error_infos: osip_list_t,
    pub from: *mut osip_from_t,
    pub mime_version: *mut osip_mime_version_t,
    pub proxy_authenticates: osip_list_t,
    pub proxy_authentication_infos: osip_list_t,
    pub proxy_authorizations: osip_list_t,
    pub record_routes: osip_list_t,
    pub routes: osip_list_t,
    pub to: *mut osip_to_t,
    pub vias: osip_list_t,
    pub www_authenticates: osip_list_t,
    pub headers: osip_list_t,
    pub bodies: osip_list_t,
    pub message_property: libc::c_int,
    pub message: *mut libc::c_char,
    pub message_length: size_t,
    pub application_data: *mut libc::c_void,
}
pub type osip_message_t = osip_message;
pub type __osip_message_config_t = ___osip_message_config_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ___osip_message_config_t {
    pub hname: *mut libc::c_char,
    pub setheader: Option::<
        unsafe extern "C" fn(*mut osip_message_t, *const libc::c_char) -> libc::c_int,
    >,
    pub ignored_when_invalid: libc::c_int,
}
pub type __osip_message_config_commaseparated_t = ___osip_message_config_commaseparated_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ___osip_message_config_commaseparated_t {
    pub hname: [libc::c_char; 256],
}
static mut pconfig: [__osip_message_config_t; 33] = [__osip_message_config_t {
    hname: 0 as *const libc::c_char as *mut libc::c_char,
    setheader: None,
    ignored_when_invalid: 0,
}; 33];
static mut pconfig_commasep: [__osip_message_config_commaseparated_t; 256] = [__osip_message_config_commaseparated_t {
    hname: [0; 256],
}; 256];
static mut hdr_ref_table: [libc::c_int; 150] = [0; 150];
pub unsafe extern "C" fn parser_init() -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut hname_length: libc::c_int = ::std::mem::size_of::<[libc::c_char; 256]>()
        as libc::c_ulong as libc::c_int;
    memset(
        pconfig_commasep.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[__osip_message_config_commaseparated_t; 256]>()
            as libc::c_ulong,
    );
    let fresh0 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh0 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Accept\0" as *const u8 as *const libc::c_char,
    );
    let fresh1 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh1 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    let fresh2 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh2 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
    );
    let fresh3 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh3 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Accept-Language\0" as *const u8 as *const libc::c_char,
    );
    let fresh4 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh4 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Alert-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh5 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh5 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Allow\0" as *const u8 as *const libc::c_char,
    );
    let fresh6 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh6 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Authentication-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh7 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh7 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Call-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh8 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh8 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Contact\0" as *const u8 as *const libc::c_char,
    );
    let fresh9 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh9 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"m\0" as *const u8 as *const libc::c_char,
    );
    let fresh10 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh10 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Content-Encoding\0" as *const u8 as *const libc::c_char,
    );
    let fresh11 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh11 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"e\0" as *const u8 as *const libc::c_char,
    );
    let fresh12 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh12 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Content-Language\0" as *const u8 as *const libc::c_char,
    );
    let fresh13 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh13 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Error-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh14 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh14 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"In-Reply-To\0" as *const u8 as *const libc::c_char,
    );
    let fresh15 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh15 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Proxy-Require\0" as *const u8 as *const libc::c_char,
    );
    let fresh16 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh16 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Record-Route\0" as *const u8 as *const libc::c_char,
    );
    let fresh17 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh17 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Require\0" as *const u8 as *const libc::c_char,
    );
    let fresh18 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh18 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Route\0" as *const u8 as *const libc::c_char,
    );
    let fresh19 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh19 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Supported\0" as *const u8 as *const libc::c_char,
    );
    let fresh20 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh20 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"k\0" as *const u8 as *const libc::c_char,
    );
    let fresh21 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh21 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Unsupported\0" as *const u8 as *const libc::c_char,
    );
    let fresh22 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh22 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Via\0" as *const u8 as *const libc::c_char,
    );
    let fresh23 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh23 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"v\0" as *const u8 as *const libc::c_char,
    );
    let fresh24 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh24 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Warning\0" as *const u8 as *const libc::c_char,
    );
    let fresh25 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh25 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Media-Authorization\0" as *const u8 as *const libc::c_char,
    );
    let fresh26 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh26 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Asserted-Identity\0" as *const u8 as *const libc::c_char,
    );
    let fresh27 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh27 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Preferred-Identity\0" as *const u8 as *const libc::c_char,
    );
    let fresh28 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh28 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Reason\0" as *const u8 as *const libc::c_char,
    );
    let fresh29 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh29 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Path\0" as *const u8 as *const libc::c_char,
    );
    let fresh30 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh30 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Security-Client\0" as *const u8 as *const libc::c_char,
    );
    let fresh31 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh31 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Security-Server\0" as *const u8 as *const libc::c_char,
    );
    let fresh32 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh32 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Security-Verify\0" as *const u8 as *const libc::c_char,
    );
    let fresh33 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh33 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Service-Route\0" as *const u8 as *const libc::c_char,
    );
    let fresh34 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh34 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Request-Disposition\0" as *const u8 as *const libc::c_char,
    );
    let fresh35 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh35 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
    );
    let fresh36 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh36 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Accept-Contact\0" as *const u8 as *const libc::c_char,
    );
    let fresh37 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh37 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    let fresh38 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh38 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Reject-Contact\0" as *const u8 as *const libc::c_char,
    );
    let fresh39 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh39 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"j\0" as *const u8 as *const libc::c_char,
    );
    let fresh40 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh40 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Resource-Priority\0" as *const u8 as *const libc::c_char,
    );
    let fresh41 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh41 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Accept-Resource-Priority\0" as *const u8 as *const libc::c_char,
    );
    let fresh42 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh42 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Early-Media\0" as *const u8 as *const libc::c_char,
    );
    let fresh43 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh43 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Refused-URI-List\0" as *const u8 as *const libc::c_char,
    );
    let fresh44 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh44 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Permission-Missing\0" as *const u8 as *const libc::c_char,
    );
    let fresh45 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh45 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Trigger-Consent\0" as *const u8 as *const libc::c_char,
    );
    let fresh46 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh46 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Asserted-Service\0" as *const u8 as *const libc::c_char,
    );
    let fresh47 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh47 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Preferred-Service\0" as *const u8 as *const libc::c_char,
    );
    let fresh48 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh48 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Recv-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh49 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh49 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Allow-Events\0" as *const u8 as *const libc::c_char,
    );
    let fresh50 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh50 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"u\0" as *const u8 as *const libc::c_char,
    );
    let fresh51 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh51 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Policy-ID\0" as *const u8 as *const libc::c_char,
    );
    let fresh52 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh52 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Policy-Contact\0" as *const u8 as *const libc::c_char,
    );
    let fresh53 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh53 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Feature-Caps\0" as *const u8 as *const libc::c_char,
    );
    let fresh54 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh54 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"History-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh55 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh55 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Accept\0" as *const u8 as *const libc::c_char,
    );
    let fresh56 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh56 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Associated-URI\0" as *const u8 as *const libc::c_char,
    );
    let fresh57 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh57 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Visited-Network-ID\0" as *const u8 as *const libc::c_char,
    );
    let fresh58 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh58 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Access-Network-Info\0" as *const u8 as *const libc::c_char,
    );
    let fresh59 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh59 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"P-Charging-Function-Addresses\0" as *const u8 as *const libc::c_char,
    );
    let fresh60 = i;
    i = i + 1;
    snprintf(
        (pconfig_commasep[fresh60 as usize].hname).as_mut_ptr(),
        hname_length as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"User-to-User\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    pconfig[i as usize]
        .hname = b"accept\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh61 = i;
    i = i + 1;
    pconfig[fresh61 as usize]
        .setheader = Some(
        osip_message_set_accept
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"accept-encoding\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh62 = i;
    i = i + 1;
    pconfig[fresh62 as usize]
        .setheader = Some(
        osip_message_set_accept_encoding
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"accept-language\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh63 = i;
    i = i + 1;
    pconfig[fresh63 as usize]
        .setheader = Some(
        osip_message_set_accept_language
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"alert-info\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh64 = i;
    i = i + 1;
    pconfig[fresh64 as usize]
        .setheader = Some(
        osip_message_set_alert_info
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"allow\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh65 = i;
    i = i + 1;
    pconfig[fresh65 as usize]
        .setheader = Some(
        osip_message_set_allow
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"authentication-info\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh66 = i;
    i = i + 1;
    pconfig[fresh66 as usize]
        .setheader = Some(
        osip_message_set_authentication_info
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"authorization\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh67 = i;
    i = i + 1;
    pconfig[fresh67 as usize]
        .setheader = Some(
        osip_message_set_authorization
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh68 = i;
    i = i + 1;
    pconfig[fresh68 as usize]
        .setheader = Some(
        osip_message_set_content_type
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"call-id\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh69 = i;
    i = i + 1;
    pconfig[fresh69 as usize]
        .setheader = Some(
        osip_message_set_call_id
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"call-info\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh70 = i;
    i = i + 1;
    pconfig[fresh70 as usize]
        .setheader = Some(
        osip_message_set_call_info
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"contact\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh71 = i;
    i = i + 1;
    pconfig[fresh71 as usize]
        .setheader = Some(
        osip_message_set_contact
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"content-encoding\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh72 = i;
    i = i + 1;
    pconfig[fresh72 as usize]
        .setheader = Some(
        osip_message_set_content_encoding
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"content-length\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh73 = i;
    i = i + 1;
    pconfig[fresh73 as usize]
        .setheader = Some(
        osip_message_set_content_length
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"content-type\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh74 = i;
    i = i + 1;
    pconfig[fresh74 as usize]
        .setheader = Some(
        osip_message_set_content_type
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"cseq\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh75 = i;
    i = i + 1;
    pconfig[fresh75 as usize]
        .setheader = Some(
        osip_message_set_cseq
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh76 = i;
    i = i + 1;
    pconfig[fresh76 as usize]
        .setheader = Some(
        osip_message_set_content_encoding
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"error-info\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh77 = i;
    i = i + 1;
    pconfig[fresh77 as usize]
        .setheader = Some(
        osip_message_set_error_info
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh78 = i;
    i = i + 1;
    pconfig[fresh78 as usize]
        .setheader = Some(
        osip_message_set_from
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"from\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh79 = i;
    i = i + 1;
    pconfig[fresh79 as usize]
        .setheader = Some(
        osip_message_set_from
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh80 = i;
    i = i + 1;
    pconfig[fresh80 as usize]
        .setheader = Some(
        osip_message_set_call_id
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh81 = i;
    i = i + 1;
    pconfig[fresh81 as usize]
        .setheader = Some(
        osip_message_set_content_length
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh82 = i;
    i = i + 1;
    pconfig[fresh82 as usize]
        .setheader = Some(
        osip_message_set_contact
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"mime-version\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh83 = i;
    i = i + 1;
    pconfig[fresh83 as usize]
        .setheader = Some(
        osip_message_set_mime_version
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"proxy-authenticate\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh84 = i;
    i = i + 1;
    pconfig[fresh84 as usize]
        .setheader = Some(
        osip_message_set_proxy_authenticate
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"proxy-authentication-info\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh85 = i;
    i = i + 1;
    pconfig[fresh85 as usize]
        .setheader = Some(
        osip_message_set_proxy_authentication_info
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"proxy-authorization\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh86 = i;
    i = i + 1;
    pconfig[fresh86 as usize]
        .setheader = Some(
        osip_message_set_proxy_authorization
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"record-route\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh87 = i;
    i = i + 1;
    pconfig[fresh87 as usize]
        .setheader = Some(
        osip_message_set_record_route
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"route\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh88 = i;
    i = i + 1;
    pconfig[fresh88 as usize]
        .setheader = Some(
        osip_message_set_route
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh89 = i;
    i = i + 1;
    pconfig[fresh89 as usize]
        .setheader = Some(
        osip_message_set_to
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"to\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh90 = i;
    i = i + 1;
    pconfig[fresh90 as usize]
        .setheader = Some(
        osip_message_set_to
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"v\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh91 = i;
    i = i + 1;
    pconfig[fresh91 as usize]
        .setheader = Some(
        osip_message_set_via
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"via\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 0 as libc::c_int;
    let fresh92 = i;
    i = i + 1;
    pconfig[fresh92 as usize]
        .setheader = Some(
        osip_message_set_via
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    pconfig[i as usize]
        .hname = b"www-authenticate\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pconfig[i as usize].ignored_when_invalid = 1 as libc::c_int;
    let fresh93 = i;
    i = i + 1;
    pconfig[fresh93 as usize]
        .setheader = Some(
        osip_message_set_www_authenticate
            as unsafe extern "C" fn(
                *mut osip_message_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 150 as libc::c_int {
        hdr_ref_table[i as usize] = -(1 as libc::c_int);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 33 as libc::c_int {
        let mut hash: libc::c_ulong = 0;
        hash = osip_hash(pconfig[i as usize].hname);
        hash = hash.wrapping_rem(150 as libc::c_int as libc::c_ulong);
        if hdr_ref_table[hash as usize] == -(1 as libc::c_int) {
            hdr_ref_table[hash as usize] = i;
        } else {
            osip_trace(
                b"osip_parser_cfg.c\0" as *const u8 as *const libc::c_char,
                299 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"conflict with current hashtable size\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn parser_add_comma_separated_header(
    mut hname: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if pconfig_commasep[i as usize].hname[0 as libc::c_int as usize] as libc::c_int
            == '\0' as i32
        {
            snprintf(
                (pconfig_commasep[i as usize].hname).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                hname,
            );
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn __osip_message_is_header_comma_separated(
    mut hname: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if pconfig_commasep[i as usize].hname[0 as libc::c_int as usize] as libc::c_int
            == '\0' as i32
        {
            break;
        }
        if osip_strcasecmp((pconfig_commasep[i as usize].hname).as_mut_ptr(), hname)
            == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn __osip_message_is_known_header(
    mut hname: *const libc::c_char,
) -> libc::c_int {
    let mut hash: libc::c_ulong = 0;
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut index: libc::c_int = 0;
    hash = osip_hash(hname);
    hash = hash.wrapping_rem(150 as libc::c_int as libc::c_ulong);
    index = hdr_ref_table[hash as usize];
    if index != -(1 as libc::c_int)
        && 0 as libc::c_int == strcmp(pconfig[index as usize].hname, hname)
    {
        result = index;
    }
    return result;
}
pub unsafe extern "C" fn __osip_message_call_method(
    mut i: libc::c_int,
    mut dest: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = (pconfig[i as usize].setheader).unwrap()(dest, hvalue);
    if err < 0 as libc::c_int {
        osip_trace(
            b"osip_parser_cfg.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Could not set header: %s: %s\n\0" as *const u8 as *const libc::c_char,
            pconfig[i as usize].hname,
            hvalue,
        );
    }
    if pconfig[i as usize].ignored_when_invalid == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    return err;
}
