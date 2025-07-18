use ::libc;
extern "C" {
    fn osip_from_init(header: *mut *mut osip_from_t) -> libc::c_int;
    fn osip_from_free(header: *mut osip_from_t);
    fn osip_from_parse(
        header: *mut osip_from_t,
        hvalue: *const libc::c_char,
    ) -> libc::c_int;
    fn osip_from_to_str(
        header: *const osip_from_t,
        dest: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn osip_from_clone(
        header: *const osip_from_t,
        dest: *mut *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_from_tag_match(
        from1: *mut osip_from_t,
        from2: *mut osip_from_t,
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
pub unsafe extern "C" fn osip_to_init(mut to: *mut *mut osip_to_t) -> libc::c_int {
    return osip_from_init(to as *mut *mut osip_from_t);
}
pub unsafe extern "C" fn osip_message_set_to(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if !((*sip).to).is_null() {
        return -(5 as libc::c_int);
    }
    i = osip_to_init(&mut (*sip).to);
    if i != 0 as libc::c_int {
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    i = osip_to_parse((*sip).to, hvalue);
    if i != 0 as libc::c_int {
        osip_to_free((*sip).to);
        (*sip).to = 0 as *mut osip_to_t;
        return i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_to(
    mut sip: *const osip_message_t,
) -> *mut osip_to_t {
    return (*sip).to;
}
pub unsafe extern "C" fn osip_to_parse(
    mut to: *mut osip_to_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    return osip_from_parse(to as *mut osip_from_t, hvalue);
}
pub unsafe extern "C" fn osip_to_to_str(
    mut to: *const osip_to_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    return osip_from_to_str(to as *mut osip_from_t, dest);
}
pub unsafe extern "C" fn osip_to_free(mut to: *mut osip_to_t) {
    osip_from_free(to as *mut osip_from_t);
}
pub unsafe extern "C" fn osip_to_clone(
    mut to: *const osip_to_t,
    mut dest: *mut *mut osip_to_t,
) -> libc::c_int {
    return osip_from_clone(to as *mut osip_from_t, dest as *mut *mut osip_from_t);
}
pub unsafe extern "C" fn osip_to_tag_match(
    mut to1: *mut osip_to_t,
    mut to2: *mut osip_to_t,
) -> libc::c_int {
    return osip_from_tag_match(to1 as *mut osip_from_t, to2 as *mut osip_from_t);
}
