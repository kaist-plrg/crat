use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
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
pub unsafe extern "C" fn osip_message_set_call_id(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if !((*sip).call_id).is_null() {
        return -(5 as libc::c_int);
    }
    i = osip_call_id_init(&mut (*sip).call_id);
    if i != 0 as libc::c_int {
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    i = osip_call_id_parse((*sip).call_id, hvalue);
    if i != 0 as libc::c_int {
        osip_call_id_free((*sip).call_id);
        (*sip).call_id = 0 as *mut osip_call_id_t;
        return i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_call_id(
    mut sip: *const osip_message_t,
) -> *mut osip_call_id_t {
    return (*sip).call_id;
}
pub unsafe extern "C" fn osip_call_id_init(
    mut callid: *mut *mut osip_call_id_t,
) -> libc::c_int {
    *callid = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_call_id_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_call_id_t>() as libc::c_ulong)
    }) as *mut osip_call_id_t;
    if (*callid).is_null() {
        return -(4 as libc::c_int);
    }
    (**callid).number = 0 as *mut libc::c_char;
    (**callid).host = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_call_id_free(mut callid: *mut osip_call_id_t) {
    if callid.is_null() {
        return;
    }
    if !((*callid).number).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*callid).number as *mut libc::c_void);
        } else {
            free((*callid).number as *mut libc::c_void);
        }
    }
    if !((*callid).host).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*callid).host as *mut libc::c_void);
        } else {
            free((*callid).host as *mut libc::c_void);
        }
    }
    (*callid).number = 0 as *mut libc::c_char;
    (*callid).host = 0 as *mut libc::c_char;
    if !callid.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(callid as *mut libc::c_void);
        } else {
            free(callid as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_call_id_parse(
    mut callid: *mut osip_call_id_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    (*callid).number = 0 as *mut libc::c_char;
    (*callid).host = 0 as *mut libc::c_char;
    host = strchr(hvalue, '@' as i32);
    end = hvalue.offset(strlen(hvalue) as isize);
    if host.is_null() {
        host = end;
    } else {
        if (end.offset_from(host) as libc::c_long + 1 as libc::c_int as libc::c_long)
            < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        (*callid)
            .host = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(end.offset_from(host) as libc::c_long as size_t)
        } else {
            malloc(end.offset_from(host) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if ((*callid).host).is_null() {
            return -(4 as libc::c_int);
        }
        osip_clrncpy(
            (*callid).host,
            host.offset(1 as libc::c_int as isize),
            (end.offset_from(host) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as size_t,
        );
    }
    if (host.offset_from(hvalue) as libc::c_long + 1 as libc::c_int as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*callid)
        .number = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (host.offset_from(hvalue) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        )
    } else {
        malloc(
            (host.offset_from(hvalue) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*callid).number).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*callid).number,
        hvalue,
        host.offset_from(hvalue) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_call_id_to_str(
    mut callid: *const osip_call_id_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    *dest = 0 as *mut libc::c_char;
    if callid.is_null() || ((*callid).number).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*callid).host).is_null() {
        *dest = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (strlen((*callid).number))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc(
                (strlen((*callid).number))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        }) as *mut libc::c_char;
        if (*dest).is_null() {
            return -(4 as libc::c_int);
        }
        sprintf(*dest, b"%s\0" as *const u8 as *const libc::c_char, (*callid).number);
    } else {
        *dest = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (strlen((*callid).number))
                    .wrapping_add(strlen((*callid).host))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc(
                (strlen((*callid).number))
                    .wrapping_add(strlen((*callid).host))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            )
        }) as *mut libc::c_char;
        if (*dest).is_null() {
            return -(4 as libc::c_int);
        }
        sprintf(
            *dest,
            b"%s@%s\0" as *const u8 as *const libc::c_char,
            (*callid).number,
            (*callid).host,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_call_id_get_number(
    mut callid: *mut osip_call_id_t,
) -> *mut libc::c_char {
    if callid.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*callid).number;
}
pub unsafe extern "C" fn osip_call_id_get_host(
    mut callid: *mut osip_call_id_t,
) -> *mut libc::c_char {
    if callid.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*callid).host;
}
pub unsafe extern "C" fn osip_call_id_set_number(
    mut callid: *mut osip_call_id_t,
    mut number: *mut libc::c_char,
) {
    (*callid).number = number;
}
pub unsafe extern "C" fn osip_call_id_set_host(
    mut callid: *mut osip_call_id_t,
    mut host: *mut libc::c_char,
) {
    (*callid).host = host;
}
pub unsafe extern "C" fn osip_call_id_clone(
    mut callid: *const osip_call_id_t,
    mut dest: *mut *mut osip_call_id_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ci: *mut osip_call_id_t = 0 as *mut osip_call_id_t;
    *dest = 0 as *mut osip_call_id_t;
    if callid.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*callid).number).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_call_id_init(&mut ci);
    if i != 0 as libc::c_int {
        return i;
    }
    (*ci).number = osip_strdup((*callid).number);
    if !((*callid).host).is_null() {
        (*ci).host = osip_strdup((*callid).host);
    }
    *dest = ci;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_call_id_match(
    mut callid1: *mut osip_call_id_t,
    mut callid2: *mut osip_call_id_t,
) -> libc::c_int {
    if callid1.is_null() || callid2.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*callid1).number).is_null() || ((*callid2).number).is_null() {
        return -(2 as libc::c_int);
    }
    if 0 as libc::c_int != strcmp((*callid1).number, (*callid2).number) {
        return -(1 as libc::c_int);
    }
    if ((*callid1).host).is_null() && ((*callid2).host).is_null() {
        return 0 as libc::c_int;
    }
    if ((*callid1).host).is_null() && !((*callid2).host).is_null() {
        return -(1 as libc::c_int);
    }
    if !((*callid1).host).is_null() && ((*callid2).host).is_null() {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != strcmp((*callid1).host, (*callid2).host) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
