use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_special_free(
        li: *mut osip_list_t,
        free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
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
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_authentication_info_free(header: *mut osip_authentication_info_t);
    fn osip_authentication_info_clone(
        header: *const osip_authentication_info_t,
        dest: *mut *mut osip_authentication_info_t,
    ) -> libc::c_int;
    fn osip_contact_free(header: *mut osip_contact_t);
    fn osip_uri_free(url: *mut osip_uri_t);
    fn osip_uri_clone(url: *const osip_uri_t, dest: *mut *mut osip_uri_t) -> libc::c_int;
    fn osip_header_free(header: *mut osip_header_t);
    fn osip_header_clone(
        header: *const osip_header_t,
        dest: *mut *mut osip_header_t,
    ) -> libc::c_int;
    fn osip_content_type_free(header: *mut osip_content_type_t);
    fn osip_content_type_clone(
        header: *const osip_content_type_t,
        dest: *mut *mut osip_content_type_t,
    ) -> libc::c_int;
    fn osip_accept_encoding_free(header: *mut osip_accept_encoding_t);
    fn osip_accept_encoding_clone(
        header: *const osip_accept_encoding_t,
        dest: *mut *mut osip_accept_encoding_t,
    ) -> libc::c_int;
    fn osip_call_info_free(header: *mut osip_call_info_t);
    fn osip_call_info_clone(
        header: *const osip_call_info_t,
        dest: *mut *mut osip_call_info_t,
    ) -> libc::c_int;
    fn osip_content_length_free(header: *mut osip_content_length_t);
    fn osip_content_length_clone(
        header: *const osip_content_length_t,
        dest: *mut *mut osip_content_length_t,
    ) -> libc::c_int;
    fn osip_authorization_free(header: *mut osip_authorization_t);
    fn osip_authorization_clone(
        header: *const osip_authorization_t,
        dest: *mut *mut osip_authorization_t,
    ) -> libc::c_int;
    fn osip_call_id_free(header: *mut osip_call_id_t);
    fn osip_call_id_clone(
        header: *const osip_call_id_t,
        dest: *mut *mut osip_call_id_t,
    ) -> libc::c_int;
    fn osip_from_free(header: *mut osip_from_t);
    fn osip_from_clone(
        header: *const osip_from_t,
        dest: *mut *mut osip_from_t,
    ) -> libc::c_int;
    fn osip_cseq_free(header: *mut osip_cseq_t);
    fn osip_cseq_clone(
        header: *const osip_cseq_t,
        dest: *mut *mut osip_cseq_t,
    ) -> libc::c_int;
    fn osip_www_authenticate_free(header: *mut osip_www_authenticate_t);
    fn osip_www_authenticate_clone(
        header: *const osip_www_authenticate_t,
        dest: *mut *mut osip_www_authenticate_t,
    ) -> libc::c_int;
    fn osip_record_route_free(header: *mut osip_record_route_t);
    fn osip_route_free(header: *mut osip_route_t);
    fn osip_to_free(header: *mut osip_to_t);
    fn osip_to_clone(header: *const osip_to_t, dest: *mut *mut osip_to_t) -> libc::c_int;
    fn osip_via_free(header: *mut osip_via_t);
    fn osip_via_clone(
        header: *const osip_via_t,
        dest: *mut *mut osip_via_t,
    ) -> libc::c_int;
    fn osip_body_free(body: *mut osip_body_t);
    fn osip_body_clone(
        body: *const osip_body_t,
        dest: *mut *mut osip_body_t,
    ) -> libc::c_int;
    fn osip_contact_clone(
        header: *const osip_contact_t,
        dest: *mut *mut osip_contact_t,
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
pub struct osip_accept_encoding {
    pub element: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_accept_encoding_t = osip_accept_encoding;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_call_info {
    pub element: *mut libc::c_char,
    pub gen_params: osip_list_t,
}
pub type osip_call_info_t = osip_call_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_content_length {
    pub value: *mut libc::c_char,
}
pub type osip_content_length_t = osip_content_length;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_authentication_info {
    pub auth_type: *mut libc::c_char,
    pub nextnonce: *mut libc::c_char,
    pub qop_options: *mut libc::c_char,
    pub rspauth: *mut libc::c_char,
    pub cnonce: *mut libc::c_char,
    pub nonce_count: *mut libc::c_char,
    pub snum: *mut libc::c_char,
    pub srand: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
}
pub type osip_authentication_info_t = osip_authentication_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_authorization {
    pub auth_type: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub uri: *mut libc::c_char,
    pub response: *mut libc::c_char,
    pub digest: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub cnonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub message_qop: *mut libc::c_char,
    pub nonce_count: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub gssapi_data: *mut libc::c_char,
    pub crand: *mut libc::c_char,
    pub cnum: *mut libc::c_char,
    pub auth_param: *mut libc::c_char,
}
pub type osip_authorization_t = osip_authorization;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_www_authenticate {
    pub auth_type: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub stale: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub qop_options: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub gssapi_data: *mut libc::c_char,
    pub auth_param: *mut libc::c_char,
}
pub type osip_www_authenticate_t = osip_www_authenticate;
pub type osip_record_route_t = osip_from_t;
pub type osip_route_t = osip_from_t;
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
pub static mut osip_protocol_version: *const libc::c_char = b"SIP/2.0\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn osip_message_init(
    mut sip: *mut *mut osip_message_t,
) -> libc::c_int {
    *sip = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_message_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_message_t>() as libc::c_ulong)
    }) as *mut osip_message_t;
    if (*sip).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *sip as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_message_t>() as libc::c_ulong,
    );
    osip_list_init(&mut (**sip).accepts);
    osip_list_init(&mut (**sip).accept_encodings);
    osip_list_init(&mut (**sip).accept_languages);
    osip_list_init(&mut (**sip).alert_infos);
    osip_list_init(&mut (**sip).allows);
    osip_list_init(&mut (**sip).authentication_infos);
    osip_list_init(&mut (**sip).authorizations);
    (**sip).call_id = 0 as *mut osip_call_id_t;
    osip_list_init(&mut (**sip).call_infos);
    osip_list_init(&mut (**sip).contacts);
    osip_list_init(&mut (**sip).content_encodings);
    (**sip).content_length = 0 as *mut osip_content_length_t;
    (**sip).content_type = 0 as *mut osip_content_type_t;
    (**sip).cseq = 0 as *mut osip_cseq_t;
    osip_list_init(&mut (**sip).error_infos);
    (**sip).from = 0 as *mut osip_from_t;
    (**sip).mime_version = 0 as *mut osip_mime_version_t;
    osip_list_init(&mut (**sip).proxy_authenticates);
    osip_list_init(&mut (**sip).proxy_authentication_infos);
    osip_list_init(&mut (**sip).proxy_authorizations);
    osip_list_init(&mut (**sip).record_routes);
    osip_list_init(&mut (**sip).routes);
    (**sip).to = 0 as *mut osip_to_t;
    osip_list_init(&mut (**sip).vias);
    osip_list_init(&mut (**sip).www_authenticates);
    osip_list_init(&mut (**sip).bodies);
    osip_list_init(&mut (**sip).headers);
    (**sip).message_property = 3 as libc::c_int;
    (**sip).message = 0 as *mut libc::c_char;
    (**sip).message_length = 0 as libc::c_int as size_t;
    (**sip).application_data = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_reason_phrase(
    mut sip: *mut osip_message_t,
    mut reason: *mut libc::c_char,
) {
    (*sip).reason_phrase = reason;
}
pub unsafe extern "C" fn osip_message_set_status_code(
    mut sip: *mut osip_message_t,
    mut status_code: libc::c_int,
) {
    (*sip).status_code = status_code;
}
pub unsafe extern "C" fn osip_message_set_method(
    mut sip: *mut osip_message_t,
    mut sip_method: *mut libc::c_char,
) {
    (*sip).sip_method = sip_method;
}
pub unsafe extern "C" fn osip_message_set_version(
    mut sip: *mut osip_message_t,
    mut sip_version: *mut libc::c_char,
) {
    (*sip).sip_version = sip_version;
}
pub unsafe extern "C" fn osip_message_set_uri(
    mut sip: *mut osip_message_t,
    mut url: *mut osip_uri_t,
) {
    (*sip).req_uri = url;
}
pub unsafe extern "C" fn osip_message_free(mut sip: *mut osip_message_t) {
    if sip.is_null() {
        return;
    }
    if !((*sip).sip_method).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sip).sip_method as *mut libc::c_void);
        } else {
            free((*sip).sip_method as *mut libc::c_void);
        }
    }
    if !((*sip).sip_version).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sip).sip_version as *mut libc::c_void);
        } else {
            free((*sip).sip_version as *mut libc::c_void);
        }
    }
    if !((*sip).req_uri).is_null() {
        osip_uri_free((*sip).req_uri);
    }
    if !((*sip).reason_phrase).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sip).reason_phrase as *mut libc::c_void);
        } else {
            free((*sip).reason_phrase as *mut libc::c_void);
        }
    }
    osip_list_special_free(
        &mut (*sip).accepts,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_content_type_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_content_type_free
                    as unsafe extern "C" fn(*mut osip_content_type_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).authorizations,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_authorization_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_authorization_free
                    as unsafe extern "C" fn(*mut osip_authorization_t) -> (),
            ),
        ),
    );
    if !((*sip).call_id).is_null() {
        osip_call_id_free((*sip).call_id);
    }
    osip_list_special_free(
        &mut (*sip).accept_encodings,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_accept_encoding_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_accept_encoding_free
                    as unsafe extern "C" fn(*mut osip_accept_encoding_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).accept_languages,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_accept_encoding_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_accept_encoding_free
                    as unsafe extern "C" fn(*mut osip_accept_encoding_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).alert_infos,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_call_info_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_call_info_free as unsafe extern "C" fn(*mut osip_call_info_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sip).allows,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_content_length_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_content_length_free
                    as unsafe extern "C" fn(*mut osip_content_length_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).authentication_infos,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_authentication_info_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_authentication_info_free
                    as unsafe extern "C" fn(*mut osip_authentication_info_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).content_encodings,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_content_length_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_content_length_free
                    as unsafe extern "C" fn(*mut osip_content_length_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).error_infos,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_call_info_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_call_info_free as unsafe extern "C" fn(*mut osip_call_info_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sip).proxy_authentication_infos,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_authentication_info_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_authentication_info_free
                    as unsafe extern "C" fn(*mut osip_authentication_info_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).call_infos,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_call_info_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_call_info_free as unsafe extern "C" fn(*mut osip_call_info_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sip).contacts,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_contact_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_contact_free as unsafe extern "C" fn(*mut osip_contact_t) -> ())),
    );
    if !((*sip).content_length).is_null() {
        osip_content_length_free((*sip).content_length);
    }
    if !((*sip).content_type).is_null() {
        osip_content_type_free((*sip).content_type);
    }
    if !((*sip).cseq).is_null() {
        osip_cseq_free((*sip).cseq);
    }
    if !((*sip).from).is_null() {
        osip_from_free((*sip).from);
    }
    if !((*sip).mime_version).is_null() {
        osip_content_length_free((*sip).mime_version);
    }
    osip_list_special_free(
        &mut (*sip).proxy_authenticates,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_www_authenticate_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_www_authenticate_free
                    as unsafe extern "C" fn(*mut osip_www_authenticate_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).proxy_authorizations,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_authorization_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_authorization_free
                    as unsafe extern "C" fn(*mut osip_authorization_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).record_routes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_record_route_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_record_route_free
                    as unsafe extern "C" fn(*mut osip_record_route_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).routes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_route_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_route_free as unsafe extern "C" fn(*mut osip_route_t) -> ())),
    );
    if !((*sip).to).is_null() {
        osip_to_free((*sip).to);
    }
    osip_list_special_free(
        &mut (*sip).vias,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_via_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_via_free as unsafe extern "C" fn(*mut osip_via_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sip).www_authenticates,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_www_authenticate_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                osip_www_authenticate_free
                    as unsafe extern "C" fn(*mut osip_www_authenticate_t) -> (),
            ),
        ),
    );
    osip_list_special_free(
        &mut (*sip).headers,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_header_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_header_free as unsafe extern "C" fn(*mut osip_header_t) -> ())),
    );
    osip_list_special_free(
        &mut (*sip).bodies,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut osip_body_t) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(osip_body_free as unsafe extern "C" fn(*mut osip_body_t) -> ())),
    );
    if !((*sip).message).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*sip).message as *mut libc::c_void);
        } else {
            free((*sip).message as *mut libc::c_void);
        }
    }
    if !sip.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(sip as *mut libc::c_void);
        } else {
            free(sip as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_message_clone(
    mut sip: *const osip_message_t,
    mut dest: *mut *mut osip_message_t,
) -> libc::c_int {
    let mut copy: *mut osip_message_t = 0 as *mut osip_message_t;
    let mut i: libc::c_int = 0;
    *dest = 0 as *mut osip_message_t;
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_message_init(&mut copy);
    if i != 0 as libc::c_int {
        return i;
    }
    (*copy).sip_method = osip_strdup((*sip).sip_method);
    if !((*sip).sip_method).is_null() && ((*copy).sip_method).is_null() {
        osip_message_free(copy);
        return -(4 as libc::c_int);
    }
    (*copy).sip_version = osip_strdup((*sip).sip_version);
    if !((*sip).sip_version).is_null() && ((*copy).sip_version).is_null() {
        osip_message_free(copy);
        return -(4 as libc::c_int);
    }
    (*copy).status_code = (*sip).status_code;
    (*copy).reason_phrase = osip_strdup((*sip).reason_phrase);
    if !((*sip).reason_phrase).is_null() && ((*copy).reason_phrase).is_null() {
        osip_message_free(copy);
        return -(4 as libc::c_int);
    }
    if !((*sip).req_uri).is_null() {
        i = osip_uri_clone((*sip).req_uri, &mut (*copy).req_uri);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    i = osip_list_clone(
        &(*sip).accepts,
        &mut (*copy).accepts,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_content_type_t,
                    *mut *mut osip_content_type_t,
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
                osip_content_type_clone
                    as unsafe extern "C" fn(
                        *const osip_content_type_t,
                        *mut *mut osip_content_type_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).accept_encodings,
        &mut (*copy).accept_encodings,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_accept_encoding_t,
                    *mut *mut osip_accept_encoding_t,
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
                osip_accept_encoding_clone
                    as unsafe extern "C" fn(
                        *const osip_accept_encoding_t,
                        *mut *mut osip_accept_encoding_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).accept_languages,
        &mut (*copy).accept_languages,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_accept_encoding_t,
                    *mut *mut osip_accept_encoding_t,
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
                osip_accept_encoding_clone
                    as unsafe extern "C" fn(
                        *const osip_accept_encoding_t,
                        *mut *mut osip_accept_encoding_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).alert_infos,
        &mut (*copy).alert_infos,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_call_info_t,
                    *mut *mut osip_call_info_t,
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
                osip_call_info_clone
                    as unsafe extern "C" fn(
                        *const osip_call_info_t,
                        *mut *mut osip_call_info_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).allows,
        &mut (*copy).allows,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_content_length_t,
                    *mut *mut osip_content_length_t,
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
                osip_content_length_clone
                    as unsafe extern "C" fn(
                        *const osip_content_length_t,
                        *mut *mut osip_content_length_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).authentication_infos,
        &mut (*copy).authentication_infos,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_authentication_info_t,
                    *mut *mut osip_authentication_info_t,
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
                osip_authentication_info_clone
                    as unsafe extern "C" fn(
                        *const osip_authentication_info_t,
                        *mut *mut osip_authentication_info_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).content_encodings,
        &mut (*copy).content_encodings,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_content_length_t,
                    *mut *mut osip_content_length_t,
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
                osip_content_length_clone
                    as unsafe extern "C" fn(
                        *const osip_content_length_t,
                        *mut *mut osip_content_length_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).error_infos,
        &mut (*copy).error_infos,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_call_info_t,
                    *mut *mut osip_call_info_t,
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
                osip_call_info_clone
                    as unsafe extern "C" fn(
                        *const osip_call_info_t,
                        *mut *mut osip_call_info_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).proxy_authentication_infos,
        &mut (*copy).proxy_authentication_infos,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_authentication_info_t,
                    *mut *mut osip_authentication_info_t,
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
                osip_authentication_info_clone
                    as unsafe extern "C" fn(
                        *const osip_authentication_info_t,
                        *mut *mut osip_authentication_info_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).call_infos,
        &mut (*copy).call_infos,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_call_info_t,
                    *mut *mut osip_call_info_t,
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
                osip_call_info_clone
                    as unsafe extern "C" fn(
                        *const osip_call_info_t,
                        *mut *mut osip_call_info_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).authorizations,
        &mut (*copy).authorizations,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_authorization_t,
                    *mut *mut osip_authorization_t,
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
                osip_authorization_clone
                    as unsafe extern "C" fn(
                        *const osip_authorization_t,
                        *mut *mut osip_authorization_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    if !((*sip).call_id).is_null() {
        i = osip_call_id_clone((*sip).call_id, &mut (*copy).call_id);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    i = osip_list_clone(
        &(*sip).contacts,
        &mut (*copy).contacts,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_contact_t,
                    *mut *mut osip_contact_t,
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
                osip_contact_clone
                    as unsafe extern "C" fn(
                        *const osip_contact_t,
                        *mut *mut osip_contact_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    if !((*sip).content_length).is_null() {
        i = osip_content_length_clone(
            (*sip).content_length,
            &mut (*copy).content_length,
        );
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    if !((*sip).content_type).is_null() {
        i = osip_content_type_clone((*sip).content_type, &mut (*copy).content_type);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    if !((*sip).cseq).is_null() {
        i = osip_cseq_clone((*sip).cseq, &mut (*copy).cseq);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    if !((*sip).from).is_null() {
        i = osip_from_clone((*sip).from, &mut (*copy).from);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    if !((*sip).mime_version).is_null() {
        i = osip_content_length_clone((*sip).mime_version, &mut (*copy).mime_version);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    i = osip_list_clone(
        &(*sip).proxy_authenticates,
        &mut (*copy).proxy_authenticates,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_www_authenticate_t,
                    *mut *mut osip_www_authenticate_t,
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
                osip_www_authenticate_clone
                    as unsafe extern "C" fn(
                        *const osip_www_authenticate_t,
                        *mut *mut osip_www_authenticate_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).proxy_authorizations,
        &mut (*copy).proxy_authorizations,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_authorization_t,
                    *mut *mut osip_authorization_t,
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
                osip_authorization_clone
                    as unsafe extern "C" fn(
                        *const osip_authorization_t,
                        *mut *mut osip_authorization_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).record_routes,
        &mut (*copy).record_routes,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_from_t,
                    *mut *mut osip_from_t,
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
                osip_from_clone
                    as unsafe extern "C" fn(
                        *const osip_from_t,
                        *mut *mut osip_from_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).routes,
        &mut (*copy).routes,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_from_t,
                    *mut *mut osip_from_t,
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
                osip_from_clone
                    as unsafe extern "C" fn(
                        *const osip_from_t,
                        *mut *mut osip_from_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    if !((*sip).to).is_null() {
        i = osip_to_clone((*sip).to, &mut (*copy).to);
        if i != 0 as libc::c_int {
            osip_message_free(copy);
            return i;
        }
    }
    i = osip_list_clone(
        &(*sip).vias,
        &mut (*copy).vias,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_via_t,
                    *mut *mut osip_via_t,
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
                osip_via_clone
                    as unsafe extern "C" fn(
                        *const osip_via_t,
                        *mut *mut osip_via_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).www_authenticates,
        &mut (*copy).www_authenticates,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_www_authenticate_t,
                    *mut *mut osip_www_authenticate_t,
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
                osip_www_authenticate_clone
                    as unsafe extern "C" fn(
                        *const osip_www_authenticate_t,
                        *mut *mut osip_www_authenticate_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).headers,
        &mut (*copy).headers,
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
        osip_message_free(copy);
        return i;
    }
    i = osip_list_clone(
        &(*sip).bodies,
        &mut (*copy).bodies,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_body_t,
                    *mut *mut osip_body_t,
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
                osip_body_clone
                    as unsafe extern "C" fn(
                        *const osip_body_t,
                        *mut *mut osip_body_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_message_free(copy);
        return i;
    }
    (*copy).message_length = (*sip).message_length;
    (*copy).message = osip_strdup((*sip).message);
    if ((*copy).message).is_null() && !((*sip).message).is_null() {
        osip_message_free(copy);
        return -(4 as libc::c_int);
    }
    (*copy).message_property = (*sip).message_property;
    (*copy).application_data = (*sip).application_data;
    *dest = copy;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_get_knownheaderlist(
    mut header_list: *mut osip_list_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut libc::c_void,
) -> libc::c_int {
    *dest = 0 as *mut libc::c_void;
    if osip_list_size(header_list) <= pos {
        return -(1 as libc::c_int);
    }
    *dest = osip_list_get(header_list, pos);
    return pos;
}
