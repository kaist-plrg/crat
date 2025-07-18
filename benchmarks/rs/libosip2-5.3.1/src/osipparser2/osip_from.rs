use ::libc;
extern "C" {
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
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
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn __osip_quote_find(qstring: *const libc::c_char) -> *const libc::c_char;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_uri_param_clone(
        url_param: *const osip_uri_param_t,
        dest: *mut *mut osip_uri_param_t,
    ) -> libc::c_int;
    fn osip_uri_param_freelist(url_params: *mut osip_list_t);
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
    fn osip_uri_to_str(
        url: *const osip_uri_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_uri_clone(url: *const osip_uri_t, dest: *mut *mut osip_uri_t) -> libc::c_int;
    fn next_separator(
        ch: *const libc::c_char,
        separator_osip_to_find: libc::c_int,
        before_separator: libc::c_int,
    ) -> *const libc::c_char;
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
pub unsafe extern "C" fn osip_message_set_from(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if !((*sip).from).is_null() {
        return -(5 as libc::c_int);
    }
    i = osip_from_init(&mut (*sip).from);
    if i != 0 as libc::c_int {
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    i = osip_from_parse((*sip).from, hvalue);
    if i != 0 as libc::c_int {
        osip_from_free((*sip).from);
        (*sip).from = 0 as *mut osip_from_t;
        return i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_from(
    mut sip: *const osip_message_t,
) -> *mut osip_from_t {
    return (*sip).from;
}
pub unsafe extern "C" fn osip_from_init(mut from: *mut *mut osip_from_t) -> libc::c_int {
    *from = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_from_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_from_t>() as libc::c_ulong)
    }) as *mut osip_from_t;
    if (*from).is_null() {
        return -(4 as libc::c_int);
    }
    (**from).displayname = 0 as *mut libc::c_char;
    (**from).url = 0 as *mut osip_uri_t;
    osip_list_init(&mut (**from).gen_params);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_from_free(mut from: *mut osip_from_t) {
    if from.is_null() {
        return;
    }
    if !((*from).url).is_null() {
        osip_uri_free((*from).url);
    }
    if !((*from).displayname).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*from).displayname as *mut libc::c_void);
        } else {
            free((*from).displayname as *mut libc::c_void);
        }
    }
    osip_uri_param_freelist(&mut (*from).gen_params);
    if !from.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(from as *mut libc::c_void);
        } else {
            free(from as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_from_parse(
    mut from: *mut osip_from_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut displayname: *const libc::c_char = 0 as *const libc::c_char;
    let mut url: *const libc::c_char = 0 as *const libc::c_char;
    let mut url_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut gen_params: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if from.is_null() || hvalue.is_null() {
        return -(2 as libc::c_int);
    }
    ptr = hvalue;
    while *ptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
            ptr = ptr.offset(1);
            ptr;
        } else {
            if !(*ptr.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32) {
                break;
            }
            displayname = ptr;
            break;
        }
    }
    if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    if !displayname.is_null() {
        let mut second: *const libc::c_char = 0 as *const libc::c_char;
        second = __osip_quote_find(displayname.offset(1 as libc::c_int as isize));
        if second.is_null() {
            return -(5 as libc::c_int);
        }
        if second.offset_from(displayname) as libc::c_long
            + 2 as libc::c_int as libc::c_long >= 2 as libc::c_int as libc::c_long
        {
            (*from)
                .displayname = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(
                    (second.offset_from(displayname) as libc::c_long
                        + 2 as libc::c_int as libc::c_long) as size_t,
                )
            } else {
                malloc(
                    (second.offset_from(displayname) as libc::c_long
                        + 2 as libc::c_int as libc::c_long) as libc::c_ulong,
                )
            }) as *mut libc::c_char;
            if ((*from).displayname).is_null() {
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                (*from).displayname,
                displayname,
                (second.offset_from(displayname) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
        ptr = second.offset(1 as libc::c_int as isize);
        while *ptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                ptr = ptr.offset(1);
                ptr;
            } else {
                if !(*ptr.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32)
                {
                    break;
                }
                url = ptr;
                break;
            }
        }
        if url.is_null() {
            return -(5 as libc::c_int);
        }
        if *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            return -(5 as libc::c_int);
        }
    } else {
        let mut beg: *const libc::c_char = ptr;
        while *ptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                ptr = ptr.offset(1);
                ptr;
            } else if *ptr.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
                && *ptr.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
            {
                ptr = ptr.offset(1);
                ptr;
            } else if *ptr.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
                && *ptr.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
            {
                ptr = ptr.offset(1);
                ptr;
            } else if *ptr.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *ptr.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
            {
                ptr = ptr.offset(1);
                ptr;
            } else if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '`' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32
                || *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
            {
                ptr = ptr.offset(1);
                ptr;
            } else {
                url = ptr;
                break;
            }
        }
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            || url.is_null()
        {
            return -(5 as libc::c_int);
        }
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
            if *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                return -(5 as libc::c_int);
            }
            if url.offset_from(beg) as libc::c_long > 0 as libc::c_int as libc::c_long {
                (*from)
                    .displayname = (if osip_malloc_func.is_some() {
                    osip_malloc_func
                        .unwrap()(
                        (url.offset_from(beg) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t,
                    )
                } else {
                    malloc(
                        (url.offset_from(beg) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    )
                }) as *mut libc::c_char;
                if ((*from).displayname).is_null() {
                    return -(4 as libc::c_int);
                }
                osip_clrncpy(
                    (*from).displayname,
                    hvalue,
                    url.offset_from(beg) as libc::c_long as size_t,
                );
            }
        } else if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            url = beg;
        } else {
            url = beg;
        }
    }
    if *url.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        url = url.offset(1);
        url;
        ptr = url;
        url_end = strchr(ptr, '>' as i32);
        if url_end.is_null() {
            return -(5 as libc::c_int);
        }
        url_end = url_end.offset(-1);
        url_end;
        gen_params = strchr(url_end, ';' as i32);
    }
    if url_end.is_null() {
        gen_params = strchr(url, ';' as i32);
        if !gen_params.is_null() {
            url_end = gen_params.offset(-(1 as libc::c_int as isize));
        } else {
            url_end = url.offset(strlen(url) as isize);
        }
    }
    if !gen_params.is_null() {
        i = __osip_generic_param_parseall(&mut (*from).gen_params, gen_params);
        if i != 0 as libc::c_int {
            return i;
        }
    }
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (url_end.offset_from(url) as libc::c_long + 2 as libc::c_int as libc::c_long)
        < 7 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    i = osip_uri_init(&mut (*from).url);
    if i != 0 as libc::c_int {
        return i;
    }
    tmp = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (url_end.offset_from(url) as libc::c_long + 2 as libc::c_int as libc::c_long)
                as size_t,
        )
    } else {
        malloc(
            (url_end.offset_from(url) as libc::c_long + 2 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if tmp.is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        tmp,
        url,
        (url_end.offset_from(url) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    i = osip_uri_parse((*from).url, tmp);
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
pub unsafe extern "C" fn osip_from_to_str(
    mut from: *const osip_from_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: size_t = 0;
    *dest = 0 as *mut libc::c_char;
    if from.is_null() || ((*from).url).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_uri_to_str((*from).url, &mut url);
    if i != 0 as libc::c_int {
        return i;
    }
    if ((*from).displayname).is_null() {
        len = (strlen(url)).wrapping_add(5 as libc::c_int as libc::c_ulong);
    } else {
        len = (strlen(url))
            .wrapping_add(strlen((*from).displayname))
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
    if !((*from).displayname).is_null() {
        sprintf(
            buf,
            b"%s <%s>\0" as *const u8 as *const libc::c_char,
            (*from).displayname,
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
        &(*from).gen_params,
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
pub unsafe extern "C" fn osip_from_get_displayname(
    mut from: *mut osip_from_t,
) -> *mut libc::c_char {
    if from.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*from).displayname;
}
pub unsafe extern "C" fn osip_from_set_displayname(
    mut from: *mut osip_from_t,
    mut displayname: *mut libc::c_char,
) {
    (*from).displayname = displayname;
}
pub unsafe extern "C" fn osip_from_get_url(
    mut from: *mut osip_from_t,
) -> *mut osip_uri_t {
    if from.is_null() {
        return 0 as *mut osip_uri_t;
    }
    return (*from).url;
}
pub unsafe extern "C" fn osip_from_set_url(
    mut from: *mut osip_from_t,
    mut url: *mut osip_uri_t,
) {
    (*from).url = url;
}
pub unsafe extern "C" fn osip_from_param_get(
    mut from: *mut osip_from_t,
    mut pos: libc::c_int,
    mut fparam: *mut *mut osip_generic_param_t,
) -> libc::c_int {
    *fparam = 0 as *mut osip_generic_param_t;
    if from.is_null() {
        return -(2 as libc::c_int);
    }
    if osip_list_size(&mut (*from).gen_params) <= pos {
        return -(1 as libc::c_int);
    }
    *fparam = osip_list_get(&mut (*from).gen_params, pos) as *mut osip_generic_param_t;
    return pos;
}
pub unsafe extern "C" fn osip_from_clone(
    mut from: *const osip_from_t,
    mut dest: *mut *mut osip_from_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut fr: *mut osip_from_t = 0 as *mut osip_from_t;
    *dest = 0 as *mut osip_from_t;
    if from.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_from_init(&mut fr);
    if i != 0 as libc::c_int {
        return i;
    }
    if !((*from).displayname).is_null() {
        (*fr).displayname = osip_strdup((*from).displayname);
        if ((*fr).displayname).is_null() {
            osip_from_free(fr);
            return -(4 as libc::c_int);
        }
    }
    if !((*from).url).is_null() {
        i = osip_uri_clone((*from).url, &mut (*fr).url);
        if i != 0 as libc::c_int {
            osip_from_free(fr);
            return i;
        }
    }
    i = osip_list_clone(
        &(*from).gen_params,
        &mut (*fr).gen_params,
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
        osip_from_free(fr);
        return i;
    }
    *dest = fr;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_from_compare(
    mut from1: *mut osip_from_t,
    mut from2: *mut osip_from_t,
) -> libc::c_int {
    let mut tag1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag2: *mut libc::c_char = 0 as *mut libc::c_char;
    if from1.is_null() || from2.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*from1).url).is_null() || ((*from2).url).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*(*from1).url).host).is_null() && ((*(*from2).url).host).is_null() {
        if ((*(*from1).url).string).is_null() || ((*(*from2).url).string).is_null() {
            return -(1 as libc::c_int);
        }
        if 0 as libc::c_int == strcmp((*(*from1).url).string, (*(*from2).url).string) {
            return 0 as libc::c_int;
        }
    }
    if ((*(*from1).url).host).is_null() || ((*(*from2).url).host).is_null() {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != strcmp((*(*from1).url).host, (*(*from2).url).host) {
        return -(1 as libc::c_int);
    }
    if !((*(*from1).url).username).is_null() && !((*(*from2).url).username).is_null() {
        if 0 as libc::c_int != strcmp((*(*from1).url).username, (*(*from2).url).username)
        {
            return -(1 as libc::c_int);
        }
    }
    tag1 = 0 as *mut libc::c_char;
    tag2 = 0 as *mut libc::c_char;
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_generic_param_t = osip_list_get_first(
        &mut (*from1).gen_params,
        &mut it,
    ) as *mut osip_generic_param_t;
    while !u_param.is_null() {
        if 0 as libc::c_int
            == strncmp(
                (*u_param).gname,
                b"tag\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            )
        {
            tag1 = (*u_param).gvalue;
            break;
        } else {
            u_param = osip_list_get_next(&mut it) as *mut osip_generic_param_t;
        }
    }
    let mut it_0: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param_0: *mut osip_generic_param_t = osip_list_get_first(
        &mut (*from2).gen_params,
        &mut it_0,
    ) as *mut osip_generic_param_t;
    while !u_param_0.is_null() {
        if 0 as libc::c_int
            == strncmp(
                (*u_param_0).gname,
                b"tag\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            )
        {
            tag2 = (*u_param_0).gvalue;
            break;
        } else {
            u_param_0 = osip_list_get_next(&mut it_0) as *mut osip_generic_param_t;
        }
    }
    if !tag1.is_null() && !tag2.is_null() {
        if 0 as libc::c_int != strcmp(tag1, tag2) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_generic_param_parseall(
    mut gen_params: *mut osip_list_t,
    mut params: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comma: *const libc::c_char = 0 as *const libc::c_char;
    let mut equal: *const libc::c_char = 0 as *const libc::c_char;
    let mut startquote: *const libc::c_char = 0 as *const libc::c_char;
    let mut endquote: *const libc::c_char = 0 as *const libc::c_char;
    equal = next_separator(
        params.offset(1 as libc::c_int as isize),
        '=' as i32,
        ';' as i32,
    );
    comma = strchr(params.offset(1 as libc::c_int as isize), ';' as i32);
    if !equal.is_null() {
        let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
        startquote = 0 as *const libc::c_char;
        tmp = equal.offset(1 as libc::c_int as isize);
        while *tmp.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
            tmp = tmp.offset(1);
            tmp;
        }
        if *tmp.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
            startquote = tmp;
        }
        if !startquote.is_null() && comma > startquote {
            comma = 0 as *const libc::c_char;
            endquote = __osip_quote_find(startquote.offset(1 as libc::c_int as isize));
            if !endquote.is_null() {
                comma = strchr(endquote, ';' as i32);
            }
        }
    }
    while !comma.is_null() {
        if equal.is_null() {
            equal = comma;
            pvalue = 0 as *mut libc::c_char;
        } else {
            let mut tmp_0: *const libc::c_char = 0 as *const libc::c_char;
            tmp_0 = equal.offset(1 as libc::c_int as isize);
            tmp_0 = tmp_0
                .offset(
                    strspn(tmp_0, b"\t \0" as *const u8 as *const libc::c_char) as isize,
                );
            pvalue = 0 as *mut libc::c_char;
            if *tmp_0 as libc::c_int != ',' as i32
                && *tmp_0 as libc::c_int != '\0' as i32
            {
                if (comma.offset_from(equal) as libc::c_long)
                    < 2 as libc::c_int as libc::c_long
                {
                    return -(5 as libc::c_int);
                }
                pvalue = (if osip_malloc_func.is_some() {
                    osip_malloc_func
                        .unwrap()(comma.offset_from(equal) as libc::c_long as size_t)
                } else {
                    malloc(comma.offset_from(equal) as libc::c_long as libc::c_ulong)
                }) as *mut libc::c_char;
                if pvalue.is_null() {
                    return -(4 as libc::c_int);
                }
                osip_strncpy(
                    pvalue,
                    equal.offset(1 as libc::c_int as isize),
                    (comma.offset_from(equal) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as size_t,
                );
            }
        }
        if (equal.offset_from(params) as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        pname = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(equal.offset_from(params) as libc::c_long as size_t)
        } else {
            malloc(equal.offset_from(params) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if pname.is_null() {
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            pname,
            params.offset(1 as libc::c_int as isize),
            (equal.offset_from(params) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        i = osip_uri_param_add(gen_params, pname, pvalue);
        if i != 0 as libc::c_int {
            if !pname.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pname as *mut libc::c_void);
                } else {
                    free(pname as *mut libc::c_void);
                }
            }
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        params = comma;
        equal = next_separator(
            params.offset(1 as libc::c_int as isize),
            '=' as i32,
            ';' as i32,
        );
        comma = strchr(params.offset(1 as libc::c_int as isize), ';' as i32);
        if !equal.is_null() {
            let mut tmp_1: *const libc::c_char = 0 as *const libc::c_char;
            startquote = 0 as *const libc::c_char;
            tmp_1 = equal.offset(1 as libc::c_int as isize);
            while *tmp_1.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                tmp_1 = tmp_1.offset(1);
                tmp_1;
            }
            if *tmp_1.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                startquote = tmp_1;
            }
            if !startquote.is_null() && comma > startquote {
                comma = 0 as *const libc::c_char;
                endquote = __osip_quote_find(
                    startquote.offset(1 as libc::c_int as isize),
                );
                if !endquote.is_null() {
                    comma = strchr(endquote, ';' as i32);
                }
            }
        }
    }
    comma = params.offset(strlen(params) as isize);
    if equal.is_null() {
        equal = comma;
        pvalue = 0 as *mut libc::c_char;
        if (equal.offset_from(params) as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return 0 as libc::c_int;
        }
    } else {
        let mut tmp_2: *const libc::c_char = 0 as *const libc::c_char;
        tmp_2 = equal.offset(1 as libc::c_int as isize);
        tmp_2 = tmp_2
            .offset(
                strspn(tmp_2, b"\t \0" as *const u8 as *const libc::c_char) as isize,
            );
        pvalue = 0 as *mut libc::c_char;
        if *tmp_2 as libc::c_int != ',' as i32 && *tmp_2 as libc::c_int != '\0' as i32 {
            if (comma.offset_from(equal) as libc::c_long)
                < 2 as libc::c_int as libc::c_long
            {
                return -(5 as libc::c_int);
            }
            pvalue = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(comma.offset_from(equal) as libc::c_long as size_t)
            } else {
                malloc(comma.offset_from(equal) as libc::c_long as libc::c_ulong)
            }) as *mut libc::c_char;
            if pvalue.is_null() {
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                pvalue,
                equal.offset(1 as libc::c_int as isize),
                (comma.offset_from(equal) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
    }
    if (equal.offset_from(params) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        if !pvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pvalue as *mut libc::c_void);
            } else {
                free(pvalue as *mut libc::c_void);
            }
        }
        return -(5 as libc::c_int);
    }
    pname = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(equal.offset_from(params) as libc::c_long as size_t)
    } else {
        malloc(equal.offset_from(params) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if pname.is_null() {
        if !pvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pvalue as *mut libc::c_void);
            } else {
                free(pvalue as *mut libc::c_void);
            }
        }
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        pname,
        params.offset(1 as libc::c_int as isize),
        (equal.offset_from(params) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    i = osip_uri_param_add(gen_params, pname, pvalue);
    if i != 0 as libc::c_int {
        if !pname.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pname as *mut libc::c_void);
            } else {
                free(pname as *mut libc::c_void);
            }
        }
        if !pvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pvalue as *mut libc::c_void);
            } else {
                free(pvalue as *mut libc::c_void);
            }
        }
        return -(4 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_generic_param_set_value(
    mut fparam: *mut osip_generic_param_t,
    mut value: *mut libc::c_char,
) {
    (*fparam).gvalue = value;
}
pub unsafe extern "C" fn osip_generic_param_get_name(
    mut fparam: *const osip_generic_param_t,
) -> *mut libc::c_char {
    if fparam.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*fparam).gname;
}
pub unsafe extern "C" fn osip_generic_param_set_name(
    mut fparam: *mut osip_generic_param_t,
    mut name: *mut libc::c_char,
) {
    (*fparam).gname = name;
}
pub unsafe extern "C" fn osip_generic_param_get_value(
    mut fparam: *const osip_generic_param_t,
) -> *mut libc::c_char {
    if fparam.is_null() {
        return 0 as *mut libc::c_char;
    }
    if ((*fparam).gname).is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*fparam).gvalue;
}
pub unsafe extern "C" fn osip_from_tag_match(
    mut from1: *mut osip_from_t,
    mut from2: *mut osip_from_t,
) -> libc::c_int {
    let mut tag_from1: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    let mut tag_from2: *mut osip_generic_param_t = 0 as *mut osip_generic_param_t;
    if from1.is_null() || from2.is_null() {
        return -(2 as libc::c_int);
    }
    osip_uri_param_get_byname(
        &mut (*from1).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag_from1,
    );
    osip_uri_param_get_byname(
        &mut (*from2).gen_params,
        b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tag_from2,
    );
    if tag_from1.is_null() && tag_from2.is_null() {
        return 0 as libc::c_int;
    }
    if !tag_from1.is_null() && tag_from2.is_null()
        || tag_from1.is_null() && !tag_from2.is_null()
    {
        return -(1 as libc::c_int);
    }
    if ((*tag_from1).gvalue).is_null() || ((*tag_from2).gvalue).is_null() {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != strcmp((*tag_from1).gvalue, (*tag_from2).gvalue) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
