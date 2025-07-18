use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
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
pub unsafe extern "C" fn osip_content_length_init(
    mut cl: *mut *mut osip_content_length_t,
) -> libc::c_int {
    *cl = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_content_length_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_content_length_t>() as libc::c_ulong)
    }) as *mut osip_content_length_t;
    if (*cl).is_null() {
        return -(4 as libc::c_int);
    }
    (**cl).value = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_content_length(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if !((*sip).content_length).is_null() {
        return -(5 as libc::c_int);
    }
    i = osip_content_length_init(&mut (*sip).content_length);
    if i != 0 as libc::c_int {
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    i = osip_content_length_parse((*sip).content_length, hvalue);
    if i != 0 as libc::c_int {
        osip_content_length_free((*sip).content_length);
        (*sip).content_length = 0 as *mut osip_content_length_t;
        return i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_content_length_parse(
    mut content_length: *mut osip_content_length_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    if hvalue.is_null() {
        return -(2 as libc::c_int);
    }
    len = strlen(hvalue);
    if len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        < 2 as libc::c_int as libc::c_ulong
    {
        return -(5 as libc::c_int);
    }
    (*content_length)
        .value = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if ((*content_length).value).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy((*content_length).value, hvalue, len);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_content_length(
    mut sip: *const osip_message_t,
) -> *mut osip_content_length_t {
    return (*sip).content_length;
}
pub unsafe extern "C" fn osip_content_length_to_str(
    mut cl: *const osip_content_length_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    if cl.is_null() {
        return -(2 as libc::c_int);
    }
    *dest = osip_strdup((*cl).value);
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_content_length_free(
    mut content_length: *mut osip_content_length_t,
) {
    if content_length.is_null() {
        return;
    }
    if !((*content_length).value).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*content_length).value as *mut libc::c_void);
        } else {
            free((*content_length).value as *mut libc::c_void);
        }
    }
    if !content_length.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(content_length as *mut libc::c_void);
        } else {
            free(content_length as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_content_length_clone(
    mut ctl: *const osip_content_length_t,
    mut dest: *mut *mut osip_content_length_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cl: *mut osip_content_length_t = 0 as *mut osip_content_length_t;
    *dest = 0 as *mut osip_content_length_t;
    if ctl.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_content_length_init(&mut cl);
    if i != 0 as libc::c_int {
        return i;
    }
    if !((*ctl).value).is_null() {
        (*cl).value = osip_strdup((*ctl).value);
        if ((*cl).value).is_null() {
            osip_content_length_free(cl);
            return -(4 as libc::c_int);
        }
    }
    *dest = cl;
    return 0 as libc::c_int;
}
