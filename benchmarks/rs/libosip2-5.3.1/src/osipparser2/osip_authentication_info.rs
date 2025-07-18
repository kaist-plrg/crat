use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
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
pub unsafe extern "C" fn osip_authentication_info_init(
    mut dest: *mut *mut osip_authentication_info_t,
) -> libc::c_int {
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            ::std::mem::size_of::<osip_authentication_info_t>() as libc::c_ulong,
        )
    } else {
        malloc(::std::mem::size_of::<osip_authentication_info_t>() as libc::c_ulong)
    }) as *mut osip_authentication_info_t;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *dest as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_authentication_info_t>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_authentication_info(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut authentication_info: *mut osip_authentication_info_t = 0
        as *mut osip_authentication_info_t;
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_authentication_info_init(&mut authentication_info);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_authentication_info_parse(authentication_info, hvalue);
    if i != 0 as libc::c_int {
        osip_authentication_info_free(authentication_info);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(
        &mut (*sip).authentication_infos,
        authentication_info as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_authentication_info_parse(
    mut ainfo: *mut osip_authentication_info_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut space: *const libc::c_char = 0 as *const libc::c_char;
    let mut hack: *const libc::c_char = 0 as *const libc::c_char;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    space = strchr(hvalue, ' ' as i32);
    hack = strchr(hvalue, '=' as i32);
    if !space.is_null() && !hack.is_null() && hack > space {
        (*ainfo)
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
        if ((*ainfo).auth_type).is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            (*ainfo).auth_type,
            hvalue,
            space.offset_from(hvalue) as libc::c_long as size_t,
        );
    } else {
        space = hvalue;
    }
    loop {
        let mut parse_ok: libc::c_int = 0 as libc::c_int;
        i = __osip_quoted_string_set(
            b"nextnonce\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*ainfo).nextnonce,
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
            &mut (*ainfo).cnonce,
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
            b"rspauth\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*ainfo).rspauth,
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
            &mut (*ainfo).nonce_count,
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
            &mut (*ainfo).qop_options,
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
            b"snum\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*ainfo).snum,
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
            b"srand\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*ainfo).srand,
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
            &mut (*ainfo).targetname,
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
            &mut (*ainfo).realm,
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
            &mut (*ainfo).opaque,
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
pub unsafe extern "C" fn osip_message_get_authentication_info(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_authentication_info_t,
) -> libc::c_int {
    let mut authentication_info: *mut osip_authentication_info_t = 0
        as *mut osip_authentication_info_t;
    *dest = 0 as *mut osip_authentication_info_t;
    if osip_list_size(&(*sip).authentication_infos) <= pos {
        return -(1 as libc::c_int);
    }
    authentication_info = osip_list_get(&(*sip).authentication_infos, pos)
        as *mut osip_authentication_info_t;
    *dest = authentication_info;
    return pos;
}
pub unsafe extern "C" fn osip_authentication_info_get_auth_type(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).auth_type;
}
pub unsafe extern "C" fn osip_authentication_info_set_auth_type(
    mut authentication_info: *mut osip_authentication_info_t,
    mut auth_type: *mut libc::c_char,
) {
    (*authentication_info).auth_type = auth_type;
}
pub unsafe extern "C" fn osip_authentication_info_get_nextnonce(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).nextnonce;
}
pub unsafe extern "C" fn osip_authentication_info_set_nextnonce(
    mut authentication_info: *mut osip_authentication_info_t,
    mut nextnonce: *mut libc::c_char,
) {
    (*authentication_info).nextnonce = nextnonce;
}
pub unsafe extern "C" fn osip_authentication_info_get_cnonce(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).cnonce;
}
pub unsafe extern "C" fn osip_authentication_info_set_cnonce(
    mut authentication_info: *mut osip_authentication_info_t,
    mut cnonce: *mut libc::c_char,
) {
    (*authentication_info).cnonce = cnonce;
}
pub unsafe extern "C" fn osip_authentication_info_get_rspauth(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).rspauth;
}
pub unsafe extern "C" fn osip_authentication_info_set_rspauth(
    mut authentication_info: *mut osip_authentication_info_t,
    mut rspauth: *mut libc::c_char,
) {
    (*authentication_info).rspauth = rspauth;
}
pub unsafe extern "C" fn osip_authentication_info_get_nonce_count(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).nonce_count;
}
pub unsafe extern "C" fn osip_authentication_info_set_nonce_count(
    mut authentication_info: *mut osip_authentication_info_t,
    mut nonce_count: *mut libc::c_char,
) {
    (*authentication_info).nonce_count = nonce_count;
}
pub unsafe extern "C" fn osip_authentication_info_get_qop_options(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).qop_options;
}
pub unsafe extern "C" fn osip_authentication_info_set_qop_options(
    mut authentication_info: *mut osip_authentication_info_t,
    mut qop_options: *mut libc::c_char,
) {
    (*authentication_info).qop_options = qop_options;
}
pub unsafe extern "C" fn osip_authentication_info_get_snum(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).snum;
}
pub unsafe extern "C" fn osip_authentication_info_set_snum(
    mut authentication_info: *mut osip_authentication_info_t,
    mut snum: *mut libc::c_char,
) {
    (*authentication_info).snum = snum;
}
pub unsafe extern "C" fn osip_authentication_info_get_srand(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).srand;
}
pub unsafe extern "C" fn osip_authentication_info_set_srand(
    mut authentication_info: *mut osip_authentication_info_t,
    mut srand: *mut libc::c_char,
) {
    (*authentication_info).srand = srand;
}
pub unsafe extern "C" fn osip_authentication_info_get_targetname(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).targetname;
}
pub unsafe extern "C" fn osip_authentication_info_set_targetname(
    mut authentication_info: *mut osip_authentication_info_t,
    mut targetname: *mut libc::c_char,
) {
    (*authentication_info).targetname = targetname;
}
pub unsafe extern "C" fn osip_authentication_info_get_realm(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).realm;
}
pub unsafe extern "C" fn osip_authentication_info_set_realm(
    mut authentication_info: *mut osip_authentication_info_t,
    mut realm: *mut libc::c_char,
) {
    (*authentication_info).realm = realm;
}
pub unsafe extern "C" fn osip_authentication_info_get_opaque(
    mut authentication_info: *mut osip_authentication_info_t,
) -> *mut libc::c_char {
    return (*authentication_info).opaque;
}
pub unsafe extern "C" fn osip_authentication_info_set_opaque(
    mut authentication_info: *mut osip_authentication_info_t,
    mut opaque: *mut libc::c_char,
) {
    (*authentication_info).opaque = opaque;
}
pub unsafe extern "C" fn osip_authentication_info_to_str(
    mut ainfo: *const osip_authentication_info_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    *dest = 0 as *mut libc::c_char;
    if ainfo.is_null() {
        return -(2 as libc::c_int);
    }
    len = 0 as libc::c_int as size_t;
    if !((*ainfo).auth_type).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).auth_type))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).nextnonce).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).nextnonce))
            .wrapping_add(12 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).rspauth).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).rspauth))
            .wrapping_add(10 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).cnonce).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).cnonce))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).nonce_count).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).nonce_count))
            .wrapping_add(5 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).qop_options).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).qop_options))
            .wrapping_add(6 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).snum).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).snum))
            .wrapping_add(7 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).srand).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).srand))
            .wrapping_add(8 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).targetname).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).targetname))
            .wrapping_add(13 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).realm).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).realm))
            .wrapping_add(8 as libc::c_int as libc::c_ulong);
    }
    if !((*ainfo).opaque).is_null() {
        len = len
            .wrapping_add(strlen((*ainfo).opaque))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    len = len.wrapping_add(1);
    len;
    tmp = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if tmp.is_null() {
        return -(4 as libc::c_int);
    }
    *dest = tmp;
    start = tmp;
    if !((*ainfo).auth_type).is_null() {
        tmp = osip_str_append(tmp, (*ainfo).auth_type);
        tmp = osip_str_append(tmp, b" \0" as *const u8 as *const libc::c_char);
        start = tmp;
    }
    if !((*ainfo).qop_options).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"qop=\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).qop_options);
    }
    if !((*ainfo).nextnonce).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"nextnonce=\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).nextnonce);
    }
    if !((*ainfo).rspauth).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"rspauth=\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).rspauth);
    }
    if !((*ainfo).cnonce).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"cnonce=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).cnonce);
    }
    if !((*ainfo).nonce_count).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"nc=\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).nonce_count);
    }
    if !((*ainfo).snum).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"snum=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).snum);
    }
    if !((*ainfo).srand).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"srand=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).srand);
    }
    if !((*ainfo).targetname).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"targetname=\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).targetname);
    }
    if !((*ainfo).realm).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"realm=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).realm);
    }
    if !((*ainfo).opaque).is_null() {
        if tmp != start {
            tmp = osip_strn_append(
                tmp,
                b", \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        }
        tmp = osip_strn_append(
            tmp,
            b"opaque=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*ainfo).opaque);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_authentication_info_free(
    mut authentication_info: *mut osip_authentication_info_t,
) {
    if authentication_info.is_null() {
        return;
    }
    if !((*authentication_info).auth_type).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*authentication_info).auth_type as *mut libc::c_void);
        } else {
            free((*authentication_info).auth_type as *mut libc::c_void);
        }
    }
    if !((*authentication_info).nextnonce).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*authentication_info).nextnonce as *mut libc::c_void);
        } else {
            free((*authentication_info).nextnonce as *mut libc::c_void);
        }
    }
    if !((*authentication_info).rspauth).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authentication_info).rspauth as *mut libc::c_void);
        } else {
            free((*authentication_info).rspauth as *mut libc::c_void);
        }
    }
    if !((*authentication_info).cnonce).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authentication_info).cnonce as *mut libc::c_void);
        } else {
            free((*authentication_info).cnonce as *mut libc::c_void);
        }
    }
    if !((*authentication_info).nonce_count).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*authentication_info).nonce_count as *mut libc::c_void);
        } else {
            free((*authentication_info).nonce_count as *mut libc::c_void);
        }
    }
    if !((*authentication_info).qop_options).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*authentication_info).qop_options as *mut libc::c_void);
        } else {
            free((*authentication_info).qop_options as *mut libc::c_void);
        }
    }
    if !((*authentication_info).snum).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authentication_info).snum as *mut libc::c_void);
        } else {
            free((*authentication_info).snum as *mut libc::c_void);
        }
    }
    if !((*authentication_info).srand).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authentication_info).srand as *mut libc::c_void);
        } else {
            free((*authentication_info).srand as *mut libc::c_void);
        }
    }
    if !((*authentication_info).targetname).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*authentication_info).targetname as *mut libc::c_void);
        } else {
            free((*authentication_info).targetname as *mut libc::c_void);
        }
    }
    if !((*authentication_info).realm).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authentication_info).realm as *mut libc::c_void);
        } else {
            free((*authentication_info).realm as *mut libc::c_void);
        }
    }
    if !((*authentication_info).opaque).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*authentication_info).opaque as *mut libc::c_void);
        } else {
            free((*authentication_info).opaque as *mut libc::c_void);
        }
    }
    if !authentication_info.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(authentication_info as *mut libc::c_void);
        } else {
            free(authentication_info as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_authentication_info_clone(
    mut ainfo: *const osip_authentication_info_t,
    mut dest: *mut *mut osip_authentication_info_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut wa: *mut osip_authentication_info_t = 0 as *mut osip_authentication_info_t;
    *dest = 0 as *mut osip_authentication_info_t;
    if ainfo.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_authentication_info_init(&mut wa);
    if i != 0 as libc::c_int {
        return i;
    }
    if !((*ainfo).auth_type).is_null() {
        (*wa).auth_type = osip_strdup((*ainfo).auth_type);
    }
    if !((*ainfo).nextnonce).is_null() {
        (*wa).nextnonce = osip_strdup((*ainfo).nextnonce);
    }
    if !((*ainfo).cnonce).is_null() {
        (*wa).cnonce = osip_strdup((*ainfo).cnonce);
    }
    if !((*ainfo).rspauth).is_null() {
        (*wa).rspauth = osip_strdup((*ainfo).rspauth);
    }
    if !((*ainfo).nonce_count).is_null() {
        (*wa).nonce_count = osip_strdup((*ainfo).nonce_count);
    }
    if !((*ainfo).qop_options).is_null() {
        (*wa).qop_options = osip_strdup((*ainfo).qop_options);
    }
    if !((*ainfo).snum).is_null() {
        (*wa).snum = osip_strdup((*ainfo).snum);
    }
    if !((*ainfo).srand).is_null() {
        (*wa).srand = osip_strdup((*ainfo).srand);
    }
    if !((*ainfo).targetname).is_null() {
        (*wa).targetname = osip_strdup((*ainfo).targetname);
    }
    if !((*ainfo).realm).is_null() {
        (*wa).realm = osip_strdup((*ainfo).realm);
    }
    if !((*ainfo).opaque).is_null() {
        (*wa).opaque = osip_strdup((*ainfo).opaque);
    }
    *dest = wa;
    return 0 as libc::c_int;
}
