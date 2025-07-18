use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_str_append(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn osip_strn_append(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_authorization_to_str(
        header: *const osip_authorization_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_header_to_str(
        header: *const osip_header_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_content_type_to_str(
        header: *const osip_content_type_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_accept_to_str(
        header: *const osip_accept_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_accept_encoding_to_str(
        header: *const osip_accept_encoding_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_call_info_to_str(
        header: *const osip_call_info_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_content_length_to_str(
        header: *const osip_content_length_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_authentication_info_to_str(
        header: *const osip_authentication_info_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_uri_to_str(
        url: *const osip_uri_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_call_id_to_str(
        header: *const osip_call_id_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_from_to_str(
        header: *const osip_from_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_contact_to_str(
        header: *const osip_contact_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_cseq_to_str(
        header: *const osip_cseq_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_www_authenticate_to_str(
        header: *const osip_www_authenticate_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_record_route_to_str(
        header: *const osip_record_route_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_route_to_str(
        header: *const osip_route_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_to_to_str(
        header: *const osip_to_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_via_to_str(
        header: *const osip_via_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_body_to_str(
        body: *const osip_body_t,
        dest: *mut *mut libc::c_char,
        length: *mut size_t,
    ) -> libc::c_int;
    static mut osip_protocol_version: *const libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list_iterator {
    pub actual: *mut __node_t,
    pub prev: *mut *mut __node_t,
    pub li: *mut osip_list_t,
    pub pos: libc::c_int,
}
pub type osip_list_iterator_t = osip_list_iterator;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type osip_realloc_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
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
pub struct osip_uri_param {
    pub gname: *mut libc::c_char,
    pub gvalue: *mut libc::c_char,
}
pub type osip_uri_param_t = osip_uri_param;
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
pub struct osip_header {
    pub hname: *mut libc::c_char,
    pub hvalue: *mut libc::c_char,
}
pub type osip_header_t = osip_header;
pub type osip_generic_param_t = osip_uri_param_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_type {
    pub type_0: *mut libc::c_char,
    pub subtype: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_content_type_t = osip_content_type;
pub type osip_accept_t = osip_content_type_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_accept_encoding {
    pub element: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_accept_encoding_t = osip_accept_encoding;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_call_info {
    pub element: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_call_info_t = osip_call_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_length {
    pub value: *mut libc::c_char,
}
pub type osip_content_length_t = osip_content_length;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_authentication_info {
    pub auth_type: *mut libc::c_char,
    pub nextnonce: *mut libc::c_char,
    pub qop_options: *mut libc::c_char,
    pub rspauth: *mut libc::c_char,
    pub cnonce: *mut libc::c_char,
    pub nonce_count: *mut libc::c_char,
    pub snum: *mut libc::c_char,
    pub srand: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
}
pub type osip_authentication_info_t = osip_authentication_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_authorization {
    pub auth_type: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub uri: *mut libc::c_char,
    pub response: *mut libc::c_char,
    pub digest: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub cnonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub message_qop: *mut libc::c_char,
    pub nonce_count: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub gssapi_data: *mut libc::c_char,
    pub crand: *mut libc::c_char,
    pub cnum: *mut libc::c_char,
    pub auth_param: *mut libc::c_char,
}
pub type osip_authorization_t = osip_authorization;
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
pub type osip_contact_t = osip_from_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_cseq {
    pub method: *mut libc::c_char,
    pub number: *mut libc::c_char,
}
pub type osip_cseq_t = osip_cseq;
pub type osip_mime_version_t = osip_content_length_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_www_authenticate {
    pub auth_type: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub stale: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub qop_options: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub gssapi_data: *mut libc::c_char,
    pub auth_param: *mut libc::c_char,
}
pub type osip_www_authenticate_t = osip_www_authenticate;
pub type osip_record_route_t = osip_from_t;
pub type osip_route_t = osip_from_t;
pub type osip_to_t = osip_from_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_via {
    pub version: *mut libc::c_char,
    pub protocol: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub via_params: osip_list_t,
}
pub type osip_via_t = osip_via;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_body {
    pub body: *mut libc::c_char,
    pub length: size_t,
    pub headers: *mut osip_list_t,
    pub content_type: *mut osip_content_type_t,
}
pub type osip_body_t = osip_body;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct to_str_table {
    pub header_name: [libc::c_char; 30],
    pub header_length: libc::c_int,
    pub header_list: *mut osip_list_t,
    pub header_data: *mut libc::c_void,
    pub to_str: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_char) -> libc::c_int,
    >,
}
unsafe extern "C" fn __osip_message_startline_to_strreq(
    mut sip: *mut osip_message_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sip_version: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rquri: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    *dest = 0 as *mut libc::c_char;
    if sip.is_null() || ((*sip).req_uri).is_null() || ((*sip).sip_method).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_uri_to_str((*sip).req_uri, &mut rquri);
    if i != 0 as libc::c_int {
        return i;
    }
    if ((*sip).sip_version).is_null() {
        sip_version = osip_protocol_version;
    } else {
        sip_version = (*sip).sip_version;
    }
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (strlen((*sip).sip_method))
                .wrapping_add(strlen(rquri))
                .wrapping_add(strlen(sip_version))
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        )
    } else {
        malloc(
            (strlen((*sip).sip_method))
                .wrapping_add(strlen(rquri))
                .wrapping_add(strlen(sip_version))
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        )
    }) as *mut libc::c_char;
    if (*dest).is_null() {
        if !rquri.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(rquri as *mut libc::c_void);
            } else {
                free(rquri as *mut libc::c_void);
            }
        }
        return -(4 as libc::c_int);
    }
    tmp = *dest;
    tmp = osip_str_append(tmp, (*sip).sip_method);
    *tmp = ' ' as i32 as libc::c_char;
    tmp = tmp.offset(1);
    tmp;
    tmp = osip_str_append(tmp, rquri);
    *tmp = ' ' as i32 as libc::c_char;
    tmp = tmp.offset(1);
    tmp;
    strcpy(tmp, sip_version);
    if !rquri.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(rquri as *mut libc::c_void);
        } else {
            free(rquri as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __osip_message_startline_to_strresp(
    mut sip: *mut osip_message_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sip_version: *const libc::c_char = 0 as *const libc::c_char;
    let mut status_code: [libc::c_char; 5] = [0; 5];
    *dest = 0 as *mut libc::c_char;
    if sip.is_null() || ((*sip).reason_phrase).is_null()
        || (*sip).status_code < 100 as libc::c_int
        || (*sip).status_code > 699 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    if ((*sip).sip_version).is_null() {
        sip_version = osip_protocol_version;
    } else {
        sip_version = (*sip).sip_version;
    }
    sprintf(
        status_code.as_mut_ptr(),
        b"%u\0" as *const u8 as *const libc::c_char,
        (*sip).status_code,
    );
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (strlen(sip_version))
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*sip).reason_phrase))
                .wrapping_add(4 as libc::c_int as libc::c_ulong),
        )
    } else {
        malloc(
            (strlen(sip_version))
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*sip).reason_phrase))
                .wrapping_add(4 as libc::c_int as libc::c_ulong),
        )
    }) as *mut libc::c_char;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    tmp = *dest;
    tmp = osip_str_append(tmp, sip_version);
    *tmp = ' ' as i32 as libc::c_char;
    tmp = tmp.offset(1);
    tmp;
    tmp = osip_strn_append(tmp, status_code.as_mut_ptr(), 3 as libc::c_int as size_t);
    *tmp = ' ' as i32 as libc::c_char;
    tmp = tmp.offset(1);
    tmp;
    strcpy(tmp, (*sip).reason_phrase);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __osip_message_startline_to_str(
    mut sip: *mut osip_message_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    if !((*sip).sip_method).is_null() {
        return __osip_message_startline_to_strreq(sip, dest);
    }
    if (*sip).status_code != 0 as libc::c_int {
        return __osip_message_startline_to_strresp(sip, dest);
    }
    osip_trace(
        b"osip_message_to_str.c\0" as *const u8 as *const libc::c_char,
        119 as libc::c_int,
        TRACE_LEVEL1,
        0 as *mut FILE,
        b"ERROR method has no value or status code is 0!\n\0" as *const u8
            as *const libc::c_char,
    );
    return -(2 as libc::c_int);
}
pub unsafe extern "C" fn osip_message_get_reason_phrase(
    mut sip: *const osip_message_t,
) -> *mut libc::c_char {
    return (*sip).reason_phrase;
}
pub unsafe extern "C" fn osip_message_get_status_code(
    mut sip: *const osip_message_t,
) -> libc::c_int {
    return (*sip).status_code;
}
pub unsafe extern "C" fn osip_message_get_method(
    mut sip: *const osip_message_t,
) -> *mut libc::c_char {
    return (*sip).sip_method;
}
pub unsafe extern "C" fn osip_message_get_version(
    mut sip: *const osip_message_t,
) -> *mut libc::c_char {
    return (*sip).sip_version;
}
pub unsafe extern "C" fn osip_message_get_uri(
    mut sip: *const osip_message_t,
) -> *mut osip_uri_t {
    return (*sip).req_uri;
}
unsafe extern "C" fn strcat_simple_header(
    mut _string: *mut *mut libc::c_char,
    mut malloc_size: *mut size_t,
    mut _message: *mut *mut libc::c_char,
    mut ptr_header: *mut libc::c_void,
    mut header_name: *mut libc::c_char,
    mut size_of_header: size_t,
    mut xxx_to_str: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_char) -> libc::c_int,
    >,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    string = *_string;
    message = *_message;
    if !ptr_header.is_null() {
        if *malloc_size
            < ((message.offset_from(string) as libc::c_long
                + 100 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_add(size_of_header)
        {
            let mut size: size_t = message.offset_from(string) as libc::c_long as size_t;
            *malloc_size = (message.offset_from(string) as libc::c_long as libc::c_ulong)
                .wrapping_add(size_of_header)
                .wrapping_add(100 as libc::c_int as libc::c_ulong);
            string = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(string as *mut libc::c_void, *malloc_size)
            } else {
                realloc(string as *mut libc::c_void, *malloc_size)
            }) as *mut libc::c_char;
            if string.is_null() {
                if !(*_string).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*_string as *mut libc::c_void);
                    } else {
                        free(*_string as *mut libc::c_void);
                    }
                }
                *_string = 0 as *mut libc::c_char;
                *_message = 0 as *mut libc::c_char;
                return -(4 as libc::c_int);
            }
            *_string = string;
            message = string.offset(size as isize);
        }
        message = osip_strn_append(message, header_name, size_of_header);
        i = xxx_to_str.unwrap()(ptr_header, &mut tmp);
        if i != 0 as libc::c_int {
            *_string = string;
            *_message = message;
            *next = 0 as *mut libc::c_char;
            return i;
        }
        if *malloc_size
            < (message.offset_from(string) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(100 as libc::c_int as libc::c_ulong)
        {
            let mut size_0: size_t = message.offset_from(string) as libc::c_long
                as size_t;
            *malloc_size = (message.offset_from(string) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(100 as libc::c_int as libc::c_ulong);
            string = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(string as *mut libc::c_void, *malloc_size)
            } else {
                realloc(string as *mut libc::c_void, *malloc_size)
            }) as *mut libc::c_char;
            if string.is_null() {
                if !(*_string).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*_string as *mut libc::c_void);
                    } else {
                        free(*_string as *mut libc::c_void);
                    }
                }
                *_string = 0 as *mut libc::c_char;
                *_message = 0 as *mut libc::c_char;
                return -(4 as libc::c_int);
            }
            *_string = string;
            message = string.offset(size_0 as isize);
        }
        message = osip_str_append(message, tmp);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        message = osip_strn_append(
            message,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
    }
    *_string = string;
    *_message = message;
    *next = message;
    return 0 as libc::c_int;
}
unsafe extern "C" fn strcat_headers_one_per_line(
    mut _string: *mut *mut libc::c_char,
    mut malloc_size: *mut size_t,
    mut _message: *mut *mut libc::c_char,
    mut headers: *mut osip_list_t,
    mut header: *mut libc::c_char,
    mut size_of_header: size_t,
    mut xxx_to_str: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_char) -> libc::c_int,
    >,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut elt: *mut libc::c_void = osip_list_get_first(headers, &mut it);
    string = *_string;
    message = *_message;
    while !elt.is_null() {
        if *malloc_size
            < ((message.offset_from(string) as libc::c_long
                + 100 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_add(size_of_header)
        {
            let mut size: size_t = message.offset_from(string) as libc::c_long as size_t;
            *malloc_size = (message.offset_from(string) as libc::c_long as libc::c_ulong)
                .wrapping_add(size_of_header)
                .wrapping_add(100 as libc::c_int as libc::c_ulong);
            string = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(string as *mut libc::c_void, *malloc_size)
            } else {
                realloc(string as *mut libc::c_void, *malloc_size)
            }) as *mut libc::c_char;
            if string.is_null() {
                if !(*_string).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*_string as *mut libc::c_void);
                    } else {
                        free(*_string as *mut libc::c_void);
                    }
                }
                *_string = 0 as *mut libc::c_char;
                *_message = 0 as *mut libc::c_char;
                return -(4 as libc::c_int);
            }
            *_string = string;
            message = string.offset(size as isize);
        }
        osip_strncpy(message, header, size_of_header);
        i = xxx_to_str.unwrap()(elt, &mut tmp);
        if i != 0 as libc::c_int {
            *_string = string;
            *_message = message;
            *next = 0 as *mut libc::c_char;
            return i;
        }
        message = message.offset(strlen(message) as isize);
        if *malloc_size
            < (message.offset_from(string) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(100 as libc::c_int as libc::c_ulong)
        {
            let mut size_0: size_t = message.offset_from(string) as libc::c_long
                as size_t;
            *malloc_size = (message.offset_from(string) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(100 as libc::c_int as libc::c_ulong);
            string = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(string as *mut libc::c_void, *malloc_size)
            } else {
                realloc(string as *mut libc::c_void, *malloc_size)
            }) as *mut libc::c_char;
            if string.is_null() {
                if !(*_string).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*_string as *mut libc::c_void);
                    } else {
                        free(*_string as *mut libc::c_void);
                    }
                }
                *_string = 0 as *mut libc::c_char;
                *_message = 0 as *mut libc::c_char;
                return -(4 as libc::c_int);
            }
            *_string = string;
            message = string.offset(size_0 as isize);
        }
        message = osip_str_append(message, tmp);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        message = osip_strn_append(
            message,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        elt = osip_list_get_next(&mut it);
    }
    *_string = string;
    *_message = message;
    *next = message;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get__property(
    mut sip: *const osip_message_t,
) -> libc::c_int {
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    return (*sip).message_property;
}
pub unsafe extern "C" fn osip_message_force_update(
    mut sip: *mut osip_message_t,
) -> libc::c_int {
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    (*sip).message_property = 2 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _osip_message_realloc(
    mut message: *mut *mut libc::c_char,
    mut dest: *mut *mut libc::c_char,
    mut needed: size_t,
    mut malloc_size: *mut size_t,
) -> libc::c_int {
    let mut size: size_t = (*message).offset_from(*dest) as libc::c_long as size_t;
    if *malloc_size
        < size.wrapping_add(needed).wrapping_add(100 as libc::c_int as libc::c_ulong)
    {
        *malloc_size = size
            .wrapping_add(needed)
            .wrapping_add(100 as libc::c_int as libc::c_ulong);
        *dest = (if osip_realloc_func.is_some() {
            osip_realloc_func.unwrap()(*dest as *mut libc::c_void, *malloc_size)
        } else {
            realloc(*dest as *mut libc::c_void, *malloc_size)
        }) as *mut libc::c_char;
        if (*dest).is_null() {
            return -(4 as libc::c_int);
        }
        *message = (*dest).offset(size as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _osip_message_to_str(
    mut sip: *mut osip_message_t,
    mut dest: *mut *mut libc::c_char,
    mut message_length: *mut size_t,
    mut sipfrag: libc::c_int,
) -> libc::c_int {
    let mut malloc_size: size_t = 0;
    let mut total_length: size_t = 0 as libc::c_int as size_t;
    let mut start_of_bodies: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut content_length_to_modify: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut boundary: *mut libc::c_char = 0 as *mut libc::c_char;
    malloc_size = 8000 as libc::c_int as size_t;
    *dest = 0 as *mut libc::c_char;
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    if 1 as libc::c_int == osip_message_get__property(sip) {
        *dest = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                ((*sip).message_length).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc(
                ((*sip).message_length).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        }) as *mut libc::c_char;
        if (*dest).is_null() {
            return -(4 as libc::c_int);
        }
        memcpy(
            *dest as *mut libc::c_void,
            (*sip).message as *const libc::c_void,
            (*sip).message_length,
        );
        *(*dest).offset((*sip).message_length as isize) = '\0' as i32 as libc::c_char;
        if !message_length.is_null() {
            *message_length = (*sip).message_length;
        }
        return 0 as libc::c_int;
    } else {
        if !((*sip).message).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*sip).message as *mut libc::c_void);
            } else {
                free((*sip).message as *mut libc::c_void);
            }
        }
        (*sip).message = 0 as *mut libc::c_char;
    }
    message = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(8000 as libc::c_int as size_t)
    } else {
        malloc(8000 as libc::c_int as libc::c_ulong)
    }) as *mut libc::c_char;
    if message.is_null() {
        return -(4 as libc::c_int);
    }
    *dest = message;
    i = __osip_message_startline_to_str(sip, &mut tmp);
    if i != 0 as libc::c_int {
        if sipfrag == 0 {
            if !(*dest).is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(*dest as *mut libc::c_void);
                } else {
                    free(*dest as *mut libc::c_void);
                }
            }
            *dest = 0 as *mut libc::c_char;
            return i;
        }
    } else {
        let mut message_len: size_t = strlen(tmp);
        if _osip_message_realloc(
            &mut message,
            dest,
            message_len.wrapping_add(3 as libc::c_int as libc::c_ulong),
            &mut malloc_size,
        ) < 0 as libc::c_int
        {
            if !tmp.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp as *mut libc::c_void);
                } else {
                    free(tmp as *mut libc::c_void);
                }
            }
            *dest = 0 as *mut libc::c_char;
            return -(4 as libc::c_int);
        }
        message = osip_str_append(message, tmp);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        message = osip_strn_append(
            message,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
    }
    let mut table: [to_str_table; 25] = [
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Via: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 5 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_via_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_via_to_str
                            as unsafe extern "C" fn(
                                *const osip_via_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Record-Route: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 14 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_record_route_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_record_route_to_str
                            as unsafe extern "C" fn(
                                *const osip_record_route_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Route: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 7 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_route_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_route_to_str
                            as unsafe extern "C" fn(
                                *const osip_route_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"From: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 6 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_from_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_from_to_str
                            as unsafe extern "C" fn(
                                *const osip_from_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"To: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 4 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_to_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_to_to_str
                            as unsafe extern "C" fn(
                                *const osip_to_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Call-ID: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 9 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_call_id_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_call_id_to_str
                            as unsafe extern "C" fn(
                                *const osip_call_id_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"CSeq: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 6 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_cseq_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_cseq_to_str
                            as unsafe extern "C" fn(
                                *const osip_cseq_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Contact: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 9 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_contact_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_contact_to_str
                            as unsafe extern "C" fn(
                                *const osip_contact_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Authorization: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 15 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_authorization_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_authorization_to_str
                            as unsafe extern "C" fn(
                                *const osip_authorization_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"WWW-Authenticate: \0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 18 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_www_authenticate_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_www_authenticate_to_str
                            as unsafe extern "C" fn(
                                *const osip_www_authenticate_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Proxy-Authenticate: \0\0\0\0\0\0\0\0\0\0"),
                header_length: 20 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_www_authenticate_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_www_authenticate_to_str
                            as unsafe extern "C" fn(
                                *const osip_www_authenticate_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Proxy-Authorization: \0\0\0\0\0\0\0\0\0"),
                header_length: 21 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_authorization_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_authorization_to_str
                            as unsafe extern "C" fn(
                                *const osip_authorization_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Call-Info: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 11 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_call_info_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_call_info_to_str
                            as unsafe extern "C" fn(
                                *const osip_call_info_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Content-Type: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 14 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_content_type_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_content_type_to_str
                            as unsafe extern "C" fn(
                                *const osip_content_type_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Mime-Version: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 14 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_content_length_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_content_length_to_str
                            as unsafe extern "C" fn(
                                *const osip_content_length_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Allow: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 7 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_content_length_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_content_length_to_str
                            as unsafe extern "C" fn(
                                *const osip_content_length_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Content-Encoding: \0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 18 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_content_length_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_content_length_to_str
                            as unsafe extern "C" fn(
                                *const osip_content_length_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Alert-Info: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 12 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_call_info_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_call_info_to_str
                            as unsafe extern "C" fn(
                                *const osip_call_info_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Error-Info: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 12 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_call_info_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_call_info_to_str
                            as unsafe extern "C" fn(
                                *const osip_call_info_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Accept: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 8 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_accept_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_accept_to_str
                            as unsafe extern "C" fn(
                                *const osip_accept_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Accept-Encoding: \0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 17 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_accept_encoding_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_accept_encoding_to_str
                            as unsafe extern "C" fn(
                                *const osip_accept_encoding_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Accept-Language: \0\0\0\0\0\0\0\0\0\0\0\0\0"),
                header_length: 17 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_accept_encoding_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_accept_encoding_to_str
                            as unsafe extern "C" fn(
                                *const osip_accept_encoding_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Authentication-Info: \0\0\0\0\0\0\0\0\0"),
                header_length: 21 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_authentication_info_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_authentication_info_to_str
                            as unsafe extern "C" fn(
                                *const osip_authentication_info_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: *::std::mem::transmute::<
                    &[u8; 30],
                    &mut [libc::c_char; 30],
                >(b"Proxy-Authentication-Info: \0\0\0"),
                header_length: 27 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const osip_authentication_info_t,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        osip_authentication_info_to_str
                            as unsafe extern "C" fn(
                                *const osip_authentication_info_t,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            };
            init
        },
        {
            let mut init = to_str_table {
                header_name: [
                    '\0' as i32 as libc::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                header_length: 0 as libc::c_int,
                header_list: 0 as *mut osip_list_t,
                header_data: 0 as *mut libc::c_void,
                to_str: None,
            };
            init
        },
    ];
    table[0 as libc::c_int as usize].header_list = &mut (*sip).vias;
    table[1 as libc::c_int as usize].header_list = &mut (*sip).record_routes;
    table[2 as libc::c_int as usize].header_list = &mut (*sip).routes;
    table[3 as libc::c_int as usize].header_data = (*sip).from as *mut libc::c_void;
    table[4 as libc::c_int as usize].header_data = (*sip).to as *mut libc::c_void;
    table[5 as libc::c_int as usize].header_data = (*sip).call_id as *mut libc::c_void;
    table[6 as libc::c_int as usize].header_data = (*sip).cseq as *mut libc::c_void;
    table[7 as libc::c_int as usize].header_list = &mut (*sip).contacts;
    table[8 as libc::c_int as usize].header_list = &mut (*sip).authorizations;
    table[9 as libc::c_int as usize].header_list = &mut (*sip).www_authenticates;
    table[10 as libc::c_int as usize].header_list = &mut (*sip).proxy_authenticates;
    table[11 as libc::c_int as usize].header_list = &mut (*sip).proxy_authorizations;
    table[12 as libc::c_int as usize].header_list = &mut (*sip).call_infos;
    table[13 as libc::c_int as usize]
        .header_data = (*sip).content_type as *mut libc::c_void;
    table[14 as libc::c_int as usize]
        .header_data = (*sip).mime_version as *mut libc::c_void;
    table[15 as libc::c_int as usize].header_list = &mut (*sip).allows;
    table[16 as libc::c_int as usize].header_list = &mut (*sip).content_encodings;
    table[17 as libc::c_int as usize].header_list = &mut (*sip).alert_infos;
    table[18 as libc::c_int as usize].header_list = &mut (*sip).error_infos;
    table[19 as libc::c_int as usize].header_list = &mut (*sip).accepts;
    table[20 as libc::c_int as usize].header_list = &mut (*sip).accept_encodings;
    table[21 as libc::c_int as usize].header_list = &mut (*sip).accept_languages;
    table[22 as libc::c_int as usize].header_list = &mut (*sip).authentication_infos;
    table[23 as libc::c_int as usize]
        .header_list = &mut (*sip).proxy_authentication_infos;
    pos = 0 as libc::c_int;
    while table[pos as usize].header_name[0 as libc::c_int as usize] as libc::c_int
        != '\0' as i32
    {
        if (table[13 as libc::c_int as usize].header_list).is_null() {
            i = strcat_simple_header(
                dest,
                &mut malloc_size,
                &mut message,
                table[pos as usize].header_data,
                (table[pos as usize].header_name).as_mut_ptr(),
                table[pos as usize].header_length as size_t,
                table[pos as usize].to_str,
                &mut next,
            );
        }
        i = strcat_headers_one_per_line(
            dest,
            &mut malloc_size,
            &mut message,
            table[pos as usize].header_list,
            (table[pos as usize].header_name).as_mut_ptr(),
            table[pos as usize].header_length as size_t,
            table[pos as usize].to_str,
            &mut next,
        );
        if i != 0 as libc::c_int {
            if !(*dest).is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(*dest as *mut libc::c_void);
                } else {
                    free(*dest as *mut libc::c_void);
                }
            }
            *dest = 0 as *mut libc::c_char;
            return i;
        }
        message = next;
        pos += 1;
        pos;
    }
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut header: *mut osip_header_t = osip_list_get_first(
        &mut (*sip).headers,
        &mut it,
    ) as *mut osip_header_t;
    while !header.is_null() {
        let mut header_len: size_t = 0 as libc::c_int as size_t;
        i = osip_header_to_str(header, &mut tmp);
        if i != 0 as libc::c_int {
            if !(*dest).is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(*dest as *mut libc::c_void);
                } else {
                    free(*dest as *mut libc::c_void);
                }
            }
            *dest = 0 as *mut libc::c_char;
            return i;
        }
        header_len = strlen(tmp);
        if _osip_message_realloc(
            &mut message,
            dest,
            header_len.wrapping_add(3 as libc::c_int as libc::c_ulong),
            &mut malloc_size,
        ) < 0 as libc::c_int
        {
            if !tmp.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp as *mut libc::c_void);
                } else {
                    free(tmp as *mut libc::c_void);
                }
            }
            *dest = 0 as *mut libc::c_char;
            return -(4 as libc::c_int);
        }
        message = osip_str_append(message, tmp);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        message = osip_strn_append(
            message,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        header = osip_list_get_next(&mut it) as *mut osip_header_t;
    }
    if _osip_message_realloc(
        &mut message,
        dest,
        16 as libc::c_int as size_t,
        &mut malloc_size,
    ) < 0 as libc::c_int
    {
        return -(4 as libc::c_int);
    }
    if sipfrag != 0 && osip_list_eol(&mut (*sip).bodies, 0 as libc::c_int) != 0 {
        osip_strncpy(
            message,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        message = message.offset(2 as libc::c_int as isize);
        (*sip).message_property = 1 as libc::c_int;
        (*sip).message = osip_strdup(*dest);
        (*sip).message_length = message.offset_from(*dest) as libc::c_long as size_t;
        if !message_length.is_null() {
            *message_length = message.offset_from(*dest) as libc::c_long as size_t;
        }
        return 0 as libc::c_int;
    }
    osip_strncpy(
        message,
        b"Content-Length: \0" as *const u8 as *const libc::c_char,
        16 as libc::c_int as size_t,
    );
    message = message.offset(16 as libc::c_int as isize);
    if osip_list_eol(&mut (*sip).bodies, 0 as libc::c_int) != 0 {
        message = osip_strn_append(
            message,
            b"0\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
    } else {
        content_length_to_modify = message;
        message = osip_str_append(
            message,
            b"     \0" as *const u8 as *const libc::c_char,
        );
    }
    message = osip_strn_append(
        message,
        b"\r\n\0\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    message = osip_strn_append(
        message,
        b"\r\n\0\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    start_of_bodies = message;
    total_length = start_of_bodies.offset_from(*dest) as libc::c_long as size_t;
    if osip_list_eol(&mut (*sip).bodies, 0 as libc::c_int) != 0 {
        (*sip).message_property = 1 as libc::c_int;
        (*sip).message = osip_strdup(*dest);
        (*sip).message_length = total_length;
        if !message_length.is_null() {
            *message_length = total_length;
        }
        return 0 as libc::c_int;
    }
    if !((*sip).content_type).is_null() && !((*(*sip).content_type).type_0).is_null()
        && osip_strcasecmp(
            (*(*sip).content_type).type_0,
            b"multipart\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut ct_param: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
        i = osip_uri_param_get_byname(
            &mut (*(*sip).content_type).gen_params,
            b"boundary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut ct_param,
        );
        if i >= 0 as libc::c_int && !ct_param.is_null()
            && !((*ct_param).gvalue).is_null()
        {
            let mut len: size_t = strlen((*ct_param).gvalue);
            if len > 70 as libc::c_int as libc::c_ulong {
                if !(*dest).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*dest as *mut libc::c_void);
                    } else {
                        free(*dest as *mut libc::c_void);
                    }
                }
                *dest = 0 as *mut libc::c_char;
                return -(5 as libc::c_int);
            }
            if len == 1 as libc::c_int as libc::c_ulong
                && *((*ct_param).gvalue).offset(0 as libc::c_int as isize) as libc::c_int
                    == '"' as i32
            {
                if !(*dest).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*dest as *mut libc::c_void);
                    } else {
                        free(*dest as *mut libc::c_void);
                    }
                }
                *dest = 0 as *mut libc::c_char;
                return -(5 as libc::c_int);
            }
            if len == 2 as libc::c_int as libc::c_ulong
                && *((*ct_param).gvalue).offset(0 as libc::c_int as isize) as libc::c_int
                    == '"' as i32
                && *((*ct_param).gvalue).offset(1 as libc::c_int as isize) as libc::c_int
                    == '"' as i32
            {
                if !(*dest).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*dest as *mut libc::c_void);
                    } else {
                        free(*dest as *mut libc::c_void);
                    }
                }
                *dest = 0 as *mut libc::c_char;
                return -(5 as libc::c_int);
            }
            boundary = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(len.wrapping_add(5 as libc::c_int as libc::c_ulong))
            } else {
                malloc(len.wrapping_add(5 as libc::c_int as libc::c_ulong))
            }) as *mut libc::c_char;
            if boundary.is_null() {
                if !(*dest).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(*dest as *mut libc::c_void);
                    } else {
                        free(*dest as *mut libc::c_void);
                    }
                }
                *dest = 0 as *mut libc::c_char;
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                boundary,
                b"\r\n\0\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
            osip_strncpy(
                boundary.offset(2 as libc::c_int as isize),
                b"--\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
            if *((*ct_param).gvalue).offset(0 as libc::c_int as isize) as libc::c_int
                == '"' as i32
                && *((*ct_param).gvalue)
                    .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '"' as i32
            {
                osip_strncpy(
                    boundary.offset(4 as libc::c_int as isize),
                    ((*ct_param).gvalue).offset(1 as libc::c_int as isize),
                    len.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                );
            } else {
                osip_strncpy(
                    boundary.offset(4 as libc::c_int as isize),
                    (*ct_param).gvalue,
                    len,
                );
            }
        }
    }
    let mut it_0: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut body: *mut osip_body_t = osip_list_get_first(&mut (*sip).bodies, &mut it_0)
        as *mut osip_body_t;
    while !body.is_null() {
        let mut body_length: size_t = 0;
        if !boundary.is_null() {
            message = osip_str_append(message, boundary);
            message = osip_strn_append(
                message,
                b"\r\n\0\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        i = osip_body_to_str(body, &mut tmp, &mut body_length);
        if i != 0 as libc::c_int {
            if !(*dest).is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(*dest as *mut libc::c_void);
                } else {
                    free(*dest as *mut libc::c_void);
                }
            }
            *dest = 0 as *mut libc::c_char;
            if !boundary.is_null() {
                if !boundary.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(boundary as *mut libc::c_void);
                    } else {
                        free(boundary as *mut libc::c_void);
                    }
                }
            }
            return i;
        }
        if malloc_size
            < ((message.offset_from(*dest) as libc::c_long
                + 100 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_add(body_length)
        {
            let mut size: size_t = message.offset_from(*dest) as libc::c_long as size_t;
            let mut offset_of_body: libc::c_int = 0;
            let mut offset_content_length_to_modify: libc::c_int = 0 as libc::c_int;
            offset_of_body = start_of_bodies.offset_from(*dest) as libc::c_long
                as libc::c_int;
            if !content_length_to_modify.is_null() {
                offset_content_length_to_modify = content_length_to_modify
                    .offset_from(*dest) as libc::c_long as libc::c_int;
            }
            malloc_size = (message.offset_from(*dest) as libc::c_long as libc::c_ulong)
                .wrapping_add(body_length)
                .wrapping_add(100 as libc::c_int as libc::c_ulong);
            *dest = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(*dest as *mut libc::c_void, malloc_size)
            } else {
                realloc(*dest as *mut libc::c_void, malloc_size)
            }) as *mut libc::c_char;
            if (*dest).is_null() {
                if !tmp.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(tmp as *mut libc::c_void);
                    } else {
                        free(tmp as *mut libc::c_void);
                    }
                }
                if !boundary.is_null() {
                    if !boundary.is_null() {
                        if osip_free_func.is_some() {
                            osip_free_func.unwrap()(boundary as *mut libc::c_void);
                        } else {
                            free(boundary as *mut libc::c_void);
                        }
                    }
                }
                return -(4 as libc::c_int);
            }
            start_of_bodies = (*dest).offset(offset_of_body as isize);
            if !content_length_to_modify.is_null() {
                content_length_to_modify = (*dest)
                    .offset(offset_content_length_to_modify as isize);
            }
            message = (*dest).offset(size as isize);
        }
        memcpy(message as *mut libc::c_void, tmp as *const libc::c_void, body_length);
        *message.offset(body_length as isize) = '\0' as i32 as libc::c_char;
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        message = message.offset(body_length as isize);
        body = osip_list_get_next(&mut it_0) as *mut osip_body_t;
    }
    if !boundary.is_null() {
        message = osip_str_append(message, boundary);
        message = osip_strn_append(
            message,
            b"--\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        message = osip_strn_append(
            message,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        if !boundary.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(boundary as *mut libc::c_void);
            } else {
                free(boundary as *mut libc::c_void);
            }
        }
        boundary = 0 as *mut libc::c_char;
    }
    if content_length_to_modify.is_null() {
        if !(*dest).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*dest as *mut libc::c_void);
            } else {
                free(*dest as *mut libc::c_void);
            }
        }
        *dest = 0 as *mut libc::c_char;
        return -(5 as libc::c_int);
    }
    let mut size_0: size_t = message.offset_from(start_of_bodies) as libc::c_long
        as size_t;
    let mut tmp2: [libc::c_char; 15] = [0; 15];
    total_length = (total_length as libc::c_ulong).wrapping_add(size_0) as size_t
        as size_t;
    snprintf(
        tmp2.as_mut_ptr(),
        15 as libc::c_int as libc::c_ulong,
        b"%i\0" as *const u8 as *const libc::c_char,
        size_0 as libc::c_int,
    );
    memcpy(
        content_length_to_modify
            .offset(5 as libc::c_int as isize)
            .offset(-(strlen(tmp2.as_mut_ptr()) as isize)) as *mut libc::c_void,
        tmp2.as_mut_ptr() as *const libc::c_void,
        strlen(tmp2.as_mut_ptr()),
    );
    (*sip).message_property = 1 as libc::c_int;
    (*sip)
        .message = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(total_length.wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc(total_length.wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if !((*sip).message).is_null() {
        memcpy(
            (*sip).message as *mut libc::c_void,
            *dest as *const libc::c_void,
            total_length,
        );
        *((*sip).message).offset(total_length as isize) = '\0' as i32 as libc::c_char;
        (*sip).message_length = total_length;
        if !message_length.is_null() {
            *message_length = total_length;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_to_str(
    mut sip: *mut osip_message_t,
    mut dest: *mut *mut libc::c_char,
    mut message_length: *mut size_t,
) -> libc::c_int {
    return _osip_message_to_str(sip, dest, message_length, 0 as libc::c_int);
}
pub unsafe extern "C" fn osip_message_to_str_sipfrag(
    mut sip: *mut osip_message_t,
    mut dest: *mut *mut libc::c_char,
    mut message_length: *mut size_t,
) -> libc::c_int {
    return _osip_message_to_str(sip, dest, message_length, 1 as libc::c_int);
}
