use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
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
pub type osip_contact_t = osip_from_t;
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
pub unsafe extern "C" fn osip_message_set_contact(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut contact: *mut osip_contact_t = 0 as *mut osip_contact_t;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    i = osip_contact_init(&mut contact);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_contact_parse(contact, hvalue);
    if i != 0 as libc::c_int {
        osip_contact_free(contact);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(
        &mut (*sip).contacts,
        contact as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_contact_parse(
    mut contact: *mut osip_contact_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    if contact.is_null() {
        return -(2 as libc::c_int);
    }
    if strncmp(
        hvalue,
        b"*\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*contact).displayname = osip_strdup(hvalue);
        if ((*contact).displayname).is_null() {
            return -(4 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    return osip_from_parse(contact as *mut osip_from_t, hvalue);
}
pub unsafe extern "C" fn osip_contact_init(
    mut contact: *mut *mut osip_contact_t,
) -> libc::c_int {
    return osip_from_init(contact as *mut *mut osip_from_t);
}
pub unsafe extern "C" fn osip_message_get_contact(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_contact_t,
) -> libc::c_int {
    *dest = 0 as *mut osip_contact_t;
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    if osip_list_size(&(*sip).contacts) <= pos {
        return -(1 as libc::c_int);
    }
    *dest = osip_list_get(&(*sip).contacts, pos) as *mut osip_contact_t;
    return pos;
}
pub unsafe extern "C" fn osip_contact_to_str(
    mut contact: *const osip_contact_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    if contact.is_null() {
        return -(2 as libc::c_int);
    }
    if !((*contact).displayname).is_null() {
        if strncmp(
            (*contact).displayname,
            b"*\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            *dest = osip_strdup(b"*\0" as *const u8 as *const libc::c_char);
            if (*dest).is_null() {
                return -(4 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
    }
    return osip_from_to_str(contact as *mut osip_from_t, dest);
}
pub unsafe extern "C" fn osip_contact_free(mut contact: *mut osip_contact_t) {
    osip_from_free(contact as *mut osip_from_t);
}
pub unsafe extern "C" fn osip_contact_clone(
    mut contact: *const osip_contact_t,
    mut dest: *mut *mut osip_contact_t,
) -> libc::c_int {
    return osip_from_clone(contact as *mut osip_from_t, dest as *mut *mut osip_from_t);
}
