use ::libc;
extern "C" {
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
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    fn osip_content_type_init(header: *mut *mut osip_content_type_t) -> libc::c_int;
    fn osip_content_type_free(header: *mut osip_content_type_t);
    fn osip_content_type_parse(
        header: *mut osip_content_type_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type osip_accept_t = osip_content_type_t;
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
pub unsafe extern "C" fn osip_message_set_accept(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut accept: *mut osip_accept_t = 0 as *mut osip_accept_t;
    let mut i: libc::c_int = 0;
    i = osip_content_type_init(&mut accept);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_content_type_parse(accept, hvalue);
    if i != 0 as libc::c_int {
        osip_content_type_free(accept);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).accepts, accept as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_accept(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_accept_t,
) -> libc::c_int {
    let mut accept: *mut osip_accept_t = 0 as *mut osip_accept_t;
    *dest = 0 as *mut osip_accept_t;
    if osip_list_size(&(*sip).accepts) <= pos {
        return -(1 as libc::c_int);
    }
    accept = osip_list_get(&(*sip).accepts, pos) as *mut osip_accept_t;
    *dest = accept;
    return pos;
}
pub unsafe extern "C" fn osip_accept_to_str(
    mut accept: *const osip_accept_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    *dest = 0 as *mut libc::c_char;
    if accept.is_null() {
        return -(2 as libc::c_int);
    }
    if !((*accept).type_0).is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen((*accept).type_0)) as size_t
            as size_t;
    }
    if !((*accept).subtype).is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen((*accept).subtype)) as size_t
            as size_t;
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        buf = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(2 as libc::c_int as size_t)
        } else {
            malloc(2 as libc::c_int as libc::c_ulong)
        }) as *mut libc::c_char;
        if buf.is_null() {
            return -(4 as libc::c_int);
        }
        *buf.offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
        *buf.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        *dest = buf;
        return 0 as libc::c_int;
    }
    len = (len as libc::c_ulong)
        .wrapping_add(
            (4 as libc::c_int
                + 10 as libc::c_int * osip_list_size(&(*accept).gen_params))
                as libc::c_ulong,
        ) as size_t as size_t;
    buf = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if buf.is_null() {
        return -(4 as libc::c_int);
    }
    tmp = buf;
    sprintf(
        tmp,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        (*accept).type_0,
        (*accept).subtype,
    );
    tmp = tmp.offset(strlen(tmp) as isize);
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_generic_param_t = osip_list_get_first(
        &(*accept).gen_params,
        &mut it,
    ) as *mut osip_generic_param_t;
    while !u_param.is_null() {
        let mut tmp_len: size_t = 0;
        if ((*u_param).gvalue).is_null() {
            if !buf.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(buf as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        tmp_len = (strlen(buf))
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*u_param).gname))
            .wrapping_add(strlen((*u_param).gvalue))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if len < tmp_len {
            buf = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(buf as *mut libc::c_void, tmp_len)
            } else {
                realloc(buf as *mut libc::c_void, tmp_len)
            }) as *mut libc::c_char;
            len = tmp_len;
            tmp = buf.offset(strlen(buf) as isize);
        }
        snprintf(
            tmp,
            len.wrapping_sub(tmp.offset_from(buf) as libc::c_long as libc::c_ulong),
            b"; %s=%s\0" as *const u8 as *const libc::c_char,
            (*u_param).gname,
            (*u_param).gvalue,
        );
        tmp = tmp.offset(strlen(tmp) as isize);
        u_param = osip_list_get_next(&mut it) as *mut osip_generic_param_t;
    }
    *dest = buf;
    return 0 as libc::c_int;
}
