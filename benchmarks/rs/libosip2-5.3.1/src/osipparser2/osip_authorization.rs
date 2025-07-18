use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn osip_strdup(ch: *const libc::c_char) -> *mut libc::c_char;
    fn __osip_quote_find(qstring: *const libc::c_char) -> *const libc::c_char;
    fn osip_str_append(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn osip_strn_append(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn __osip_quoted_string_set(
        name: *const libc::c_char,
        str: *const libc::c_char,
        result: *mut *mut libc::c_char,
        next: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn __osip_token_set(
        name: *const libc::c_char,
        str: *const libc::c_char,
        result: *mut *mut libc::c_char,
        next: *mut *const libc::c_char,
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
pub unsafe extern "C" fn osip_authorization_init(
    mut dest: *mut *mut osip_authorization_t,
) -> libc::c_int {
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_authorization_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_authorization_t>() as libc::c_ulong)
    }) as *mut osip_authorization_t;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *dest as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_authorization_t>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_authorization(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut authorization: *mut osip_authorization_t = 0 as *mut osip_authorization_t;
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_authorization_init(&mut authorization);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_authorization_parse(authorization, hvalue);
    if i != 0 as libc::c_int {
        osip_authorization_free(authorization);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(
        &mut (*sip).authorizations,
        authorization as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_authorization_parse(
    mut auth: *mut osip_authorization_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut space: *const libc::c_char = 0 as *const libc::c_char;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    space = strchr(hvalue, ' ' as i32);
    if space.is_null() {
        return -(5 as libc::c_int);
    }
    if (space.offset_from(hvalue) as libc::c_long) < 1 as libc::c_int as libc::c_long {
        return -(5 as libc::c_int);
    }
    (*auth)
        .auth_type = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (space.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        )
    } else {
        malloc(
            (space.offset_from(hvalue) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*auth).auth_type).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*auth).auth_type,
        hvalue,
        space.offset_from(hvalue) as libc::c_long as size_t,
    );
    loop {
        let mut parse_ok: libc::c_int = 0 as libc::c_int;
        i = __osip_quoted_string_set(
            b"username\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).username,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"realm\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).realm,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"nonce\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).nonce,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"uri\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).uri,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"response\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).response,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"digest\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).digest,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_token_set(
            b"algorithm\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).algorithm,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"cnonce\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).cnonce,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"opaque\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).opaque,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_token_set(
            b"qop\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).message_qop,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_token_set(
            b"nc\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).nonce_count,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_token_set(
            b"version\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).version,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"targetname\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).targetname,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"gssapi-data\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).gssapi_data,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"crand\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).crand,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        i = __osip_quoted_string_set(
            b"cnum\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*auth).cnum,
            &mut next,
        );
        if i != 0 as libc::c_int {
            return i;
        }
        if next.is_null() {
            return 0 as libc::c_int
        } else if next != space {
            space = next;
            parse_ok += 1;
            parse_ok;
        }
        if 0 as libc::c_int == parse_ok {
            let mut quote1: *const libc::c_char = 0 as *const libc::c_char;
            let mut quote2: *const libc::c_char = 0 as *const libc::c_char;
            let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
            if strlen(space) < 1 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int;
            }
            tmp = strchr(space.offset(1 as libc::c_int as isize), ',' as i32);
            if tmp.is_null() {
                return 0 as libc::c_int;
            }
            quote1 = __osip_quote_find(space);
            if !quote1.is_null() && quote1 < tmp {
                quote2 = __osip_quote_find(quote1.offset(1 as libc::c_int as isize));
                if quote2.is_null() {
                    return -(5 as libc::c_int);
                }
                if tmp < quote2 {
                    space = strchr(quote2, ',' as i32);
                } else {
                    space = tmp;
                }
                if space.is_null() {
                    return 0 as libc::c_int;
                }
            } else {
                space = tmp;
            }
        }
    };
}
pub unsafe extern "C" fn osip_message_get_authorization(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_authorization_t,
) -> libc::c_int {
    let mut authorization: *mut osip_authorization_t = 0 as *mut osip_authorization_t;
    *dest = 0 as *mut osip_authorization_t;
    if osip_list_size(&(*sip).authorizations) <= pos {
        return -(1 as libc::c_int);
    }
    authorization = osip_list_get(&(*sip).authorizations, pos)
        as *mut osip_authorization_t;
    *dest = authorization;
    return pos;
}
pub unsafe extern "C" fn osip_authorization_get_auth_type(
    mut authorization: *const osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).auth_type;
}
pub unsafe extern "C" fn osip_authorization_set_auth_type(
    mut authorization: *mut osip_authorization_t,
    mut auth_type: *mut libc::c_char,
) {
    (*authorization).auth_type = auth_type;
}
pub unsafe extern "C" fn osip_authorization_get_username(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).username;
}
pub unsafe extern "C" fn osip_authorization_set_username(
    mut authorization: *mut osip_authorization_t,
    mut username: *mut libc::c_char,
) {
    (*authorization).username = username;
}
pub unsafe extern "C" fn osip_authorization_get_realm(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).realm;
}
pub unsafe extern "C" fn osip_authorization_set_realm(
    mut authorization: *mut osip_authorization_t,
    mut realm: *mut libc::c_char,
) {
    (*authorization).realm = realm;
}
pub unsafe extern "C" fn osip_authorization_get_nonce(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).nonce;
}
pub unsafe extern "C" fn osip_authorization_set_nonce(
    mut authorization: *mut osip_authorization_t,
    mut nonce: *mut libc::c_char,
) {
    (*authorization).nonce = nonce;
}
pub unsafe extern "C" fn osip_authorization_get_uri(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).uri;
}
pub unsafe extern "C" fn osip_authorization_set_uri(
    mut authorization: *mut osip_authorization_t,
    mut uri: *mut libc::c_char,
) {
    (*authorization).uri = uri;
}
pub unsafe extern "C" fn osip_authorization_get_response(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).response;
}
pub unsafe extern "C" fn osip_authorization_set_response(
    mut authorization: *mut osip_authorization_t,
    mut response: *mut libc::c_char,
) {
    (*authorization).response = response;
}
pub unsafe extern "C" fn osip_authorization_get_digest(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).digest;
}
pub unsafe extern "C" fn osip_authorization_set_digest(
    mut authorization: *mut osip_authorization_t,
    mut digest: *mut libc::c_char,
) {
    (*authorization).digest = digest;
}
pub unsafe extern "C" fn osip_authorization_get_algorithm(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).algorithm;
}
pub unsafe extern "C" fn osip_authorization_set_algorithm(
    mut authorization: *mut osip_authorization_t,
    mut algorithm: *mut libc::c_char,
) {
    (*authorization).algorithm = algorithm;
}
pub unsafe extern "C" fn osip_authorization_get_cnonce(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).cnonce;
}
pub unsafe extern "C" fn osip_authorization_set_cnonce(
    mut authorization: *mut osip_authorization_t,
    mut cnonce: *mut libc::c_char,
) {
    (*authorization).cnonce = cnonce;
}
pub unsafe extern "C" fn osip_authorization_get_opaque(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).opaque;
}
pub unsafe extern "C" fn osip_authorization_set_opaque(
    mut authorization: *mut osip_authorization_t,
    mut opaque: *mut libc::c_char,
) {
    (*authorization).opaque = opaque;
}
pub unsafe extern "C" fn osip_authorization_get_message_qop(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).message_qop;
}
pub unsafe extern "C" fn osip_authorization_set_message_qop(
    mut authorization: *mut osip_authorization_t,
    mut message_qop: *mut libc::c_char,
) {
    (*authorization).message_qop = message_qop;
}
pub unsafe extern "C" fn osip_authorization_get_nonce_count(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).nonce_count;
}
pub unsafe extern "C" fn osip_authorization_set_nonce_count(
    mut authorization: *mut osip_authorization_t,
    mut nonce_count: *mut libc::c_char,
) {
    (*authorization).nonce_count = nonce_count;
}
pub unsafe extern "C" fn osip_authorization_get_version(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).version;
}
pub unsafe extern "C" fn osip_authorization_set_version(
    mut authorization: *mut osip_authorization_t,
    mut version: *mut libc::c_char,
) {
    (*authorization).version = version;
}
pub unsafe extern "C" fn osip_authorization_get_targetname(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).targetname;
}
pub unsafe extern "C" fn osip_authorization_set_targetname(
    mut authorization: *mut osip_authorization_t,
    mut targetname: *mut libc::c_char,
) {
    (*authorization).targetname = targetname;
}
pub unsafe extern "C" fn osip_authorization_get_gssapi_data(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).gssapi_data;
}
pub unsafe extern "C" fn osip_authorization_set_gssapi_data(
    mut authorization: *mut osip_authorization_t,
    mut gssapi_data: *mut libc::c_char,
) {
    (*authorization).gssapi_data = gssapi_data;
}
pub unsafe extern "C" fn osip_authorization_get_crand(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).crand;
}
pub unsafe extern "C" fn osip_authorization_set_crand(
    mut authorization: *mut osip_authorization_t,
    mut crand: *mut libc::c_char,
) {
    (*authorization).crand = crand;
}
pub unsafe extern "C" fn osip_authorization_get_cnum(
    mut authorization: *mut osip_authorization_t,
) -> *mut libc::c_char {
    return (*authorization).cnum;
}
pub unsafe extern "C" fn osip_authorization_set_cnum(
    mut authorization: *mut osip_authorization_t,
    mut cnum: *mut libc::c_char,
) {
    (*authorization).cnum = cnum;
}
pub unsafe extern "C" fn osip_authorization_to_str(
    mut auth: *const osip_authorization_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut first: libc::c_int = 1 as libc::c_int;
    *dest = 0 as *mut libc::c_char;
    if auth.is_null() || ((*auth).auth_type).is_null() {
        return -(2 as libc::c_int);
    }
    len = (strlen((*auth).auth_type)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !((*auth).username).is_null() {
        len = len
            .wrapping_add(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*auth).username));
    }
    if !((*auth).realm).is_null() {
        len = len
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*auth).realm));
    }
    if !((*auth).nonce).is_null() {
        len = len
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*auth).nonce));
    }
    if !((*auth).uri).is_null() {
        len = len
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*auth).uri));
    }
    if !((*auth).response).is_null() {
        len = len
            .wrapping_add(11 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*auth).response));
    }
    len = len.wrapping_add(2 as libc::c_int as libc::c_ulong);
    if !((*auth).digest).is_null() {
        len = len
            .wrapping_add(strlen((*auth).digest))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).algorithm).is_null() {
        len = len
            .wrapping_add(strlen((*auth).algorithm))
            .wrapping_add(12 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).cnonce).is_null() {
        len = len
            .wrapping_add(strlen((*auth).cnonce))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).opaque).is_null() {
        len = len
            .wrapping_add(9 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*auth).opaque));
    }
    if !((*auth).nonce_count).is_null() {
        len = len
            .wrapping_add(strlen((*auth).nonce_count))
            .wrapping_add(5 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).message_qop).is_null() {
        len = len
            .wrapping_add(strlen((*auth).message_qop))
            .wrapping_add(6 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).version).is_null() {
        len = len
            .wrapping_add(strlen((*auth).version))
            .wrapping_add(10 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).targetname).is_null() {
        len = len
            .wrapping_add(strlen((*auth).targetname))
            .wrapping_add(13 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).gssapi_data).is_null() {
        len = len
            .wrapping_add(strlen((*auth).gssapi_data))
            .wrapping_add(14 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).crand).is_null() {
        len = len
            .wrapping_add(strlen((*auth).crand))
            .wrapping_add(8 as libc::c_int as libc::c_ulong);
    }
    if !((*auth).cnum).is_null() {
        len = len
            .wrapping_add(strlen((*auth).cnum))
            .wrapping_add(7 as libc::c_int as libc::c_ulong);
    }
    tmp = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if tmp.is_null() {
        return -(4 as libc::c_int);
    }
    *dest = tmp;
    tmp = osip_str_append(tmp, (*auth).auth_type);
    if !((*auth).username).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" username=\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).username);
    }
    if !((*auth).realm).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" realm=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).realm);
    }
    if !((*auth).nonce).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" nonce=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).nonce);
    }
    if !((*auth).uri).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" uri=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).uri);
    }
    if !((*auth).response).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" response=\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).response);
    }
    if !((*auth).digest).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" digest=\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).digest);
    }
    if !((*auth).algorithm).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" algorithm=\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).algorithm);
    }
    if !((*auth).cnonce).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" cnonce=\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).cnonce);
    }
    if !((*auth).opaque).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" opaque=\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).opaque);
    }
    if !((*auth).message_qop).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" qop=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).message_qop);
    }
    if !((*auth).nonce_count).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" nc=\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).nonce_count);
    }
    if !((*auth).version).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" version=\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).version);
    }
    if !((*auth).targetname).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" targetname=\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).targetname);
    }
    if !((*auth).gssapi_data).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" gssapi-data=\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).gssapi_data);
    }
    if !((*auth).crand).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" crand=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).crand);
    }
    if !((*auth).cnum).is_null() {
        if first == 0 {
            tmp = osip_strn_append(
                tmp,
                b",\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        first = 0 as libc::c_int;
        tmp = osip_strn_append(
            tmp,
            b" cnum=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*auth).cnum);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_authorization_free(
    mut authorization: *mut osip_authorization_t,
) {
    if authorization.is_null() {
        return;
    }
    if !((*authorization).auth_type).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).auth_type as *mut libc::c_void);
        } else {
            free((*authorization).auth_type as *mut libc::c_void);
        }
    }
    if !((*authorization).username).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).username as *mut libc::c_void);
        } else {
            free((*authorization).username as *mut libc::c_void);
        }
    }
    if !((*authorization).realm).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).realm as *mut libc::c_void);
        } else {
            free((*authorization).realm as *mut libc::c_void);
        }
    }
    if !((*authorization).nonce).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).nonce as *mut libc::c_void);
        } else {
            free((*authorization).nonce as *mut libc::c_void);
        }
    }
    if !((*authorization).uri).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).uri as *mut libc::c_void);
        } else {
            free((*authorization).uri as *mut libc::c_void);
        }
    }
    if !((*authorization).response).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).response as *mut libc::c_void);
        } else {
            free((*authorization).response as *mut libc::c_void);
        }
    }
    if !((*authorization).digest).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).digest as *mut libc::c_void);
        } else {
            free((*authorization).digest as *mut libc::c_void);
        }
    }
    if !((*authorization).algorithm).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).algorithm as *mut libc::c_void);
        } else {
            free((*authorization).algorithm as *mut libc::c_void);
        }
    }
    if !((*authorization).cnonce).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).cnonce as *mut libc::c_void);
        } else {
            free((*authorization).cnonce as *mut libc::c_void);
        }
    }
    if !((*authorization).opaque).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).opaque as *mut libc::c_void);
        } else {
            free((*authorization).opaque as *mut libc::c_void);
        }
    }
    if !((*authorization).message_qop).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).message_qop as *mut libc::c_void);
        } else {
            free((*authorization).message_qop as *mut libc::c_void);
        }
    }
    if !((*authorization).nonce_count).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).nonce_count as *mut libc::c_void);
        } else {
            free((*authorization).nonce_count as *mut libc::c_void);
        }
    }
    if !((*authorization).version).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).version as *mut libc::c_void);
        } else {
            free((*authorization).version as *mut libc::c_void);
        }
    }
    if !((*authorization).targetname).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).targetname as *mut libc::c_void);
        } else {
            free((*authorization).targetname as *mut libc::c_void);
        }
    }
    if !((*authorization).gssapi_data).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).gssapi_data as *mut libc::c_void);
        } else {
            free((*authorization).gssapi_data as *mut libc::c_void);
        }
    }
    if !((*authorization).crand).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).crand as *mut libc::c_void);
        } else {
            free((*authorization).crand as *mut libc::c_void);
        }
    }
    if !((*authorization).cnum).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authorization).cnum as *mut libc::c_void);
        } else {
            free((*authorization).cnum as *mut libc::c_void);
        }
    }
    if !authorization.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(authorization as *mut libc::c_void);
        } else {
            free(authorization as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_authorization_clone(
    mut auth: *const osip_authorization_t,
    mut dest: *mut *mut osip_authorization_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut au: *mut osip_authorization_t = 0 as *mut osip_authorization_t;
    *dest = 0 as *mut osip_authorization_t;
    if auth.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_authorization_init(&mut au);
    if i != 0 as libc::c_int {
        return i;
    }
    if !((*auth).auth_type).is_null() {
        (*au).auth_type = osip_strdup((*auth).auth_type);
        if ((*au).auth_type).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).username).is_null() {
        (*au).username = osip_strdup((*auth).username);
        if ((*au).username).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).realm).is_null() {
        (*au).realm = osip_strdup((*auth).realm);
        if ((*auth).realm).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).nonce).is_null() {
        (*au).nonce = osip_strdup((*auth).nonce);
        if ((*auth).nonce).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).uri).is_null() {
        (*au).uri = osip_strdup((*auth).uri);
        if ((*au).uri).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).response).is_null() {
        (*au).response = osip_strdup((*auth).response);
        if ((*auth).response).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).digest).is_null() {
        (*au).digest = osip_strdup((*auth).digest);
        if ((*au).digest).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).algorithm).is_null() {
        (*au).algorithm = osip_strdup((*auth).algorithm);
        if ((*auth).algorithm).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).cnonce).is_null() {
        (*au).cnonce = osip_strdup((*auth).cnonce);
        if ((*au).cnonce).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).opaque).is_null() {
        (*au).opaque = osip_strdup((*auth).opaque);
        if ((*auth).opaque).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).message_qop).is_null() {
        (*au).message_qop = osip_strdup((*auth).message_qop);
        if ((*auth).message_qop).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).nonce_count).is_null() {
        (*au).nonce_count = osip_strdup((*auth).nonce_count);
        if ((*auth).nonce_count).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).version).is_null() {
        (*au).version = osip_strdup((*auth).version);
        if ((*auth).version).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).targetname).is_null() {
        (*au).targetname = osip_strdup((*auth).targetname);
        if ((*auth).targetname).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).gssapi_data).is_null() {
        (*au).gssapi_data = osip_strdup((*auth).gssapi_data);
        if ((*auth).gssapi_data).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).crand).is_null() {
        (*au).crand = osip_strdup((*auth).crand);
        if ((*auth).crand).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    if !((*auth).cnum).is_null() {
        (*au).cnum = osip_strdup((*auth).cnum);
        if ((*auth).cnum).is_null() {
            osip_authorization_free(au);
            return -(4 as libc::c_int);
        }
    }
    *dest = au;
    return 0 as libc::c_int;
}
