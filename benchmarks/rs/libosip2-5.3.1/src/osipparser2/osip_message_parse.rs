use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn osip_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_tolower(word: *mut libc::c_char) -> libc::c_int;
    fn osip_clrncpy(
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
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_uri_param_add(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        value: *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_uri_param_get_byname(
        url_params: *mut osip_list_t,
        name: *mut libc::c_char,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_uri_init(url: *mut *mut osip_uri_t) -> libc::c_int;
    fn osip_uri_free(url: *mut osip_uri_t);
    fn osip_uri_parse(url: *mut osip_uri_t, buf: *const libc::c_char) -> libc::c_int;
    fn osip_content_length_free(header: *mut osip_content_length_t);
    fn osip_message_set_content_length(
        sip: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_message_set_body_mime(
        sip: *mut osip_message_t,
        buf: *const libc::c_char,
        length: size_t,
    ) -> libc::c_int;
    fn osip_message_set_body(
        sip: *mut osip_message_t,
        buf: *const libc::c_char,
        length: size_t,
    ) -> libc::c_int;
    fn osip_message_set_header(
        sip: *mut osip_message_t,
        hname: *const libc::c_char,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn __osip_message_call_method(
        i: libc::c_int,
        dest: *mut osip_message_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn __osip_message_is_header_comma_separated(
        hname: *const libc::c_char,
    ) -> libc::c_int;
    fn __osip_message_is_known_header(hname: *const libc::c_char) -> libc::c_int;
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
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub type osip_generic_param_t = osip_uri_param_t;
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
pub struct code_to_reason {
    pub code: libc::c_int,
    pub reason: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn __osip_message_startline_parsereq(
    mut dest: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut headers: *mut *const libc::c_char,
) -> libc::c_int {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut requesturi: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    (*dest).sip_method = 0 as *mut libc::c_char;
    (*dest).status_code = 0 as libc::c_int;
    (*dest).reason_phrase = 0 as *mut libc::c_char;
    *headers = buf;
    p2 = strchr(buf, ' ' as i32);
    if p2.is_null() {
        return -(5 as libc::c_int);
    }
    if *p2.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *p2.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return -(5 as libc::c_int);
    }
    if p2.offset_from(buf) as libc::c_long == 0 as libc::c_int as libc::c_long {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"No space allowed here\n\0" as *const u8 as *const libc::c_char,
        );
        return -(5 as libc::c_int);
    }
    (*dest)
        .sip_method = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (p2.offset_from(buf) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        )
    } else {
        malloc(
            (p2.offset_from(buf) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*dest).sip_method).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy((*dest).sip_method, buf, p2.offset_from(buf) as libc::c_long as size_t);
    p1 = strchr(p2.offset(2 as libc::c_int as isize), ' ' as i32);
    if p1.is_null() {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Uncompliant request-uri\n\0" as *const u8 as *const libc::c_char,
        );
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        return -(5 as libc::c_int);
    }
    if (p1.offset_from(p2) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        return -(5 as libc::c_int);
    }
    requesturi = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(p1.offset_from(p2) as libc::c_long as size_t)
    } else {
        malloc(p1.offset_from(p2) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if requesturi.is_null() {
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        requesturi,
        p2.offset(1 as libc::c_int as isize),
        (p1.offset_from(p2) as libc::c_long - 1 as libc::c_int as libc::c_long) as size_t,
    );
    i = osip_uri_init(&mut (*dest).req_uri);
    if i != 0 as libc::c_int {
        if !requesturi.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(requesturi as *mut libc::c_void);
            } else {
                free(requesturi as *mut libc::c_void);
            }
        }
        requesturi = 0 as *mut libc::c_char;
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        return -(4 as libc::c_int);
    }
    i = osip_uri_parse((*dest).req_uri, requesturi);
    if !requesturi.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(requesturi as *mut libc::c_void);
        } else {
            free(requesturi as *mut libc::c_void);
        }
    }
    if i != 0 as libc::c_int {
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        osip_uri_free((*dest).req_uri);
        (*dest).req_uri = 0 as *mut osip_uri_t;
        return -(5 as libc::c_int);
    }
    let mut hp: *const libc::c_char = p1;
    hp = hp.offset(1);
    hp;
    if *hp as libc::c_int == '\0' as i32
        || *hp.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *hp.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *hp.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *hp.offset(4 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *hp.offset(5 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *hp.offset(6 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Uncomplete request line\n\0" as *const u8 as *const libc::c_char,
        );
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        osip_uri_free((*dest).req_uri);
        (*dest).req_uri = 0 as *mut osip_uri_t;
        return -(5 as libc::c_int);
    }
    if *hp.offset(0 as libc::c_int as isize) as libc::c_int != 'S' as i32
        && *hp.offset(0 as libc::c_int as isize) as libc::c_int != 's' as i32
        || *hp.offset(1 as libc::c_int as isize) as libc::c_int != 'I' as i32
            && *hp.offset(1 as libc::c_int as isize) as libc::c_int != 'i' as i32
        || *hp.offset(2 as libc::c_int as isize) as libc::c_int != 'P' as i32
            && *hp.offset(2 as libc::c_int as isize) as libc::c_int != 'p' as i32
        || *hp.offset(3 as libc::c_int as isize) as libc::c_int != '/' as i32
    {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"No crlf found/No SIP/2.0 found\n\0" as *const u8 as *const libc::c_char,
        );
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        osip_uri_free((*dest).req_uri);
        (*dest).req_uri = 0 as *mut osip_uri_t;
        return -(5 as libc::c_int);
    }
    hp = hp.offset(4 as libc::c_int as isize);
    while *hp as libc::c_int != '\r' as i32 && *hp as libc::c_int != '\n' as i32 {
        if *hp != 0 {
            if *hp as libc::c_int >= '0' as i32 && *hp as libc::c_int <= '9' as i32 {
                hp = hp.offset(1);
                hp;
            } else if *hp as libc::c_int == '.' as i32 {
                hp = hp.offset(1);
                hp;
            } else {
                osip_trace(
                    b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                    146 as libc::c_int,
                    TRACE_LEVEL2,
                    0 as *mut FILE,
                    b"incorrect sip version string\n\0" as *const u8
                        as *const libc::c_char,
                );
                if !((*dest).sip_method).is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
                    } else {
                        free((*dest).sip_method as *mut libc::c_void);
                    }
                }
                (*dest).sip_method = 0 as *mut libc::c_char;
                osip_uri_free((*dest).req_uri);
                (*dest).req_uri = 0 as *mut osip_uri_t;
                return -(5 as libc::c_int);
            }
        } else {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                155 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"No crlf found\n\0" as *const u8 as *const libc::c_char,
            );
            if !((*dest).sip_method).is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
                } else {
                    free((*dest).sip_method as *mut libc::c_void);
                }
            }
            (*dest).sip_method = 0 as *mut libc::c_char;
            osip_uri_free((*dest).req_uri);
            (*dest).req_uri = 0 as *mut osip_uri_t;
            return -(5 as libc::c_int);
        }
    }
    if (hp.offset_from(p1) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        osip_uri_free((*dest).req_uri);
        (*dest).req_uri = 0 as *mut osip_uri_t;
        return -(5 as libc::c_int);
    }
    (*dest)
        .sip_version = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(hp.offset_from(p1) as libc::c_long as size_t)
    } else {
        malloc(hp.offset_from(p1) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*dest).sip_version).is_null() {
        if !((*dest).sip_method).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_method as *mut libc::c_void);
            } else {
                free((*dest).sip_method as *mut libc::c_void);
            }
        }
        (*dest).sip_method = 0 as *mut libc::c_char;
        osip_uri_free((*dest).req_uri);
        (*dest).req_uri = 0 as *mut osip_uri_t;
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*dest).sip_version,
        p1.offset(1 as libc::c_int as isize),
        (hp.offset_from(p1) as libc::c_long - 1 as libc::c_int as libc::c_long) as size_t,
    );
    if 0 as libc::c_int
        != osip_strcasecmp(
            (*dest).sip_version,
            b"SIP/2.0\0" as *const u8 as *const libc::c_char,
        )
    {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Wrong version number\n\0" as *const u8 as *const libc::c_char,
        );
    }
    hp = hp.offset(1);
    hp;
    if *hp as libc::c_int != 0
        && '\r' as i32 == *hp.offset(-(1 as libc::c_int) as isize) as libc::c_int
        && '\n' as i32 == *hp.offset(0 as libc::c_int as isize) as libc::c_int
    {
        hp = hp.offset(1);
        hp;
    }
    *headers = hp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn __osip_message_startline_parseresp(
    mut dest: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut headers: *mut *const libc::c_char,
) -> libc::c_int {
    let mut statuscode: *const libc::c_char = 0 as *const libc::c_char;
    let mut reasonphrase: *const libc::c_char = 0 as *const libc::c_char;
    (*dest).req_uri = 0 as *mut osip_uri_t;
    (*dest).sip_method = 0 as *mut libc::c_char;
    *headers = buf;
    statuscode = strchr(buf, ' ' as i32);
    if statuscode.is_null() {
        return -(5 as libc::c_int);
    }
    if (statuscode.offset_from(*headers) as libc::c_long)
        < 7 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*dest)
        .sip_version = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (statuscode.offset_from(*headers) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (statuscode.offset_from(*headers) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*dest).sip_version).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*dest).sip_version,
        *headers,
        statuscode.offset_from(*headers) as libc::c_long as size_t,
    );
    reasonphrase = strchr(statuscode.offset(1 as libc::c_int as isize), ' ' as i32);
    if reasonphrase.is_null() {
        if !((*dest).sip_version).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_version as *mut libc::c_void);
            } else {
                free((*dest).sip_version as *mut libc::c_void);
            }
        }
        (*dest).sip_version = 0 as *mut libc::c_char;
        return -(5 as libc::c_int);
    }
    if sscanf(
        statuscode.offset(1 as libc::c_int as isize),
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut (*dest).status_code as *mut libc::c_int,
    ) != 1 as libc::c_int
    {
        return -(5 as libc::c_int);
    }
    if (*dest).status_code == 0 as libc::c_int {
        return -(5 as libc::c_int);
    }
    let mut hp: *const libc::c_char = reasonphrase;
    while *hp as libc::c_int != '\r' as i32 && *hp as libc::c_int != '\n' as i32 {
        if *hp != 0 {
            hp = hp.offset(1);
            hp;
        } else {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"No crlf found\n\0" as *const u8 as *const libc::c_char,
            );
            return -(5 as libc::c_int);
        }
    }
    (*dest)
        .reason_phrase = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(hp.offset_from(reasonphrase) as libc::c_long as size_t)
    } else {
        malloc(hp.offset_from(reasonphrase) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*dest).reason_phrase).is_null() {
        if !((*dest).sip_version).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*dest).sip_version as *mut libc::c_void);
            } else {
                free((*dest).sip_version as *mut libc::c_void);
            }
        }
        (*dest).sip_version = 0 as *mut libc::c_char;
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*dest).reason_phrase,
        reasonphrase.offset(1 as libc::c_int as isize),
        (hp.offset_from(reasonphrase) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    hp = hp.offset(1);
    hp;
    if *hp as libc::c_int != 0
        && '\r' as i32 == *hp.offset(-(1 as libc::c_int) as isize) as libc::c_int
        && '\n' as i32 == *hp.offset(0 as libc::c_int as isize) as libc::c_int
    {
        hp = hp.offset(1);
        hp;
    }
    *headers = hp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn __osip_message_startline_parse(
    mut dest: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut headers: *mut *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int
        == strncmp(
            buf,
            b"SIP/\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        )
    {
        return __osip_message_startline_parseresp(dest, buf, headers)
    } else {
        return __osip_message_startline_parsereq(dest, buf, headers)
    };
}
pub unsafe extern "C" fn __osip_find_next_occurence(
    mut str: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut index_of_str: *mut *const libc::c_char,
    mut end_of_buf: *const libc::c_char,
) -> libc::c_int {
    let mut slen: size_t = 0;
    *index_of_str = 0 as *const libc::c_char;
    if str.is_null() || buf.is_null() {
        return -(2 as libc::c_int);
    }
    slen = strlen(str);
    while slen < end_of_buf.offset_from(buf) as libc::c_long as size_t {
        if memcmp(str as *const libc::c_void, buf as *const libc::c_void, slen) == 0 {
            *index_of_str = buf;
            return 0 as libc::c_int;
        }
        buf = buf.offset(1);
        buf;
    }
    return -(5 as libc::c_int);
}
unsafe extern "C" fn osip_util_replace_all_lws(mut sip_message: *mut libc::c_char) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if sip_message.is_null() {
        return;
    }
    tmp = sip_message;
    while *tmp.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        if '\0' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
            || '\0' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
            || '\0' as i32 == *tmp.offset(2 as libc::c_int as isize) as libc::c_int
            || '\0' as i32 == *tmp.offset(3 as libc::c_int as isize) as libc::c_int
        {
            return;
        }
        if '\r' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
            && '\n' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
            && '\r' as i32 == *tmp.offset(2 as libc::c_int as isize) as libc::c_int
            && '\n' as i32 == *tmp.offset(3 as libc::c_int as isize) as libc::c_int
            || '\r' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                && '\r' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
            || '\n' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                && '\n' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
        {
            return;
        }
        if '\r' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
            && '\n' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
            && (' ' as i32 == *tmp.offset(2 as libc::c_int as isize) as libc::c_int
                || '\t' as i32 == *tmp.offset(2 as libc::c_int as isize) as libc::c_int)
            || '\r' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                && (' ' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
                    || '\t' as i32
                        == *tmp.offset(1 as libc::c_int as isize) as libc::c_int)
            || '\n' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                && (' ' as i32 == *tmp.offset(1 as libc::c_int as isize) as libc::c_int
                    || '\t' as i32
                        == *tmp.offset(1 as libc::c_int as isize) as libc::c_int)
        {
            *tmp.offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            *tmp.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            tmp = tmp.offset(2 as libc::c_int as isize);
            while '\t' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                || ' ' as i32 == *tmp.offset(0 as libc::c_int as isize) as libc::c_int
            {
                *tmp.offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
                tmp = tmp.offset(1);
                tmp;
            }
            if *tmp.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                return;
            }
        }
        tmp = tmp.offset(1);
        tmp;
    }
}
pub unsafe extern "C" fn __osip_find_next_crlf(
    mut start_of_header: *const libc::c_char,
    mut end_of_header: *mut *const libc::c_char,
) -> libc::c_int {
    let mut soh: *const libc::c_char = start_of_header;
    *end_of_header = 0 as *const libc::c_char;
    while '\r' as i32 != *soh as libc::c_int && '\n' as i32 != *soh as libc::c_int {
        if *soh != 0 {
            soh = soh.offset(1);
            soh;
        } else {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                352 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"Final CRLF is missing\n\0" as *const u8 as *const libc::c_char,
            );
            return -(5 as libc::c_int);
        }
    }
    if '\r' as i32 == *soh.offset(0 as libc::c_int as isize) as libc::c_int
        && '\n' as i32 == *soh.offset(1 as libc::c_int as isize) as libc::c_int
    {
        soh = soh.offset(1 as libc::c_int as isize);
    }
    if ' ' as i32 == *soh.offset(1 as libc::c_int as isize) as libc::c_int
        || '\t' as i32 == *soh.offset(1 as libc::c_int as isize) as libc::c_int
    {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            370 as libc::c_int,
            TRACE_LEVEL1,
            0 as *mut FILE,
            b"Message that contains LWS must be processed with osip_util_replace_all_lws(char *tmp) before being parsed.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(2 as libc::c_int);
    }
    *end_of_header = soh.offset(1 as libc::c_int as isize);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_find_next_crlfcrlf(
    mut start_of_part: *const libc::c_char,
    mut end_of_part: *mut *const libc::c_char,
) -> libc::c_int {
    let mut start_of_line: *const libc::c_char = 0 as *const libc::c_char;
    let mut end_of_line: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    start_of_line = start_of_part;
    loop {
        i = __osip_find_next_crlf(start_of_line, &mut end_of_line);
        if !(i == -(2 as libc::c_int)) {
            if i != 0 as libc::c_int {
                osip_trace(
                    b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                    390 as libc::c_int,
                    TRACE_LEVEL2,
                    0 as *mut FILE,
                    b"Final CRLF is missing\n\0" as *const u8 as *const libc::c_char,
                );
                return i;
            }
        }
        if '\0' as i32 == *end_of_line.offset(0 as libc::c_int as isize) as libc::c_int {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                395 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"Final CRLF is missing\n\0" as *const u8 as *const libc::c_char,
            );
            return -(5 as libc::c_int);
        } else if '\r' as i32
            == *end_of_line.offset(0 as libc::c_int as isize) as libc::c_int
        {
            if '\n' as i32
                == *end_of_line.offset(1 as libc::c_int as isize) as libc::c_int
            {
                end_of_line = end_of_line.offset(1);
                end_of_line;
            }
            *end_of_part = end_of_line.offset(1 as libc::c_int as isize);
            return 0 as libc::c_int;
        } else if '\n' as i32
            == *end_of_line.offset(0 as libc::c_int as isize) as libc::c_int
        {
            *end_of_part = end_of_line.offset(1 as libc::c_int as isize);
            return 0 as libc::c_int;
        }
        start_of_line = end_of_line;
    };
}
unsafe extern "C" fn osip_message_set__header(
    mut sip: *mut osip_message_t,
    mut hname: *const libc::c_char,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut my_index: libc::c_int = 0;
    if hname.is_null() {
        return -(5 as libc::c_int);
    }
    my_index = __osip_message_is_known_header(hname);
    if my_index >= 0 as libc::c_int {
        let mut ret: libc::c_int = 0;
        ret = __osip_message_call_method(my_index, sip, hvalue);
        if ret != 0 as libc::c_int {
            return ret;
        }
        return 0 as libc::c_int;
    }
    if osip_message_set_header(sip, hname, hvalue) != 0 as libc::c_int {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int,
            TRACE_LEVEL3,
            0 as *mut FILE,
            b"Could not set unknown header\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_multiple_header(
    mut sip: *mut osip_message_t,
    mut hname: *mut libc::c_char,
    mut hvalue: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut beg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inquotes: libc::c_int = 0;
    let mut inuri: libc::c_int = 0;
    osip_tolower(hname);
    if hvalue.is_null() {
        i = osip_message_set__header(sip, hname, hvalue);
        if i != 0 as libc::c_int {
            return i;
        }
        return 0 as libc::c_int;
    }
    ptr = hvalue;
    comma = strchr(ptr, ',' as i32);
    if comma.is_null()
        || __osip_message_is_header_comma_separated(hname) != 0 as libc::c_int
    {
        i = osip_message_set__header(sip, hname, hvalue);
        if i != 0 as libc::c_int {
            return i;
        }
        return 0 as libc::c_int;
    }
    beg = hvalue;
    inquotes = 0 as libc::c_int;
    inuri = 0 as libc::c_int;
    loop {
        let mut current_block_55: u64;
        match *ptr as libc::c_int {
            34 => {
                i = 0 as libc::c_int;
                p = ptr.offset(-(1 as libc::c_int as isize));
                while p >= beg && *p as libc::c_int == '\\' as i32 {
                    p = p.offset(-1);
                    p;
                    i += 1;
                    i;
                }
                if i % 2 as libc::c_int == 0 as libc::c_int {
                    inquotes = (inquotes == 0) as libc::c_int;
                }
                current_block_55 = 7420279277351916581;
            }
            60 => {
                if inquotes == 0 {
                    if inuri == 0 {
                        if (osip_strncasecmp(
                            ptr.offset(1 as libc::c_int as isize),
                            b"sip:\0" as *const u8 as *const libc::c_char,
                            4 as libc::c_int as size_t,
                        ) == 0 as libc::c_int
                            || osip_strncasecmp(
                                ptr.offset(1 as libc::c_int as isize),
                                b"sips:\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as size_t,
                            ) == 0 as libc::c_int
                            || osip_strncasecmp(
                                ptr.offset(1 as libc::c_int as isize),
                                b"http:\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as size_t,
                            ) == 0 as libc::c_int
                            || osip_strncasecmp(
                                ptr.offset(1 as libc::c_int as isize),
                                b"https:\0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as size_t,
                            ) == 0 as libc::c_int
                            || osip_strncasecmp(
                                ptr.offset(1 as libc::c_int as isize),
                                b"tel:\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as size_t,
                            ) == 0 as libc::c_int)
                            && !(strchr(ptr, '>' as i32)).is_null()
                        {
                            inuri = 1 as libc::c_int;
                        }
                    }
                }
                current_block_55 = 7420279277351916581;
            }
            62 => {
                if inquotes == 0 {
                    if inuri != 0 {
                        inuri = 0 as libc::c_int;
                    }
                }
                current_block_55 = 7420279277351916581;
            }
            0 => {
                inquotes = 0 as libc::c_int;
                inuri = 0 as libc::c_int;
                current_block_55 = 10370040694135445195;
            }
            44 => {
                current_block_55 = 10370040694135445195;
            }
            _ => {
                current_block_55 = 7420279277351916581;
            }
        }
        match current_block_55 {
            10370040694135445195 => {
                if inquotes == 0 && inuri == 0 {
                    let mut avalue: *mut libc::c_char = 0 as *mut libc::c_char;
                    if *beg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                    {
                        return 0 as libc::c_int;
                    }
                    end = ptr;
                    if (end.offset_from(beg) as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                        < 2 as libc::c_int as libc::c_long
                    {
                        beg = end.offset(1 as libc::c_int as isize);
                        current_block_55 = 7420279277351916581;
                    } else {
                        avalue = (if osip_malloc_func.is_some() {
                            osip_malloc_func
                                .unwrap()(
                                (end.offset_from(beg) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long) as size_t,
                            )
                        } else {
                            malloc(
                                (end.offset_from(beg) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                            )
                        }) as *mut libc::c_char;
                        if avalue.is_null() {
                            return -(4 as libc::c_int);
                        }
                        osip_clrncpy(
                            avalue,
                            beg,
                            end.offset_from(beg) as libc::c_long as size_t,
                        );
                        i = osip_message_set__header(sip, hname, avalue);
                        if !avalue.is_null() {
                            if osip_free_func.is_some() {
                                osip_free_func.unwrap()(avalue as *mut libc::c_void);
                            } else {
                                free(avalue as *mut libc::c_void);
                            }
                        }
                        if i != 0 as libc::c_int {
                            return i;
                        }
                        beg = end.offset(1 as libc::c_int as isize);
                        current_block_55 = 13619784596304402172;
                    }
                } else {
                    current_block_55 = 13619784596304402172;
                }
                match current_block_55 {
                    7420279277351916581 => {}
                    _ => {
                        if *ptr as libc::c_int == '\0' as i32 {
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
            _ => {}
        }
        ptr = ptr.offset(1);
        ptr;
    };
}
unsafe extern "C" fn msg_headers_parse(
    mut sip: *mut osip_message_t,
    mut start_of_header: *const libc::c_char,
    mut body: *mut *const libc::c_char,
) -> libc::c_int {
    let mut colon_index: *const libc::c_char = 0 as *const libc::c_char;
    let mut hname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_of_header: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    loop {
        if *start_of_header.offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
        {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                584 as libc::c_int,
                TRACE_LEVEL4,
                0 as *mut FILE,
                b"SIP message does not end with CRLFCRLF\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        i = __osip_find_next_crlf(start_of_header, &mut end_of_header);
        if !(i == -(2 as libc::c_int)) {
            if i != 0 as libc::c_int {
                osip_trace(
                    b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                    592 as libc::c_int,
                    TRACE_LEVEL2,
                    0 as *mut FILE,
                    b"End of header Not found\n\0" as *const u8 as *const libc::c_char,
                );
                return i;
            }
        }
        if *start_of_header.offset(0 as libc::c_int as isize) as libc::c_int
            == '\r' as i32
            || *start_of_header.offset(0 as libc::c_int as isize) as libc::c_int
                == '\n' as i32
        {
            *body = start_of_header;
            return 0 as libc::c_int;
        }
        colon_index = strchr(start_of_header, ':' as i32);
        if colon_index.is_null() {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                607 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"End of header Not found\n\0" as *const u8 as *const libc::c_char,
            );
            return -(5 as libc::c_int);
        }
        if (colon_index.offset_from(start_of_header) as libc::c_long
            + 1 as libc::c_int as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        if end_of_header <= colon_index {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                615 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"Malformed message\n\0" as *const u8 as *const libc::c_char,
            );
            return -(5 as libc::c_int);
        }
        hname = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (colon_index.offset_from(start_of_header) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            )
        } else {
            malloc(
                (colon_index.offset_from(start_of_header) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        if hname.is_null() {
            return -(4 as libc::c_int);
        }
        osip_clrncpy(
            hname,
            start_of_header,
            colon_index.offset_from(start_of_header) as libc::c_long as size_t,
        );
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        if *end_of_header.offset(-(2 as libc::c_int) as isize) as libc::c_int
            == '\r' as i32
            || *end_of_header.offset(-(2 as libc::c_int) as isize) as libc::c_int
                == '\n' as i32
        {
            end = end_of_header.offset(-(2 as libc::c_int as isize));
        } else {
            end = end_of_header.offset(-(1 as libc::c_int as isize));
        }
        if (end.offset_from(colon_index) as libc::c_long)
            < 2 as libc::c_int as libc::c_long
        {
            hvalue = 0 as *mut libc::c_char;
        } else {
            hvalue = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(
                    (end.offset_from(colon_index) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as size_t,
                )
            } else {
                malloc(
                    (end.offset_from(colon_index) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                )
            }) as *mut libc::c_char;
            if hvalue.is_null() {
                if !hname.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(hname as *mut libc::c_void);
                    } else {
                        free(hname as *mut libc::c_void);
                    }
                }
                return -(4 as libc::c_int);
            }
            osip_clrncpy(
                hvalue,
                colon_index.offset(1 as libc::c_int as isize),
                (end.offset_from(colon_index) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
        i = osip_message_set_multiple_header(sip, hname, hvalue);
        if !hname.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(hname as *mut libc::c_void);
            } else {
                free(hname as *mut libc::c_void);
            }
        }
        if !hvalue.is_null() {
            if !hvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(hvalue as *mut libc::c_void);
                } else {
                    free(hvalue as *mut libc::c_void);
                }
            }
        }
        if i != 0 as libc::c_int {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                663 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"End of header Not found\n\0" as *const u8 as *const libc::c_char,
            );
            return -(5 as libc::c_int);
        }
        start_of_header = end_of_header;
    };
}
unsafe extern "C" fn msg_osip_body_parse(
    mut sip: *mut osip_message_t,
    mut start_of_buf: *const libc::c_char,
    mut next_body: *mut *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut start_of_body: *const libc::c_char = 0 as *const libc::c_char;
    let mut end_of_body: *const libc::c_char = 0 as *const libc::c_char;
    let mut end_of_buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut sep_boundary: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len_sep_boundary: size_t = 0;
    let mut ct_param: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    if ((*sip).content_type).is_null() || ((*(*sip).content_type).type_0).is_null()
        || ((*(*sip).content_type).subtype).is_null()
    {
        return 0 as libc::c_int;
    }
    if 0 as libc::c_int
        != osip_strcasecmp(
            (*(*sip).content_type).type_0,
            b"multipart\0" as *const u8 as *const libc::c_char,
        )
    {
        let mut osip_body_len: size_t = 0;
        if *start_of_buf.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            return -(5 as libc::c_int);
        }
        if '\r' as i32 == *start_of_buf.offset(0 as libc::c_int as isize) as libc::c_int
        {
            if '\n' as i32
                == *start_of_buf.offset(1 as libc::c_int as isize) as libc::c_int
            {
                start_of_body = start_of_buf.offset(2 as libc::c_int as isize);
            } else {
                start_of_body = start_of_buf.offset(1 as libc::c_int as isize);
            }
        } else if '\n' as i32
            == *start_of_buf.offset(0 as libc::c_int as isize) as libc::c_int
        {
            start_of_body = start_of_buf.offset(1 as libc::c_int as isize);
        } else {
            return -(5 as libc::c_int)
        }
        length = length
            .wrapping_sub(
                start_of_body.offset_from(start_of_buf) as libc::c_long as libc::c_ulong,
            );
        if length <= 0 as libc::c_int as libc::c_ulong {
            return -(5 as libc::c_int);
        }
        if !((*sip).content_length).is_null() {
            osip_body_len = osip_atoi((*(*sip).content_length).value) as size_t;
        } else {
            let mut tmp_0: [libc::c_char; 16] = [0; 16];
            osip_body_len = length;
            sprintf(
                tmp_0.as_mut_ptr(),
                b"%i\0" as *const u8 as *const libc::c_char,
                osip_body_len as libc::c_int,
            );
            i = osip_message_set_content_length(sip, tmp_0.as_mut_ptr());
            if i != 0 as libc::c_int {
                return i;
            }
        }
        if length < osip_body_len {
            osip_trace(
                b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
                734 as libc::c_int,
                TRACE_LEVEL2,
                0 as *mut FILE,
                b"Message was not receieved enterely. length=%i osip_body_len=%i\n\0"
                    as *const u8 as *const libc::c_char,
                length as libc::c_int,
                osip_body_len as libc::c_int,
            );
            return -(5 as libc::c_int);
        }
        end_of_body = start_of_body.offset(osip_body_len as isize);
        tmp = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (end_of_body.offset_from(start_of_body) as libc::c_long
                    + 2 as libc::c_int as libc::c_long) as size_t,
            )
        } else {
            malloc(
                (end_of_body.offset_from(start_of_body) as libc::c_long
                    + 2 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        if tmp.is_null() {
            return -(4 as libc::c_int);
        }
        memcpy(
            tmp as *mut libc::c_void,
            start_of_body as *const libc::c_void,
            end_of_body.offset_from(start_of_body) as libc::c_long as libc::c_ulong,
        );
        *tmp
            .offset(
                end_of_body.offset_from(start_of_body) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        i = osip_message_set_body(
            sip,
            tmp,
            end_of_body.offset_from(start_of_body) as libc::c_long as size_t,
        );
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        if i != 0 as libc::c_int {
            return i;
        }
        return 0 as libc::c_int;
    }
    i = osip_uri_param_get_byname(
        &mut (*(*sip).content_type).gen_params,
        b"boundary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut ct_param,
    );
    if i != 0 as libc::c_int {
        return i;
    }
    if ct_param.is_null() {
        return -(5 as libc::c_int);
    }
    if ((*ct_param).gvalue).is_null() {
        return -(5 as libc::c_int);
    }
    let mut boundary_prefix: *const libc::c_char = b"\n--\0" as *const u8
        as *const libc::c_char;
    let mut len: size_t = strlen((*ct_param).gvalue);
    sep_boundary = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len.wrapping_add(4 as libc::c_int as libc::c_ulong))
    } else {
        malloc(len.wrapping_add(4 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if sep_boundary.is_null() {
        return -(4 as libc::c_int);
    }
    strcpy(sep_boundary, boundary_prefix);
    if *((*ct_param).gvalue).offset(0 as libc::c_int as isize) as libc::c_int
        == '"' as i32
        && *((*ct_param).gvalue)
            .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '"' as i32
    {
        strncat(
            sep_boundary,
            ((*ct_param).gvalue).offset(1 as libc::c_int as isize),
            len.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
    } else {
        strncat(sep_boundary, (*ct_param).gvalue, len);
    }
    len_sep_boundary = strlen(sep_boundary);
    *next_body = 0 as *const libc::c_char;
    start_of_body = start_of_buf;
    end_of_buf = start_of_buf.offset(length as isize);
    loop {
        let mut body_len: size_t = 0 as libc::c_int as size_t;
        i = __osip_find_next_occurence(
            sep_boundary,
            start_of_body,
            &mut start_of_body,
            end_of_buf,
        );
        if i != 0 as libc::c_int {
            if !sep_boundary.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(sep_boundary as *mut libc::c_void);
                } else {
                    free(sep_boundary as *mut libc::c_void);
                }
            }
            return i;
        }
        i = __osip_find_next_occurence(
            sep_boundary,
            start_of_body.offset(len_sep_boundary as isize),
            &mut end_of_body,
            end_of_buf,
        );
        if i != 0 as libc::c_int {
            if !sep_boundary.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(sep_boundary as *mut libc::c_void);
                } else {
                    free(sep_boundary as *mut libc::c_void);
                }
            }
            return i;
        }
        start_of_body = start_of_body
            .offset(len_sep_boundary as isize)
            .offset(1 as libc::c_int as isize);
        if '\n' as i32 == *start_of_body.offset(0 as libc::c_int as isize) as libc::c_int
            || '\r' as i32
                == *start_of_body.offset(0 as libc::c_int as isize) as libc::c_int
        {
            start_of_body = start_of_body.offset(1);
            start_of_body;
        }
        if end_of_body <= start_of_body {
            if !sep_boundary.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(sep_boundary as *mut libc::c_void);
                } else {
                    free(sep_boundary as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        body_len = end_of_body.offset_from(start_of_body) as libc::c_long as size_t;
        if *end_of_body.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == '\r' as i32
        {
            body_len = body_len.wrapping_sub(1);
            body_len;
        }
        tmp = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(body_len.wrapping_add(2 as libc::c_int as libc::c_ulong))
        } else {
            malloc(body_len.wrapping_add(2 as libc::c_int as libc::c_ulong))
        }) as *mut libc::c_char;
        if tmp.is_null() {
            if !sep_boundary.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(sep_boundary as *mut libc::c_void);
                } else {
                    free(sep_boundary as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        memcpy(tmp as *mut libc::c_void, start_of_body as *const libc::c_void, body_len);
        *tmp.offset(body_len as isize) = '\0' as i32 as libc::c_char;
        i = osip_message_set_body_mime(sip, tmp, body_len);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        if i != 0 as libc::c_int {
            if !sep_boundary.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(sep_boundary as *mut libc::c_void);
                } else {
                    free(sep_boundary as *mut libc::c_void);
                }
            }
            return i;
        }
        if strncmp(
            end_of_body.offset(len_sep_boundary as isize),
            b"--\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            *next_body = end_of_body;
            if !sep_boundary.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(sep_boundary as *mut libc::c_void);
                } else {
                    free(sep_boundary as *mut libc::c_void);
                }
            }
            return 0 as libc::c_int;
        }
        start_of_body = end_of_body;
    };
}
unsafe extern "C" fn _osip_message_parse(
    mut sip: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut length: size_t,
    mut sipfrag: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut next_header_index: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut beg: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(length.wrapping_add(2 as libc::c_int as libc::c_ulong))
    } else {
        malloc(length.wrapping_add(2 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if tmp.is_null() {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Could not allocate memory.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(4 as libc::c_int);
    }
    beg = tmp;
    memcpy(tmp as *mut libc::c_void, buf as *const libc::c_void, length);
    *tmp.offset(length as isize) = '\0' as i32 as libc::c_char;
    tmp = tmp
        .offset(strspn(tmp, b"\r\n\0" as *const u8 as *const libc::c_char) as isize);
    osip_util_replace_all_lws(tmp);
    i = __osip_message_startline_parse(sip, tmp, &mut next_header_index);
    if i != 0 as libc::c_int && sipfrag == 0 {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            886 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Could not parse start line of message.\n\0" as *const u8
                as *const libc::c_char,
        );
        if !beg.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(beg as *mut libc::c_void);
            } else {
                free(beg as *mut libc::c_void);
            }
        }
        return i;
    }
    tmp = next_header_index as *mut libc::c_char;
    i = msg_headers_parse(sip, tmp, &mut next_header_index);
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            897 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"error in msg_headers_parse()\n\0" as *const u8 as *const libc::c_char,
        );
        if !beg.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(beg as *mut libc::c_void);
            } else {
                free(beg as *mut libc::c_void);
            }
        }
        return i;
    }
    tmp = next_header_index as *mut libc::c_char;
    if !((*sip).content_length).is_null() && ((*(*sip).content_length).value).is_null() {
        osip_content_length_free((*sip).content_length);
        (*sip).content_length = 0 as *mut osip_content_length_t;
    }
    if !(!((*sip).content_length).is_null()
        && !((*(*sip).content_length).value).is_null()
        && atoi((*(*sip).content_length).value) > 0 as libc::c_int)
    {
        if !(((*sip).content_length).is_null()
            && '\r' as i32
                == *next_header_index.offset(0 as libc::c_int as isize) as libc::c_int
            && '\n' as i32
                == *next_header_index.offset(1 as libc::c_int as isize) as libc::c_int
            && length
                .wrapping_sub(tmp.offset_from(beg) as libc::c_long as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                > 0 as libc::c_int as libc::c_ulong)
        {
            if ((*sip).content_length).is_null()
                && '\n' as i32
                    == *next_header_index.offset(0 as libc::c_int as isize)
                        as libc::c_int
                && length
                    .wrapping_sub(tmp.offset_from(beg) as libc::c_long as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    > 0 as libc::c_int as libc::c_ulong
            {} else {
                if ((*sip).content_length).is_null() {
                    osip_message_set_content_length(
                        sip,
                        b"0\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !beg.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(beg as *mut libc::c_void);
                    } else {
                        free(beg as *mut libc::c_void);
                    }
                }
                return 0 as libc::c_int;
            }
        }
    }
    i = msg_osip_body_parse(
        sip,
        tmp,
        &mut next_header_index,
        length.wrapping_sub(tmp.offset_from(beg) as libc::c_long as libc::c_ulong),
    );
    if !beg.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(beg as *mut libc::c_void);
        } else {
            free(beg as *mut libc::c_void);
        }
    }
    if i != 0 as libc::c_int {
        osip_trace(
            b"osip_message_parse.c\0" as *const u8 as *const libc::c_char,
            928 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"error in msg_osip_body_parse()\n\0" as *const u8 as *const libc::c_char,
        );
        return i;
    }
    if ((*sip).content_length).is_null() {
        osip_message_set_content_length(sip, b"0\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_parse(
    mut sip: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    return _osip_message_parse(sip, buf, length, 0 as libc::c_int);
}
pub unsafe extern "C" fn osip_message_parse_sipfrag(
    mut sip: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    return _osip_message_parse(sip, buf, length, 1 as libc::c_int);
}
pub unsafe extern "C" fn osip_message_fix_last_via_header(
    mut request: *mut osip_message_t,
    mut ip_addr: *const libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut rport: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    if request.is_null() {
        return -(2 as libc::c_int);
    }
    if (*request).status_code != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    via = osip_list_get(&mut (*request).vias, 0 as libc::c_int) as *mut osip_via_t;
    if via.is_null() || ((*via).host).is_null() {
        return -(2 as libc::c_int);
    }
    osip_uri_param_get_byname(
        &mut (*via).via_params,
        b"rport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut rport,
    );
    if !rport.is_null() {
        if ((*rport).gvalue).is_null() {
            (*rport)
                .gvalue = (if osip_malloc_func.is_some() {
                osip_malloc_func.unwrap()(9 as libc::c_int as size_t)
            } else {
                malloc(9 as libc::c_int as libc::c_ulong)
            }) as *mut libc::c_char;
            if ((*rport).gvalue).is_null() {
                return -(4 as libc::c_int);
            }
            snprintf(
                (*rport).gvalue,
                8 as libc::c_int as libc::c_ulong,
                b"%i\0" as *const u8 as *const libc::c_char,
                port,
            );
        }
    }
    if 0 as libc::c_int == strcmp((*via).host, ip_addr) {
        return 0 as libc::c_int;
    }
    osip_uri_param_add(
        &mut (*via).via_params,
        osip_strdup(b"received\0" as *const u8 as *const libc::c_char),
        osip_strdup(ip_addr),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_reason(
    mut replycode: libc::c_int,
) -> *const libc::c_char {
    static mut reasons1xx: [code_to_reason; 6] = [
        {
            let mut init = code_to_reason {
                code: 100 as libc::c_int,
                reason: b"Trying\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 180 as libc::c_int,
                reason: b"Ringing\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 181 as libc::c_int,
                reason: b"Call Is Being Forwarded\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 182 as libc::c_int,
                reason: b"Queued\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 183 as libc::c_int,
                reason: b"Session Progress\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 199 as libc::c_int,
                reason: b"Early Dialog Terminated\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    static mut reasons2xx: [code_to_reason; 3] = [
        {
            let mut init = code_to_reason {
                code: 200 as libc::c_int,
                reason: b"OK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 202 as libc::c_int,
                reason: b"Accepted\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 204 as libc::c_int,
                reason: b"No Notification\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    static mut reasons3xx: [code_to_reason; 5] = [
        {
            let mut init = code_to_reason {
                code: 300 as libc::c_int,
                reason: b"Multiple Choices\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 301 as libc::c_int,
                reason: b"Moved Permanently\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 302 as libc::c_int,
                reason: b"Moved Temporarily\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 305 as libc::c_int,
                reason: b"Use Proxy\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 380 as libc::c_int,
                reason: b"Alternative Service\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    static mut reasons4xx: [code_to_reason; 47] = [
        {
            let mut init = code_to_reason {
                code: 400 as libc::c_int,
                reason: b"Bad Request\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 401 as libc::c_int,
                reason: b"Unauthorized\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 402 as libc::c_int,
                reason: b"Payment Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 403 as libc::c_int,
                reason: b"Forbidden\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 404 as libc::c_int,
                reason: b"Not Found\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 405 as libc::c_int,
                reason: b"Method Not Allowed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 406 as libc::c_int,
                reason: b"Not Acceptable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 407 as libc::c_int,
                reason: b"Proxy Authentication Required\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 408 as libc::c_int,
                reason: b"Request Timeout\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 409 as libc::c_int,
                reason: b"Conflict\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 410 as libc::c_int,
                reason: b"Gone\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 411 as libc::c_int,
                reason: b"Length Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 412 as libc::c_int,
                reason: b"Conditional Request Failed\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 413 as libc::c_int,
                reason: b"Request Entity Too Large\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 414 as libc::c_int,
                reason: b"Request-URI Too Long\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 415 as libc::c_int,
                reason: b"Unsupported Media Type\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 416 as libc::c_int,
                reason: b"Unsupported URI Scheme\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 417 as libc::c_int,
                reason: b"Unknown Resource-Priority\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 420 as libc::c_int,
                reason: b"Bad Extension\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 421 as libc::c_int,
                reason: b"Extension Required\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 422 as libc::c_int,
                reason: b"Session Interval Too Small\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 423 as libc::c_int,
                reason: b"Interval Too Brief\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 424 as libc::c_int,
                reason: b"Bad Location Information\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 428 as libc::c_int,
                reason: b"Use Identity Header\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 429 as libc::c_int,
                reason: b"Provide Referrer Identity\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 430 as libc::c_int,
                reason: b"Flow Failed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 433 as libc::c_int,
                reason: b"Anonymity Disallowed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 436 as libc::c_int,
                reason: b"Bad Identity Info\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 437 as libc::c_int,
                reason: b"Unsupported Credential\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 438 as libc::c_int,
                reason: b"Invalid Identity Header\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 439 as libc::c_int,
                reason: b"First Hop Lacks Outbound Support\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 440 as libc::c_int,
                reason: b"Max-Breadth Exceeded\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 469 as libc::c_int,
                reason: b"Bad Info Package\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 470 as libc::c_int,
                reason: b"Consent Needed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 480 as libc::c_int,
                reason: b"Temporarily Unavailable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 481 as libc::c_int,
                reason: b"Call/Transaction Does Not Exist\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 482 as libc::c_int,
                reason: b"Loop Detected\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 483 as libc::c_int,
                reason: b"Too Many Hops\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 484 as libc::c_int,
                reason: b"Address Incomplete\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 485 as libc::c_int,
                reason: b"Ambiguous\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 486 as libc::c_int,
                reason: b"Busy Here\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 487 as libc::c_int,
                reason: b"Request Terminated\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 488 as libc::c_int,
                reason: b"Not Acceptable Here\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 489 as libc::c_int,
                reason: b"Bad Event\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 491 as libc::c_int,
                reason: b"Request Pending\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 493 as libc::c_int,
                reason: b"Undecipherable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 494 as libc::c_int,
                reason: b"Security Agreement Required\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
    ];
    static mut reasons5xx: [code_to_reason; 8] = [
        {
            let mut init = code_to_reason {
                code: 500 as libc::c_int,
                reason: b"Server Internal Error\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 501 as libc::c_int,
                reason: b"Not Implemented\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 502 as libc::c_int,
                reason: b"Bad Gateway\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 503 as libc::c_int,
                reason: b"Service Unavailable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 504 as libc::c_int,
                reason: b"Server Time-out\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 505 as libc::c_int,
                reason: b"Version Not Supported\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 513 as libc::c_int,
                reason: b"Message Too Large\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 580 as libc::c_int,
                reason: b"Precondition Failure\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    static mut reasons6xx: [code_to_reason; 6] = [
        {
            let mut init = code_to_reason {
                code: 600 as libc::c_int,
                reason: b"Busy Everywhere\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 603 as libc::c_int,
                reason: b"Decline\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 604 as libc::c_int,
                reason: b"Does Not Exist Anywhere\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 606 as libc::c_int,
                reason: b"Not Acceptable\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 607 as libc::c_int,
                reason: b"Unwanted\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = code_to_reason {
                code: 687 as libc::c_int,
                reason: b"Dialog Terminated\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    let mut reasons: *const code_to_reason = 0 as *const code_to_reason;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    match replycode / 100 as libc::c_int {
        1 => {
            reasons = reasons1xx.as_ptr();
            len = (::std::mem::size_of::<[code_to_reason; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<code_to_reason>() as libc::c_ulong)
                as libc::c_int;
        }
        2 => {
            reasons = reasons2xx.as_ptr();
            len = (::std::mem::size_of::<[code_to_reason; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<code_to_reason>() as libc::c_ulong)
                as libc::c_int;
        }
        3 => {
            reasons = reasons3xx.as_ptr();
            len = (::std::mem::size_of::<[code_to_reason; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<code_to_reason>() as libc::c_ulong)
                as libc::c_int;
        }
        4 => {
            reasons = reasons4xx.as_ptr();
            len = (::std::mem::size_of::<[code_to_reason; 47]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<code_to_reason>() as libc::c_ulong)
                as libc::c_int;
        }
        5 => {
            reasons = reasons5xx.as_ptr();
            len = (::std::mem::size_of::<[code_to_reason; 8]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<code_to_reason>() as libc::c_ulong)
                as libc::c_int;
        }
        6 => {
            reasons = reasons6xx.as_ptr();
            len = (::std::mem::size_of::<[code_to_reason; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<code_to_reason>() as libc::c_ulong)
                as libc::c_int;
        }
        _ => return 0 as *const libc::c_char,
    }
    i = 0 as libc::c_int;
    while i < len {
        if (*reasons.offset(i as isize)).code == replycode {
            return (*reasons.offset(i as isize)).reason;
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
