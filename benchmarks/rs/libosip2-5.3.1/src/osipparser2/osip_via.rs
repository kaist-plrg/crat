use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_atoi(number: *const libc::c_char) -> libc::c_int;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_clone(
        src: *const osip_list_t,
        dst: *mut osip_list_t,
        clone_func: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut *mut libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
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
    fn osip_uri_param_clone(
        url_param: *const osip_uri_param_t,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_uri_param_freelist(url_params: *mut osip_list_t);
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
pub unsafe extern "C" fn osip_message_set_via(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    i = osip_via_init(&mut via);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_via_parse(via, hvalue);
    if i != 0 as libc::c_int {
        osip_via_free(via);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).vias, via as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_append_via(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut via: *mut osip_via_t = 0 as *mut osip_via_t;
    let mut i: libc::c_int = 0;
    i = osip_via_init(&mut via);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_via_parse(via, hvalue);
    if i != 0 as libc::c_int {
        osip_via_free(via);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).vias, via as *mut libc::c_void, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_via(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_via_t,
) -> libc::c_int {
    *dest = 0 as *mut osip_via_t;
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    if osip_list_size(&(*sip).vias) <= pos {
        return -(1 as libc::c_int);
    }
    *dest = osip_list_get(&(*sip).vias, pos) as *mut osip_via_t;
    return pos;
}
pub unsafe extern "C" fn osip_via_init(mut via: *mut *mut osip_via_t) -> libc::c_int {
    *via = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_via_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_via_t>() as libc::c_ulong)
    }) as *mut osip_via_t;
    if (*via).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *via as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_via_t>() as libc::c_ulong,
    );
    osip_list_init(&mut (**via).via_params);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_via_free(mut via: *mut osip_via_t) {
    if via.is_null() {
        return;
    }
    if !((*via).version).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*via).version as *mut libc::c_void);
        } else {
            free((*via).version as *mut libc::c_void);
        }
    }
    if !((*via).protocol).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*via).protocol as *mut libc::c_void);
        } else {
            free((*via).protocol as *mut libc::c_void);
        }
    }
    if !((*via).host).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*via).host as *mut libc::c_void);
        } else {
            free((*via).host as *mut libc::c_void);
        }
    }
    if !((*via).port).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*via).port as *mut libc::c_void);
        } else {
            free((*via).port as *mut libc::c_void);
        }
    }
    if !((*via).comment).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*via).comment as *mut libc::c_void);
        } else {
            free((*via).comment as *mut libc::c_void);
        }
    }
    osip_uri_param_freelist(&mut (*via).via_params);
    if !via.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(via as *mut libc::c_void);
        } else {
            free(via as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_via_parse(
    mut via: *mut osip_via_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut version: *const libc::c_char = 0 as *const libc::c_char;
    let mut protocol: *const libc::c_char = 0 as *const libc::c_char;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut ipv6host: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: *const libc::c_char = 0 as *const libc::c_char;
    let mut via_params: *const libc::c_char = 0 as *const libc::c_char;
    let mut comment: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if hvalue.is_null() {
        return -(2 as libc::c_int);
    }
    version = strchr(hvalue, '/' as i32);
    if version.is_null() {
        return -(5 as libc::c_int);
    }
    protocol = strchr(version.offset(1 as libc::c_int as isize), '/' as i32);
    if protocol.is_null() {
        return -(5 as libc::c_int);
    }
    if (protocol.offset_from(version) as libc::c_long) < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*via)
        .version = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(protocol.offset_from(version) as libc::c_long as size_t)
    } else {
        malloc(protocol.offset_from(version) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*via).version).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*via).version,
        version.offset(1 as libc::c_int as isize),
        (protocol.offset_from(version) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t,
    );
    host = strchr(protocol.offset(1 as libc::c_int as isize), ' ' as i32);
    if host.is_null() {
        return -(5 as libc::c_int);
    }
    if host == protocol.offset(1 as libc::c_int as isize) {
        while 0 as libc::c_int
            == strncmp(
                host,
                b" \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            )
        {
            host = host.offset(1);
            host;
            if strlen(host) <= 1 as libc::c_int as libc::c_ulong {
                return -(5 as libc::c_int);
            }
        }
        host = strchr(host.offset(1 as libc::c_int as isize), ' ' as i32);
        if host.is_null() {
            return -(5 as libc::c_int);
        }
    }
    if (host.offset_from(protocol) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return -(5 as libc::c_int);
    }
    (*via)
        .protocol = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(host.offset_from(protocol) as libc::c_long as size_t)
    } else {
        malloc(host.offset_from(protocol) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*via).protocol).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*via).protocol,
        protocol.offset(1 as libc::c_int as isize),
        (host.offset_from(protocol) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    comment = strchr(host, '(' as i32);
    if !comment.is_null() {
        let mut end_comment: *mut libc::c_char = 0 as *mut libc::c_char;
        end_comment = strchr(host, ')' as i32);
        if end_comment.is_null() {
            return -(5 as libc::c_int);
        }
        if (end_comment.offset_from(comment) as libc::c_long)
            < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        (*via)
            .comment = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(end_comment.offset_from(comment) as libc::c_long as size_t)
        } else {
            malloc(end_comment.offset_from(comment) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if ((*via).comment).is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            (*via).comment,
            comment.offset(1 as libc::c_int as isize),
            (end_comment.offset_from(comment) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        comment = comment.offset(-1);
        comment;
    } else {
        comment = host.offset(strlen(host) as isize);
    }
    via_params = strchr(host, ';' as i32);
    if !via_params.is_null() && via_params < comment {
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        if (comment.offset_from(via_params) as libc::c_long
            + 1 as libc::c_int as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        tmp = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (comment.offset_from(via_params) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            )
        } else {
            malloc(
                (comment.offset_from(via_params) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        if tmp.is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            tmp,
            via_params,
            comment.offset_from(via_params) as libc::c_long as size_t,
        );
        i = __osip_generic_param_parseall(&mut (*via).via_params, tmp);
        if i != 0 as libc::c_int {
            if !tmp.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp as *mut libc::c_void);
                } else {
                    free(tmp as *mut libc::c_void);
                }
            }
            return i;
        }
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
    }
    if via_params.is_null() {
        via_params = comment;
    }
    ipv6host = strchr(host, '[' as i32);
    if !ipv6host.is_null() && ipv6host < via_params {
        port = strchr(ipv6host, ']' as i32);
        if port.is_null() || port > via_params {
            return -(5 as libc::c_int);
        }
        if (port.offset_from(ipv6host) as libc::c_long)
            < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        (*via)
            .host = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(port.offset_from(ipv6host) as libc::c_long as size_t)
        } else {
            malloc(port.offset_from(ipv6host) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if ((*via).host).is_null() {
            return -(4 as libc::c_int);
        }
        osip_clrncpy(
            (*via).host,
            ipv6host.offset(1 as libc::c_int as isize),
            (port.offset_from(ipv6host) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        port = strchr(port, ':' as i32);
    } else {
        port = strchr(host, ':' as i32);
        ipv6host = 0 as *const libc::c_char;
    }
    if !port.is_null() && port < via_params {
        if (via_params.offset_from(port) as libc::c_long)
            < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        (*via)
            .port = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(via_params.offset_from(port) as libc::c_long as size_t)
        } else {
            malloc(via_params.offset_from(port) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if ((*via).port).is_null() {
            return -(4 as libc::c_int);
        }
        osip_clrncpy(
            (*via).port,
            port.offset(1 as libc::c_int as isize),
            (via_params.offset_from(port) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        if osip_atoi((*via).port) < 0 as libc::c_int {
            return -(5 as libc::c_int);
        }
    } else {
        port = via_params;
    }
    if !ipv6host.is_null() {
        return 0 as libc::c_int;
    }
    if (port.offset_from(host) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return -(5 as libc::c_int);
    }
    (*via)
        .host = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(port.offset_from(host) as libc::c_long as size_t)
    } else {
        malloc(port.offset_from(host) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*via).host).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*via).host,
        host.offset(1 as libc::c_int as isize),
        (port.offset_from(host) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_via_to_str(
    mut via: *const osip_via_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut plen: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    *dest = 0 as *mut libc::c_char;
    if via.is_null() || ((*via).host).is_null() || ((*via).version).is_null()
        || ((*via).protocol).is_null()
    {
        return -(2 as libc::c_int);
    }
    len = (strlen((*via).version))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen((*via).protocol))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    len = len
        .wrapping_add(strlen((*via).host))
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !((*via).port).is_null() {
        len = len
            .wrapping_add(strlen((*via).port))
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
    }
    buf = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if buf.is_null() {
        return -(4 as libc::c_int);
    }
    if !(strchr((*via).host, ':' as i32)).is_null() {
        if ((*via).port).is_null() {
            sprintf(
                buf,
                b"SIP/%s/%s [%s]\0" as *const u8 as *const libc::c_char,
                (*via).version,
                (*via).protocol,
                (*via).host,
            );
        } else {
            sprintf(
                buf,
                b"SIP/%s/%s [%s]:%s\0" as *const u8 as *const libc::c_char,
                (*via).version,
                (*via).protocol,
                (*via).host,
                (*via).port,
            );
        }
    } else if ((*via).port).is_null() {
        sprintf(
            buf,
            b"SIP/%s/%s %s\0" as *const u8 as *const libc::c_char,
            (*via).version,
            (*via).protocol,
            (*via).host,
        );
    } else {
        sprintf(
            buf,
            b"SIP/%s/%s %s:%s\0" as *const u8 as *const libc::c_char,
            (*via).version,
            (*via).protocol,
            (*via).host,
            (*via).port,
        );
    }
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_generic_param_t = osip_list_get_first(
        &(*via).via_params,
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
            sprintf(tmp, b";%s\0" as *const u8 as *const libc::c_char, (*u_param).gname);
        } else {
            sprintf(
                tmp,
                b";%s=%s\0" as *const u8 as *const libc::c_char,
                (*u_param).gname,
                (*u_param).gvalue,
            );
        }
        u_param = osip_list_get_next(&mut it) as *mut osip_generic_param_t;
    }
    if !((*via).comment).is_null() {
        len = len
            .wrapping_add(strlen((*via).comment))
            .wrapping_add(4 as libc::c_int as libc::c_ulong);
        buf = (if osip_realloc_func.is_some() {
            osip_realloc_func.unwrap()(buf as *mut libc::c_void, len)
        } else {
            realloc(buf as *mut libc::c_void, len)
        }) as *mut libc::c_char;
        tmp = buf;
        tmp = tmp.offset(strlen(tmp) as isize);
        snprintf(
            tmp,
            len.wrapping_sub(tmp.offset_from(buf) as libc::c_long as libc::c_ulong),
            b" (%s)\0" as *const u8 as *const libc::c_char,
            (*via).comment,
        );
    }
    *dest = buf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_via_set_version(
    mut via: *mut osip_via_t,
    mut version: *mut libc::c_char,
) {
    (*via).version = version;
}
pub unsafe extern "C" fn osip_via_get_version(
    mut via: *mut osip_via_t,
) -> *mut libc::c_char {
    if via.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*via).version;
}
pub unsafe extern "C" fn osip_via_set_protocol(
    mut via: *mut osip_via_t,
    mut protocol: *mut libc::c_char,
) {
    (*via).protocol = protocol;
}
pub unsafe extern "C" fn osip_via_get_protocol(
    mut via: *mut osip_via_t,
) -> *mut libc::c_char {
    if via.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*via).protocol;
}
pub unsafe extern "C" fn osip_via_set_host(
    mut via: *mut osip_via_t,
    mut host: *mut libc::c_char,
) {
    (*via).host = host;
}
pub unsafe extern "C" fn osip_via_get_host(
    mut via: *mut osip_via_t,
) -> *mut libc::c_char {
    if via.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*via).host;
}
pub unsafe extern "C" fn osip_via_set_port(
    mut via: *mut osip_via_t,
    mut port: *mut libc::c_char,
) {
    (*via).port = port;
}
pub unsafe extern "C" fn osip_via_get_port(
    mut via: *mut osip_via_t,
) -> *mut libc::c_char {
    if via.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*via).port;
}
pub unsafe extern "C" fn osip_via_set_comment(
    mut via: *mut osip_via_t,
    mut comment: *mut libc::c_char,
) {
    (*via).comment = comment;
}
pub unsafe extern "C" fn osip_via_get_comment(
    mut via: *mut osip_via_t,
) -> *mut libc::c_char {
    if via.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*via).comment;
}
pub unsafe extern "C" fn osip_via_clone(
    mut via: *const osip_via_t,
    mut dest: *mut *mut osip_via_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut vi: *mut osip_via_t = 0 as *mut osip_via_t;
    *dest = 0 as *mut osip_via_t;
    if via.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*via).version).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*via).protocol).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*via).host).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_via_init(&mut vi);
    if i != 0 as libc::c_int {
        return i;
    }
    (*vi).version = osip_strdup((*via).version);
    if ((*vi).version).is_null() && !((*via).version).is_null() {
        osip_via_free(vi);
        return -(4 as libc::c_int);
    }
    (*vi).protocol = osip_strdup((*via).protocol);
    if ((*vi).protocol).is_null() && !((*via).protocol).is_null() {
        osip_via_free(vi);
        return -(4 as libc::c_int);
    }
    (*vi).host = osip_strdup((*via).host);
    if ((*vi).host).is_null() && !((*via).host).is_null() {
        osip_via_free(vi);
        return -(4 as libc::c_int);
    }
    if !((*via).port).is_null() {
        (*vi).port = osip_strdup((*via).port);
        if ((*vi).port).is_null() {
            osip_via_free(vi);
            return -(4 as libc::c_int);
        }
    }
    if !((*via).comment).is_null() {
        (*vi).comment = osip_strdup((*via).comment);
        if ((*vi).comment).is_null() {
            osip_via_free(vi);
            return -(4 as libc::c_int);
        }
    }
    i = osip_list_clone(
        &(*via).via_params,
        &mut (*vi).via_params,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_uri_param_t,
                    *mut *mut osip_uri_param_t,
                ) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                osip_uri_param_clone
                    as unsafe extern "C" fn(
                        *const osip_uri_param_t,
                        *mut *mut osip_uri_param_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_via_free(vi);
        return i;
    }
    *dest = vi;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_via_match(
    mut via1: *mut osip_via_t,
    mut via2: *mut osip_via_t,
) -> libc::c_int {
    let mut _via1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _via2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if via1.is_null() || via2.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_via_to_str(via1, &mut _via1);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_via_to_str(via2, &mut _via2);
    if i != 0 as libc::c_int {
        if !_via1.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(_via1 as *mut libc::c_void);
            } else {
                free(_via1 as *mut libc::c_void);
            }
        }
        return i;
    }
    i = strcmp(_via1, _via2);
    if !_via1.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(_via1 as *mut libc::c_void);
        } else {
            free(_via1 as *mut libc::c_void);
        }
    }
    if !_via2.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(_via2 as *mut libc::c_void);
        } else {
            free(_via2 as *mut libc::c_void);
        }
    }
    if i != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
