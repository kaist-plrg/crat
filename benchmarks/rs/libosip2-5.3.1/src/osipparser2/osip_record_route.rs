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
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_uri_to_str(
        url: *const osip_uri_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_from_init(header: *mut *mut osip_from_t) -> libc::c_int;
    fn osip_from_free(header: *mut osip_from_t);
    fn osip_from_parse(
        header: *mut osip_from_t,
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
pub type osip_record_route_t = osip_from_t;
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
pub unsafe extern "C" fn osip_record_route_init(
    mut record_route: *mut *mut osip_record_route_t,
) -> libc::c_int {
    return osip_from_init(record_route as *mut *mut osip_from_t);
}
pub unsafe extern "C" fn osip_message_set_record_route(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut record_route: *mut osip_record_route_t = 0 as *mut osip_record_route_t;
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    i = osip_record_route_init(&mut record_route);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_record_route_parse(record_route, hvalue);
    if i != 0 as libc::c_int {
        osip_record_route_free(record_route);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(
        &mut (*sip).record_routes,
        record_route as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_record_route(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_record_route_t,
) -> libc::c_int {
    let mut record_route: *mut osip_record_route_t = 0 as *mut osip_record_route_t;
    *dest = 0 as *mut osip_record_route_t;
    if osip_list_size(&(*sip).record_routes) <= pos {
        return -(1 as libc::c_int);
    }
    record_route = osip_list_get(&(*sip).record_routes, pos) as *mut osip_record_route_t;
    *dest = record_route;
    return pos;
}
pub unsafe extern "C" fn osip_record_route_parse(
    mut record_route: *mut osip_record_route_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    return osip_from_parse(record_route as *mut osip_from_t, hvalue);
}
pub unsafe extern "C" fn osip_record_route_to_str(
    mut record_route: *const osip_record_route_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: size_t = 0;
    *dest = 0 as *mut libc::c_char;
    if record_route.is_null() || ((*record_route).url).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_uri_to_str((*record_route).url, &mut url);
    if i != 0 as libc::c_int {
        return i;
    }
    if ((*record_route).displayname).is_null() {
        len = (strlen(url)).wrapping_add(5 as libc::c_int as libc::c_ulong);
    } else {
        len = (strlen(url))
            .wrapping_add(strlen((*record_route).displayname))
            .wrapping_add(5 as libc::c_int as libc::c_ulong);
    }
    buf = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if buf.is_null() {
        if !url.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(url as *mut libc::c_void);
            } else {
                free(url as *mut libc::c_void);
            }
        }
        return -(4 as libc::c_int);
    }
    if !((*record_route).displayname).is_null() {
        sprintf(
            buf,
            b"%s <%s>\0" as *const u8 as *const libc::c_char,
            (*record_route).displayname,
            url,
        );
    } else {
        sprintf(buf, b"<%s>\0" as *const u8 as *const libc::c_char, url);
    }
    if !url.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(url as *mut libc::c_void);
        } else {
            free(url as *mut libc::c_void);
        }
    }
    let mut plen: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_generic_param_t = osip_list_get_first(
        &(*record_route).gen_params,
        &mut it,
    ) as *mut osip_generic_param_t;
    while !u_param.is_null() {
        if ((*u_param).gvalue).is_null() {
            plen = (strlen((*u_param).gname))
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
        } else {
            plen = (strlen((*u_param).gname))
                .wrapping_add(strlen((*u_param).gvalue))
                .wrapping_add(3 as libc::c_int as libc::c_ulong);
        }
        len = len.wrapping_add(plen);
        buf = (if osip_realloc_func.is_some() {
            osip_realloc_func.unwrap()(buf as *mut libc::c_void, len)
        } else {
            realloc(buf as *mut libc::c_void, len)
        }) as *mut libc::c_char;
        tmp = buf;
        tmp = tmp.offset(strlen(tmp) as isize);
        if ((*u_param).gvalue).is_null() {
            snprintf(
                tmp,
                len.wrapping_sub(tmp.offset_from(buf) as libc::c_long as libc::c_ulong),
                b";%s\0" as *const u8 as *const libc::c_char,
                (*u_param).gname,
            );
        } else {
            snprintf(
                tmp,
                len.wrapping_sub(tmp.offset_from(buf) as libc::c_long as libc::c_ulong),
                b";%s=%s\0" as *const u8 as *const libc::c_char,
                (*u_param).gname,
                (*u_param).gvalue,
            );
        }
        u_param = osip_list_get_next(&mut it) as *mut osip_generic_param_t;
    }
    *dest = buf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_record_route_free(
    mut record_route: *mut osip_record_route_t,
) {
    osip_from_free(record_route as *mut osip_from_t);
}
