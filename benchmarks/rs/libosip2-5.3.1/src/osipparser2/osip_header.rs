use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_remove(li: *mut osip_list_t, pos: libc::c_int) -> libc::c_int;
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
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub unsafe extern "C" fn osip_message_set_header(
    mut sip: *mut osip_message_t,
    mut hname: *const libc::c_char,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut h: *mut osip_header_t = 0 as *mut osip_header_t;
    let mut i: libc::c_int = 0;
    if sip.is_null() || hname.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_header_init(&mut h);
    if i != 0 as libc::c_int {
        return i;
    }
    (*h)
        .hname = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()((strlen(hname)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc((strlen(hname)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if ((*h).hname).is_null() {
        osip_header_free(h);
        return -(4 as libc::c_int);
    }
    osip_clrncpy((*h).hname, hname, strlen(hname));
    if !hvalue.is_null() {
        (*h)
            .hvalue = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (strlen(hvalue)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc((strlen(hvalue)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        }) as *mut libc::c_char;
        if ((*h).hvalue).is_null() {
            osip_header_free(h);
            return -(4 as libc::c_int);
        }
        osip_clrncpy((*h).hvalue, hvalue, strlen(hvalue));
    } else {
        (*h).hvalue = 0 as *mut libc::c_char;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).headers, h as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_replace_header(
    mut sip: *mut osip_message_t,
    mut hname: *const libc::c_char,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut h: *mut osip_header_t = 0 as *mut osip_header_t;
    let mut oldh: *mut osip_header_t = 0 as *mut osip_header_t;
    let mut i: libc::c_int = 0;
    let mut oldpos: libc::c_int = -(1 as libc::c_int);
    if sip.is_null() || hname.is_null() {
        return -(2 as libc::c_int);
    }
    oldpos = osip_message_header_get_byname(sip, hname, 0 as libc::c_int, &mut oldh);
    i = osip_header_init(&mut h);
    if i != 0 as libc::c_int {
        return i;
    }
    (*h)
        .hname = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()((strlen(hname)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc((strlen(hname)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if ((*h).hname).is_null() {
        osip_header_free(h);
        return -(4 as libc::c_int);
    }
    osip_clrncpy((*h).hname, hname, strlen(hname));
    if !hvalue.is_null() {
        (*h)
            .hvalue = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (strlen(hvalue)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc((strlen(hvalue)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        }) as *mut libc::c_char;
        if ((*h).hvalue).is_null() {
            osip_header_free(h);
            return -(4 as libc::c_int);
        }
        osip_clrncpy((*h).hvalue, hvalue, strlen(hvalue));
    } else {
        (*h).hvalue = 0 as *mut libc::c_char;
    }
    if oldpos != -(1 as libc::c_int) {
        osip_list_remove(&mut (*sip).headers, oldpos);
        osip_header_free(oldh);
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).headers, h as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_topheader(
    mut sip: *mut osip_message_t,
    mut hname: *const libc::c_char,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut h: *mut osip_header_t = 0 as *mut osip_header_t;
    let mut i: libc::c_int = 0;
    if sip.is_null() || hname.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_header_init(&mut h);
    if i != 0 as libc::c_int {
        return i;
    }
    (*h)
        .hname = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()((strlen(hname)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc((strlen(hname)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if ((*h).hname).is_null() {
        osip_header_free(h);
        return -(4 as libc::c_int);
    }
    osip_clrncpy((*h).hname, hname, strlen(hname));
    if !hvalue.is_null() {
        (*h)
            .hvalue = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (strlen(hvalue)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc((strlen(hvalue)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        }) as *mut libc::c_char;
        if ((*h).hvalue).is_null() {
            osip_header_free(h);
            return -(4 as libc::c_int);
        }
        osip_clrncpy((*h).hvalue, hvalue, strlen(hvalue));
    } else {
        (*h).hvalue = 0 as *mut libc::c_char;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(&mut (*sip).headers, h as *mut libc::c_void, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_header(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_header_t,
) -> libc::c_int {
    *dest = 0 as *mut osip_header_t;
    if osip_list_size(&(*sip).headers) <= pos {
        return -(1 as libc::c_int);
    }
    *dest = osip_list_get(&(*sip).headers, pos) as *mut osip_header_t;
    return pos;
}
pub unsafe extern "C" fn osip_message_header_get_byname(
    mut sip: *const osip_message_t,
    mut hname: *const libc::c_char,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_header_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut osip_header_t = 0 as *mut osip_header_t;
    *dest = 0 as *mut osip_header_t;
    i = pos;
    if osip_list_size(&(*sip).headers) <= pos {
        return -(1 as libc::c_int);
    }
    while osip_list_size(&(*sip).headers) > i {
        tmp = osip_list_get(&(*sip).headers, i) as *mut osip_header_t;
        if osip_strcasecmp((*tmp).hname, hname) == 0 as libc::c_int {
            *dest = tmp;
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn osip_header_init(
    mut header: *mut *mut osip_header_t,
) -> libc::c_int {
    *header = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_header_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_header_t>() as libc::c_ulong)
    }) as *mut osip_header_t;
    if (*header).is_null() {
        return -(4 as libc::c_int);
    }
    (**header).hname = 0 as *mut libc::c_char;
    (**header).hvalue = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_header_free(mut header: *mut osip_header_t) {
    if header.is_null() {
        return;
    }
    if !((*header).hname).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*header).hname as *mut libc::c_void);
        } else {
            free((*header).hname as *mut libc::c_void);
        }
    }
    if !((*header).hvalue).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*header).hvalue as *mut libc::c_void);
        } else {
            free((*header).hvalue as *mut libc::c_void);
        }
    }
    (*header).hname = 0 as *mut libc::c_char;
    (*header).hvalue = 0 as *mut libc::c_char;
    if !header.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(header as *mut libc::c_void);
        } else {
            free(header as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_header_to_str(
    mut header: *const osip_header_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut hlen: size_t = 0;
    *dest = 0 as *mut libc::c_char;
    if header.is_null() || ((*header).hname).is_null() {
        return -(2 as libc::c_int);
    }
    len = 0 as libc::c_int as size_t;
    hlen = strlen((*header).hname);
    if !((*header).hvalue).is_null() {
        len = strlen((*header).hvalue);
    }
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            hlen.wrapping_add(len).wrapping_add(3 as libc::c_int as libc::c_ulong),
        )
    } else {
        malloc(hlen.wrapping_add(len).wrapping_add(3 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    if !((*header).hvalue).is_null() {
        snprintf(
            *dest,
            hlen.wrapping_add(len).wrapping_add(3 as libc::c_int as libc::c_ulong),
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            (*header).hname,
            (*header).hvalue,
        );
    } else {
        snprintf(
            *dest,
            hlen.wrapping_add(len).wrapping_add(3 as libc::c_int as libc::c_ulong),
            b"%s: \0" as *const u8 as *const libc::c_char,
            (*header).hname,
        );
    }
    if **dest.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
        && **dest.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
    {
        **dest
            .offset(
                0 as libc::c_int as isize,
            ) = (**dest.offset(0 as libc::c_int as isize) as libc::c_int
            - 32 as libc::c_int) as libc::c_char;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_header_get_name(
    mut header: *const osip_header_t,
) -> *mut libc::c_char {
    if header.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*header).hname;
}
pub unsafe extern "C" fn osip_header_set_name(
    mut header: *mut osip_header_t,
    mut name: *mut libc::c_char,
) {
    (*header).hname = name;
}
pub unsafe extern "C" fn osip_header_get_value(
    mut header: *const osip_header_t,
) -> *mut libc::c_char {
    if header.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*header).hvalue;
}
pub unsafe extern "C" fn osip_header_set_value(
    mut header: *mut osip_header_t,
    mut value: *mut libc::c_char,
) {
    (*header).hvalue = value;
}
pub unsafe extern "C" fn osip_header_clone(
    mut header: *const osip_header_t,
    mut dest: *mut *mut osip_header_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut he: *mut osip_header_t = 0 as *mut osip_header_t;
    *dest = 0 as *mut osip_header_t;
    if header.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*header).hname).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_header_init(&mut he);
    if i != 0 as libc::c_int {
        return i;
    }
    (*he).hname = osip_strdup((*header).hname);
    if ((*he).hname).is_null() {
        osip_header_free(he);
        return -(4 as libc::c_int);
    }
    if !((*header).hvalue).is_null() {
        (*he).hvalue = osip_strdup((*header).hvalue);
        if ((*he).hvalue).is_null() {
            osip_header_free(he);
            return -(4 as libc::c_int);
        }
    }
    *dest = he;
    return 0 as libc::c_int;
}
