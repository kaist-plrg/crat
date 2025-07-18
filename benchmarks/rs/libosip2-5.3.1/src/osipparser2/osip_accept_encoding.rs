use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
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
    fn osip_uri_param_freelist(url_params: *mut osip_list_t);
    fn osip_uri_param_clone(
        url_param: *const osip_uri_param_t,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn __osip_generic_param_parseall(
        gen_params: *mut osip_list_t,
        params: *const libc::c_char,
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
pub struct osip_accept_encoding {
    pub element: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_accept_encoding_t = osip_accept_encoding;
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
pub unsafe extern "C" fn osip_message_set_accept_encoding(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut accept_encoding: *mut osip_accept_encoding_t = 0
        as *mut osip_accept_encoding_t;
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    i = osip_accept_encoding_init(&mut accept_encoding);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_accept_encoding_parse(accept_encoding, hvalue);
    if i != 0 as libc::c_int {
        osip_accept_encoding_free(accept_encoding);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(
        &mut (*sip).accept_encodings,
        accept_encoding as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_accept_encoding(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_accept_encoding_t,
) -> libc::c_int {
    let mut accept_encoding: *mut osip_accept_encoding_t = 0
        as *mut osip_accept_encoding_t;
    *dest = 0 as *mut osip_accept_encoding_t;
    if osip_list_size(&(*sip).accept_encodings) <= pos {
        return -(1 as libc::c_int);
    }
    accept_encoding = osip_list_get(&(*sip).accept_encodings, pos)
        as *mut osip_accept_encoding_t;
    *dest = accept_encoding;
    return pos;
}
pub unsafe extern "C" fn osip_accept_encoding_init(
    mut accept_encoding: *mut *mut osip_accept_encoding_t,
) -> libc::c_int {
    *accept_encoding = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_accept_encoding_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_accept_encoding_t>() as libc::c_ulong)
    }) as *mut osip_accept_encoding_t;
    if (*accept_encoding).is_null() {
        return -(4 as libc::c_int);
    }
    (**accept_encoding).element = 0 as *mut libc::c_char;
    osip_list_init(&mut (**accept_encoding).gen_params);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_accept_encoding_parse(
    mut accept_encoding: *mut osip_accept_encoding_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut osip_accept_encoding_params: *const libc::c_char = 0 as *const libc::c_char;
    osip_accept_encoding_params = strchr(hvalue, ';' as i32);
    if !osip_accept_encoding_params.is_null() {
        i = __osip_generic_param_parseall(
            &mut (*accept_encoding).gen_params,
            osip_accept_encoding_params,
        );
        if i != 0 as libc::c_int {
            return i;
        }
    } else {
        osip_accept_encoding_params = hvalue.offset(strlen(hvalue) as isize);
    }
    if (osip_accept_encoding_params.offset_from(hvalue) as libc::c_long
        + 1 as libc::c_int as libc::c_long) < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*accept_encoding)
        .element = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (osip_accept_encoding_params.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (osip_accept_encoding_params.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*accept_encoding).element).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*accept_encoding).element,
        hvalue,
        osip_accept_encoding_params.offset_from(hvalue) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_accept_encoding_to_str(
    mut accept_encoding: *const osip_accept_encoding_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    *dest = 0 as *mut libc::c_char;
    if accept_encoding.is_null() || ((*accept_encoding).element).is_null() {
        return -(2 as libc::c_int);
    }
    len = (strlen((*accept_encoding).element))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    buf = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if buf.is_null() {
        return -(4 as libc::c_int);
    }
    sprintf(
        buf,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*accept_encoding).element,
    );
    let mut plen: size_t = 0;
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_generic_param_t = osip_list_get_first(
        &(*accept_encoding).gen_params,
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
pub unsafe extern "C" fn osip_accept_encoding_free(
    mut accept_encoding: *mut osip_accept_encoding_t,
) {
    if accept_encoding.is_null() {
        return;
    }
    if !((*accept_encoding).element).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*accept_encoding).element as *mut libc::c_void);
        } else {
            free((*accept_encoding).element as *mut libc::c_void);
        }
    }
    osip_uri_param_freelist(&mut (*accept_encoding).gen_params);
    (*accept_encoding).element = 0 as *mut libc::c_char;
    if !accept_encoding.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(accept_encoding as *mut libc::c_void);
        } else {
            free(accept_encoding as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_accept_encoding_clone(
    mut ctt: *const osip_accept_encoding_t,
    mut dest: *mut *mut osip_accept_encoding_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ct: *mut osip_accept_encoding_t = 0 as *mut osip_accept_encoding_t;
    *dest = 0 as *mut osip_accept_encoding_t;
    if ctt.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*ctt).element).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_accept_encoding_init(&mut ct);
    if i != 0 as libc::c_int {
        return i;
    }
    (*ct).element = osip_strdup((*ctt).element);
    if ((*ct).element).is_null() {
        osip_accept_encoding_free(ct);
        return -(4 as libc::c_int);
    }
    let mut dest_param: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_generic_param_t = osip_list_get_first(
        &(*ctt).gen_params,
        &mut it,
    ) as *mut osip_generic_param_t;
    while !u_param.is_null() {
        i = osip_uri_param_clone(u_param, &mut dest_param);
        if i != 0 as libc::c_int {
            osip_accept_encoding_free(ct);
            return i;
        }
        osip_list_add(
            &mut (*ct).gen_params,
            dest_param as *mut libc::c_void,
            -(1 as libc::c_int),
        );
        u_param = osip_list_get_next(&mut it) as *mut osip_generic_param_t;
    }
    *dest = ct;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_accept_encoding_get_element(
    mut ae: *const osip_accept_encoding_t,
) -> *mut libc::c_char {
    return (*ae).element;
}
pub unsafe extern "C" fn osip_accept_encoding_set_element(
    mut ae: *mut osip_accept_encoding_t,
    mut element: *mut libc::c_char,
) {
    (*ae).element = element;
}
