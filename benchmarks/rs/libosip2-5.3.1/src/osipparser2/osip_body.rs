use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_str_append(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_special_free(
        li: *mut osip_list_t,
        free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_strn_append(
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
    fn osip_header_init(header: *mut *mut osip_header_t) -> libc::c_int;
    fn osip_header_free(header: *mut osip_header_t);
    fn osip_header_to_str(
        header: *const osip_header_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_header_clone(
        header: *const osip_header_t,
        dest: *mut *mut osip_header_t,
    ) -> libc::c_int;
    fn osip_content_type_init(header: *mut *mut osip_content_type_t) -> libc::c_int;
    fn osip_content_type_free(header: *mut osip_content_type_t);
    fn osip_content_type_parse(
        header: *mut osip_content_type_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_content_type_to_str(
        header: *const osip_content_type_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_content_type_clone(
        header: *const osip_content_type_t,
        dest: *mut *mut osip_content_type_t,
    ) -> libc::c_int;
    fn __osip_find_next_crlf(
        start_of_header: *const libc::c_char,
        end_of_header: *mut *const libc::c_char,
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
pub unsafe extern "C" fn osip_body_init(mut body: *mut *mut osip_body_t) -> libc::c_int {
    *body = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_body_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_body_t>() as libc::c_ulong)
    }) as *mut osip_body_t;
    if (*body).is_null() {
        return -(4 as libc::c_int);
    }
    (**body).body = 0 as *mut libc::c_char;
    (**body).content_type = 0 as *mut osip_content_type_t;
    (**body).length = 0 as libc::c_int as size_t;
    (**body)
        .headers = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_list_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_list_t>() as libc::c_ulong)
    }) as *mut osip_list_t;
    if ((**body).headers).is_null() {
        if !(*body).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(*body as *mut libc::c_void);
            } else {
                free(*body as *mut libc::c_void);
            }
        }
        *body = 0 as *mut osip_body_t;
        return -(4 as libc::c_int);
    }
    osip_list_init((**body).headers);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_body(
    mut sip: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut body: *mut osip_body_t = 0 as *mut osip_body_t;
    let mut i: libc::c_int = 0;
    i = osip_body_init(&mut body);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_body_parse(body, buf, length);
    if i != 0 as libc::c_int {
        osip_body_free(body);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).bodies, body as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_body_clone(
    mut body: *const osip_body_t,
    mut dest: *mut *mut osip_body_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut copy: *mut osip_body_t = 0 as *mut osip_body_t;
    if body.is_null() || (*body).length <= 0 as libc::c_int as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    i = osip_body_init(&mut copy);
    if i != 0 as libc::c_int {
        return i;
    }
    (*copy)
        .body = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(((*body).length).wrapping_add(2 as libc::c_int as libc::c_ulong))
    } else {
        malloc(((*body).length).wrapping_add(2 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if ((*copy).body).is_null() {
        osip_body_free(copy);
        return -(4 as libc::c_int);
    }
    (*copy).length = (*body).length;
    memcpy(
        (*copy).body as *mut libc::c_void,
        (*body).body as *const libc::c_void,
        (*body).length,
    );
    *((*copy).body).offset((*body).length as isize) = '\0' as i32 as libc::c_char;
    if !((*body).content_type).is_null() {
        i = osip_content_type_clone((*body).content_type, &mut (*copy).content_type);
        if i != 0 as libc::c_int {
            osip_body_free(copy);
            return i;
        }
    }
    i = osip_list_clone(
        (*body).headers,
        (*copy).headers,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_header_t,
                    *mut *mut osip_header_t,
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
                osip_header_clone
                    as unsafe extern "C" fn(
                        *const osip_header_t,
                        *mut *mut osip_header_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_body_free(copy);
        return i;
    }
    *dest = copy;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_body(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_body_t,
) -> libc::c_int {
    let mut body: *mut osip_body_t = 0 as *mut osip_body_t;
    *dest = 0 as *mut osip_body_t;
    if osip_list_size(&(*sip).bodies) <= pos {
        return -(1 as libc::c_int);
    }
    body = osip_list_get(&(*sip).bodies, pos) as *mut osip_body_t;
    *dest = body;
    return pos;
}
pub unsafe extern "C" fn osip_body_set_contenttype(
    mut body: *mut osip_body_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if body.is_null() {
        return -(2 as libc::c_int);
    }
    if hvalue.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_content_type_init(&mut (*body).content_type);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_content_type_parse((*body).content_type, hvalue);
    if i != 0 as libc::c_int {
        osip_content_type_free((*body).content_type);
        (*body).content_type = 0 as *mut osip_content_type_t;
        return i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_body_set_header(
    mut body: *mut osip_body_t,
    mut hname: *const libc::c_char,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut h: *mut osip_header_t = 0 as *mut osip_header_t;
    let mut i: libc::c_int = 0;
    if body.is_null() {
        return -(2 as libc::c_int);
    }
    if hname.is_null() {
        return -(2 as libc::c_int);
    }
    if hvalue.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_header_init(&mut h);
    if i != 0 as libc::c_int {
        return i;
    }
    (*h).hname = osip_strdup(hname);
    if ((*h).hname).is_null() {
        osip_header_free(h);
        return -(4 as libc::c_int);
    }
    (*h).hvalue = osip_strdup(hvalue);
    if ((*h).hvalue).is_null() {
        osip_header_free(h);
        return -(4 as libc::c_int);
    }
    osip_list_add((*body).headers, h as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_body_mime(
    mut sip: *mut osip_message_t,
    mut buf: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut body: *mut osip_body_t = 0 as *mut osip_body_t;
    let mut i: libc::c_int = 0;
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_body_init(&mut body);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_body_parse_mime(body, buf, length);
    if i != 0 as libc::c_int {
        osip_body_free(body);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).bodies, body as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
unsafe extern "C" fn osip_body_parse_header(
    mut body: *mut osip_body_t,
    mut start_of_osip_body_header: *const libc::c_char,
    mut next_body: *mut *const libc::c_char,
) -> libc::c_int {
    let mut start_of_line: *const libc::c_char = 0 as *const libc::c_char;
    let mut end_of_line: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon_index: *const libc::c_char = 0 as *const libc::c_char;
    let mut hname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    *next_body = 0 as *const libc::c_char;
    start_of_line = start_of_osip_body_header;
    loop {
        i = __osip_find_next_crlf(start_of_line, &mut end_of_line);
        if i == -(2 as libc::c_int) {
            return -(5 as libc::c_int)
        } else if i != 0 as libc::c_int {
            return i
        }
        colon_index = strchr(start_of_line, ':' as i32);
        if colon_index.is_null() {
            return -(5 as libc::c_int);
        }
        if (colon_index.offset_from(start_of_line) as libc::c_long
            + 1 as libc::c_int as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        hname = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (colon_index.offset_from(start_of_line) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            )
        } else {
            malloc(
                (colon_index.offset_from(start_of_line) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        if hname.is_null() {
            return -(4 as libc::c_int);
        }
        osip_clrncpy(
            hname,
            start_of_line,
            colon_index.offset_from(start_of_line) as libc::c_long as size_t,
        );
        if (end_of_line.offset(-(2 as libc::c_int as isize)).offset_from(colon_index)
            as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            if !hname.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(hname as *mut libc::c_void);
                } else {
                    free(hname as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        hvalue = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                end_of_line.offset(-(2 as libc::c_int as isize)).offset_from(colon_index)
                    as libc::c_long as size_t,
            )
        } else {
            malloc(
                end_of_line.offset(-(2 as libc::c_int as isize)).offset_from(colon_index)
                    as libc::c_long as libc::c_ulong,
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
            (end_of_line.offset(-(2 as libc::c_int as isize)).offset_from(colon_index)
                as libc::c_long - 1 as libc::c_int as libc::c_long) as size_t,
        );
        if osip_strncasecmp(
            hname,
            b"content-type\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        ) == 0 as libc::c_int
        {
            i = osip_body_set_contenttype(body, hvalue);
        } else {
            i = osip_body_set_header(body, hname, hvalue);
        }
        if !hname.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(hname as *mut libc::c_void);
            } else {
                free(hname as *mut libc::c_void);
            }
        }
        if !hvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(hvalue as *mut libc::c_void);
            } else {
                free(hvalue as *mut libc::c_void);
            }
        }
        if i != 0 as libc::c_int {
            return i;
        }
        if strncmp(
            end_of_line,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                end_of_line,
                b"\n\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || strncmp(
                end_of_line,
                b"\r\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            *next_body = end_of_line;
            return 0 as libc::c_int;
        }
        start_of_line = end_of_line;
    };
}
pub unsafe extern "C" fn osip_body_parse(
    mut body: *mut osip_body_t,
    mut start_of_body: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    if body.is_null() {
        return -(2 as libc::c_int);
    }
    if start_of_body.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*body).headers).is_null() {
        return -(2 as libc::c_int);
    }
    (*body)
        .body = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if ((*body).body).is_null() {
        return -(4 as libc::c_int);
    }
    memcpy(
        (*body).body as *mut libc::c_void,
        start_of_body as *const libc::c_void,
        length,
    );
    *((*body).body).offset(length as isize) = '\0' as i32 as libc::c_char;
    (*body).length = length;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_body_parse_mime(
    mut body: *mut osip_body_t,
    mut start_of_body: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut end_of_osip_body_header: *const libc::c_char = 0 as *const libc::c_char;
    let mut start_of_osip_body_header: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if body.is_null() {
        return -(2 as libc::c_int);
    }
    if start_of_body.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*body).headers).is_null() {
        return -(2 as libc::c_int);
    }
    start_of_osip_body_header = start_of_body;
    i = osip_body_parse_header(
        body,
        start_of_osip_body_header,
        &mut end_of_osip_body_header,
    );
    if i != 0 as libc::c_int {
        return i;
    }
    start_of_osip_body_header = end_of_osip_body_header;
    if strncmp(
        start_of_osip_body_header,
        b"\r\n\0\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        start_of_osip_body_header = start_of_osip_body_header
            .offset(2 as libc::c_int as isize);
    } else if strncmp(
        start_of_osip_body_header,
        b"\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncmp(
            start_of_osip_body_header,
            b"\r\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        start_of_osip_body_header = start_of_osip_body_header
            .offset(1 as libc::c_int as isize);
    } else {
        return -(5 as libc::c_int)
    }
    end_of_osip_body_header = start_of_body.offset(length as isize);
    if end_of_osip_body_header.offset_from(start_of_osip_body_header) as libc::c_long
        <= 0 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*body)
        .body = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (end_of_osip_body_header.offset_from(start_of_osip_body_header)
                as libc::c_long + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (end_of_osip_body_header.offset_from(start_of_osip_body_header)
                as libc::c_long + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*body).body).is_null() {
        return -(4 as libc::c_int);
    }
    memcpy(
        (*body).body as *mut libc::c_void,
        start_of_osip_body_header as *const libc::c_void,
        end_of_osip_body_header.offset_from(start_of_osip_body_header) as libc::c_long
            as libc::c_ulong,
    );
    (*body)
        .length = end_of_osip_body_header.offset_from(start_of_osip_body_header)
        as libc::c_long as size_t;
    *((*body).body).offset((*body).length as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_body_to_str(
    mut body: *const osip_body_t,
    mut dest: *mut *mut libc::c_char,
    mut str_length: *mut size_t,
) -> libc::c_int {
    let mut tmp_body: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut length: size_t = 0;
    if !dest.is_null() {
        *dest = 0 as *mut libc::c_char;
    }
    if !str_length.is_null() {
        *str_length = 0 as libc::c_int as size_t;
    }
    if body.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*body).body).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*body).headers).is_null() {
        return -(2 as libc::c_int);
    }
    if (*body).length <= 0 as libc::c_int as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    length = (15 as libc::c_int as libc::c_ulong)
        .wrapping_add((*body).length)
        .wrapping_add(
            (osip_list_size((*body).headers) * 40 as libc::c_int) as libc::c_ulong,
        );
    tmp_body = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(length)
    } else {
        malloc(length)
    }) as *mut libc::c_char;
    if tmp_body.is_null() {
        return -(4 as libc::c_int);
    }
    ptr = tmp_body;
    if !((*body).content_type).is_null() {
        tmp_body = osip_strn_append(
            tmp_body,
            b"content-type: \0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as size_t,
        );
        i = osip_content_type_to_str((*body).content_type, &mut tmp);
        if i != 0 as libc::c_int {
            if !ptr.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(ptr as *mut libc::c_void);
                } else {
                    free(ptr as *mut libc::c_void);
                }
            }
            return i;
        }
        if length
            < (tmp_body.offset_from(ptr) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
        {
            let mut len: size_t = 0;
            len = tmp_body.offset_from(ptr) as libc::c_long as size_t;
            length = length
                .wrapping_add(strlen(tmp))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            ptr = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(ptr as *mut libc::c_void, length)
            } else {
                realloc(ptr as *mut libc::c_void, length)
            }) as *mut libc::c_char;
            tmp_body = ptr.offset(len as isize);
        }
        tmp_body = osip_str_append(tmp_body, tmp);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        tmp_body = osip_strn_append(
            tmp_body,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
    }
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut header: *mut osip_header_t = osip_list_get_first((*body).headers, &mut it)
        as *mut osip_header_t;
    while !header.is_null() {
        i = osip_header_to_str(header, &mut tmp);
        if i != 0 as libc::c_int {
            if !ptr.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(ptr as *mut libc::c_void);
                } else {
                    free(ptr as *mut libc::c_void);
                }
            }
            return i;
        }
        if length
            < (tmp_body.offset_from(ptr) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
        {
            let mut len_0: size_t = 0;
            len_0 = tmp_body.offset_from(ptr) as libc::c_long as size_t;
            length = length
                .wrapping_add(strlen(tmp))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            ptr = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(ptr as *mut libc::c_void, length)
            } else {
                realloc(ptr as *mut libc::c_void, length)
            }) as *mut libc::c_char;
            tmp_body = ptr.offset(len_0 as isize);
        }
        tmp_body = osip_str_append(tmp_body, tmp);
        if !tmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp as *mut libc::c_void);
            } else {
                free(tmp as *mut libc::c_void);
            }
        }
        tmp_body = osip_strn_append(
            tmp_body,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        header = osip_list_get_next(&mut it) as *mut osip_header_t;
    }
    if osip_list_size((*body).headers) > 0 as libc::c_int
        || !((*body).content_type).is_null()
    {
        if length
            < (tmp_body.offset_from(ptr) as libc::c_long
                + 3 as libc::c_int as libc::c_long) as size_t
        {
            let mut len_1: size_t = 0;
            len_1 = tmp_body.offset_from(ptr) as libc::c_long as size_t;
            length = length
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add((*body).length);
            ptr = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(ptr as *mut libc::c_void, length)
            } else {
                realloc(ptr as *mut libc::c_void, length)
            }) as *mut libc::c_char;
            tmp_body = ptr.offset(len_1 as isize);
        }
        tmp_body = osip_strn_append(
            tmp_body,
            b"\r\n\0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
    }
    if length
        < (tmp_body.offset_from(ptr) as libc::c_long as libc::c_ulong)
            .wrapping_add((*body).length)
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
    {
        let mut len_2: size_t = 0;
        len_2 = tmp_body.offset_from(ptr) as libc::c_long as size_t;
        length = length
            .wrapping_add((*body).length)
            .wrapping_add(4 as libc::c_int as libc::c_ulong);
        ptr = (if osip_realloc_func.is_some() {
            osip_realloc_func.unwrap()(ptr as *mut libc::c_void, length)
        } else {
            realloc(ptr as *mut libc::c_void, length)
        }) as *mut libc::c_char;
        tmp_body = ptr.offset(len_2 as isize);
    }
    memcpy(
        tmp_body as *mut libc::c_void,
        (*body).body as *const libc::c_void,
        (*body).length,
    );
    tmp_body = tmp_body.offset((*body).length as isize);
    if !str_length.is_null() {
        *str_length = tmp_body.offset_from(ptr) as libc::c_long as size_t;
    }
    *dest = ptr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_body_free(mut body: *mut osip_body_t) {
    if body.is_null() {
        return;
    }
    if !((*body).body).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*body).body as *mut libc::c_void);
        } else {
            free((*body).body as *mut libc::c_void);
        }
    }
    if !((*body).content_type).is_null() {
        osip_content_type_free((*body).content_type);
    }
    osip_list_special_free(
        (*body).headers,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_header_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_header_free as unsafe extern "C" fn(*mut osip_header_t) -> ())),
    );
    if !((*body).headers).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*body).headers as *mut libc::c_void);
        } else {
            free((*body).headers as *mut libc::c_void);
        }
    }
    if !body.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(body as *mut libc::c_void);
        } else {
            free(body as *mut libc::c_void);
        }
    }
}
