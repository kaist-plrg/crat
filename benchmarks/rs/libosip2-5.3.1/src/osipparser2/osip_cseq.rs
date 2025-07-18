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
pub unsafe extern "C" fn osip_cseq_init(mut cseq: *mut *mut osip_cseq_t) -> libc::c_int {
    *cseq = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_cseq_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_cseq_t>() as libc::c_ulong)
    }) as *mut osip_cseq_t;
    if (*cseq).is_null() {
        return -(4 as libc::c_int);
    }
    (**cseq).method = 0 as *mut libc::c_char;
    (**cseq).number = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_cseq(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if !((*sip).cseq).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_cseq_init(&mut (*sip).cseq);
    if i != 0 as libc::c_int {
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    i = osip_cseq_parse((*sip).cseq, hvalue);
    if i != 0 as libc::c_int {
        osip_cseq_free((*sip).cseq);
        (*sip).cseq = 0 as *mut osip_cseq_t;
        return i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_cseq_parse(
    mut cseq: *mut osip_cseq_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut method: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if cseq.is_null() || hvalue.is_null() {
        return -(2 as libc::c_int);
    }
    (*cseq).number = 0 as *mut libc::c_char;
    (*cseq).method = 0 as *mut libc::c_char;
    method = strchr(hvalue, ' ' as i32);
    if method.is_null() {
        return -(5 as libc::c_int);
    }
    end = hvalue.offset(strlen(hvalue) as isize);
    if (method.offset_from(hvalue) as libc::c_long + 1 as libc::c_int as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*cseq)
        .number = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (method.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (method.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*cseq).number).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*cseq).number,
        hvalue,
        method.offset_from(hvalue) as libc::c_long as size_t,
    );
    if (end.offset_from(method) as libc::c_long + 1 as libc::c_int as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*cseq)
        .method = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (end.offset_from(method) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        )
    } else {
        malloc(
            (end.offset_from(method) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*cseq).method).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*cseq).method,
        method.offset(1 as libc::c_int as isize),
        end.offset_from(method) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_cseq(
    mut sip: *const osip_message_t,
) -> *mut osip_cseq_t {
    return (*sip).cseq;
}
pub unsafe extern "C" fn osip_cseq_get_number(
    mut cseq: *mut osip_cseq_t,
) -> *mut libc::c_char {
    return (*cseq).number;
}
pub unsafe extern "C" fn osip_cseq_get_method(
    mut cseq: *mut osip_cseq_t,
) -> *mut libc::c_char {
    return (*cseq).method;
}
pub unsafe extern "C" fn osip_cseq_set_number(
    mut cseq: *mut osip_cseq_t,
    mut number: *mut libc::c_char,
) {
    (*cseq).number = number;
}
pub unsafe extern "C" fn osip_cseq_set_method(
    mut cseq: *mut osip_cseq_t,
    mut method: *mut libc::c_char,
) {
    (*cseq).method = method;
}
pub unsafe extern "C" fn osip_cseq_to_str(
    mut cseq: *const osip_cseq_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    *dest = 0 as *mut libc::c_char;
    if cseq.is_null() || ((*cseq).number).is_null() || ((*cseq).method).is_null() {
        return -(2 as libc::c_int);
    }
    len = (strlen((*cseq).method))
        .wrapping_add(strlen((*cseq).number))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    snprintf(
        *dest,
        len,
        b"%s %s\0" as *const u8 as *const libc::c_char,
        (*cseq).number,
        (*cseq).method,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_cseq_free(mut cseq: *mut osip_cseq_t) {
    if cseq.is_null() {
        return;
    }
    if !((*cseq).method).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*cseq).method as *mut libc::c_void);
        } else {
            free((*cseq).method as *mut libc::c_void);
        }
    }
    if !((*cseq).number).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*cseq).number as *mut libc::c_void);
        } else {
            free((*cseq).number as *mut libc::c_void);
        }
    }
    if !cseq.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(cseq as *mut libc::c_void);
        } else {
            free(cseq as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_cseq_clone(
    mut cseq: *const osip_cseq_t,
    mut dest: *mut *mut osip_cseq_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cs: *mut osip_cseq_t = 0 as *mut osip_cseq_t;
    *dest = 0 as *mut osip_cseq_t;
    if cseq.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*cseq).method).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*cseq).number).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_cseq_init(&mut cs);
    if i != 0 as libc::c_int {
        osip_cseq_free(cs);
        return i;
    }
    (*cs).method = osip_strdup((*cseq).method);
    (*cs).number = osip_strdup((*cseq).number);
    *dest = cs;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_cseq_match(
    mut cseq1: *mut osip_cseq_t,
    mut cseq2: *mut osip_cseq_t,
) -> libc::c_int {
    if cseq1.is_null() || cseq2.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*cseq1).number).is_null() || ((*cseq2).number).is_null()
        || ((*cseq1).method).is_null() || ((*cseq2).method).is_null()
    {
        return -(2 as libc::c_int);
    }
    if 0 as libc::c_int == strcmp((*cseq1).number, (*cseq2).number) {
        if 0 as libc::c_int
            == strcmp((*cseq2).method, b"INVITE\0" as *const u8 as *const libc::c_char)
            || 0 as libc::c_int
                == strcmp((*cseq2).method, b"ACK\0" as *const u8 as *const libc::c_char)
        {
            if 0 as libc::c_int
                == strcmp(
                    (*cseq1).method,
                    b"INVITE\0" as *const u8 as *const libc::c_char,
                )
                || 0 as libc::c_int
                    == strcmp(
                        (*cseq1).method,
                        b"ACK\0" as *const u8 as *const libc::c_char,
                    )
            {
                return 0 as libc::c_int;
            }
        } else if 0 as libc::c_int == strcmp((*cseq1).method, (*cseq2).method) {
            return 0 as libc::c_int
        }
    }
    return -(1 as libc::c_int);
}
