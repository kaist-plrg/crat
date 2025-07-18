use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
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
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
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
pub unsafe extern "C" fn osip_www_authenticate_init(
    mut dest: *mut *mut osip_www_authenticate_t,
) -> libc::c_int {
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_www_authenticate_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_www_authenticate_t>() as libc::c_ulong)
    }) as *mut osip_www_authenticate_t;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    memset(
        *dest as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_www_authenticate_t>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_message_set_www_authenticate(
    mut sip: *mut osip_message_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut www_authenticate: *mut osip_www_authenticate_t = 0
        as *mut osip_www_authenticate_t;
    let mut i: libc::c_int = 0;
    if hvalue.is_null()
        || *hvalue.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if sip.is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_www_authenticate_init(&mut www_authenticate);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_www_authenticate_parse(www_authenticate, hvalue);
    if i != 0 as libc::c_int {
        osip_www_authenticate_free(www_authenticate);
        return i;
    }
    (*sip).message_property = 2 as libc::c_int;
    osip_list_add(
        &mut (*sip).www_authenticates,
        www_authenticate as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_quoted_string_set(
    mut name: *const libc::c_char,
    mut str: *const libc::c_char,
    mut result: *mut *mut libc::c_char,
    mut next: *mut *const libc::c_char,
) -> libc::c_int {
    *next = str;
    if !(*result).is_null() {
        return 0 as libc::c_int;
    }
    *next = 0 as *const libc::c_char;
    while ' ' as i32 == *str as libc::c_int || '\t' as i32 == *str as libc::c_int
        || ',' as i32 == *str as libc::c_int
    {
        if *str != 0 {
            str = str.offset(1);
            str;
        } else {
            return -(5 as libc::c_int)
        }
    }
    if osip_strncasecmp(name, str, strlen(name)) == 0 as libc::c_int {
        let mut quote1: *const libc::c_char = 0 as *const libc::c_char;
        let mut quote2: *const libc::c_char = 0 as *const libc::c_char;
        let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
        let mut hack: *const libc::c_char = strchr(str, '=' as i32);
        if hack.is_null() {
            return -(5 as libc::c_int);
        }
        while ' ' as i32 == *hack.offset(-(1 as libc::c_int as isize)) as libc::c_int {
            hack = hack.offset(-1);
            hack;
        }
        if hack.offset_from(str) as libc::c_long as size_t != strlen(name) {
            *next = str;
            return 0 as libc::c_int;
        }
        quote1 = __osip_quote_find(str);
        if quote1.is_null() {
            return -(5 as libc::c_int);
        }
        quote2 = __osip_quote_find(quote1.offset(1 as libc::c_int as isize));
        if quote2.is_null() {
            return -(5 as libc::c_int);
        }
        if quote2.offset_from(quote1) as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            tmp = quote2.offset(1 as libc::c_int as isize);
            tmp = tmp
                .offset(
                    strspn(tmp, b" \t\0" as *const u8 as *const libc::c_char) as isize,
                );
            tmp = tmp
                .offset(
                    strspn(tmp, b"\n\r\0" as *const u8 as *const libc::c_char) as isize,
                );
            *next = 0 as *const libc::c_char;
            if *tmp as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            if *tmp as libc::c_int != '\t' as i32 && *tmp as libc::c_int != ' ' as i32 {
                *next = tmp;
            } else {
                tmp = tmp
                    .offset(
                        strspn(tmp, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
                if *tmp as libc::c_int == '\0' as i32 {
                    return 0 as libc::c_int;
                }
                *next = tmp;
            }
            return 0 as libc::c_int;
        }
        *result = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (quote2.offset_from(quote1) as libc::c_long
                    + 3 as libc::c_int as libc::c_long) as size_t,
            )
        } else {
            malloc(
                (quote2.offset_from(quote1) as libc::c_long
                    + 3 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        if (*result).is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            *result,
            quote1,
            (quote2.offset_from(quote1) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
        );
        tmp = quote2.offset(1 as libc::c_int as isize);
        tmp = tmp
            .offset(strspn(tmp, b" \t\0" as *const u8 as *const libc::c_char) as isize);
        tmp = tmp
            .offset(strspn(tmp, b"\n\r\0" as *const u8 as *const libc::c_char) as isize);
        *next = 0 as *const libc::c_char;
        if *tmp as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int;
        }
        if *tmp as libc::c_int != '\t' as i32 && *tmp as libc::c_int != ' ' as i32 {
            *next = tmp;
        } else {
            tmp = tmp
                .offset(
                    strspn(tmp, b" \t\0" as *const u8 as *const libc::c_char) as isize,
                );
            if *tmp as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            *next = tmp;
        }
    } else {
        *next = str;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_token_set(
    mut name: *const libc::c_char,
    mut str: *const libc::c_char,
    mut result: *mut *mut libc::c_char,
    mut next: *mut *const libc::c_char,
) -> libc::c_int {
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    *next = str;
    if !(*result).is_null() {
        return 0 as libc::c_int;
    }
    *next = 0 as *const libc::c_char;
    beg = strchr(str, '=' as i32);
    if beg.is_null() {
        return -(5 as libc::c_int);
    }
    if strlen(str) < 6 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ' ' as i32 == *str as libc::c_int || '\t' as i32 == *str as libc::c_int
        || ',' as i32 == *str as libc::c_int
    {
        if *str != 0 {
            str = str.offset(1);
            str;
        } else {
            return -(5 as libc::c_int)
        }
    }
    if osip_strncasecmp(name, str, strlen(name)) == 0 as libc::c_int {
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        end = strchr(str, ',' as i32);
        if end.is_null() {
            end = str.offset(strlen(str) as isize);
        }
        if (end.offset_from(beg) as libc::c_long) < 2 as libc::c_int as libc::c_long {
            return -(5 as libc::c_int);
        }
        *result = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(end.offset_from(beg) as libc::c_long as size_t)
        } else {
            malloc(end.offset_from(beg) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if (*result).is_null() {
            return -(4 as libc::c_int);
        }
        osip_clrncpy(
            *result,
            beg.offset(1 as libc::c_int as isize),
            (end.offset_from(beg) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as size_t,
        );
        tmp = if *end as libc::c_int != 0 {
            end.offset(1 as libc::c_int as isize)
        } else {
            end
        };
        tmp = tmp
            .offset(strspn(tmp, b" \t\0" as *const u8 as *const libc::c_char) as isize);
        tmp = tmp
            .offset(strspn(tmp, b"\n\r\0" as *const u8 as *const libc::c_char) as isize);
        *next = 0 as *const libc::c_char;
        if *tmp as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int;
        }
        if *tmp as libc::c_int != '\t' as i32 && *tmp as libc::c_int != ' ' as i32 {
            *next = tmp;
        } else {
            tmp = tmp
                .offset(
                    strspn(tmp, b" \t\0" as *const u8 as *const libc::c_char) as isize,
                );
            if *tmp as libc::c_int == '\0' as i32 {
                return 0 as libc::c_int;
            }
            *next = tmp;
        }
    } else {
        *next = str;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_www_authenticate_parse(
    mut wwwa: *mut osip_www_authenticate_t,
    mut hvalue: *const libc::c_char,
) -> libc::c_int {
    let mut space: *const libc::c_char = 0 as *const libc::c_char;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    space = strchr(hvalue, ' ' as i32);
    if space.is_null() {
        return -(5 as libc::c_int);
    }
    if (space.offset_from(hvalue) as libc::c_long + 1 as libc::c_int as libc::c_long)
        < 2 as libc::c_int as libc::c_long
    {
        return -(5 as libc::c_int);
    }
    (*wwwa)
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
    if ((*wwwa).auth_type).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        (*wwwa).auth_type,
        hvalue,
        space.offset_from(hvalue) as libc::c_long as size_t,
    );
    loop {
        let mut parse_ok: libc::c_int = 0 as libc::c_int;
        i = __osip_quoted_string_set(
            b"realm\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*wwwa).realm,
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
            b"domain\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*wwwa).domain,
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
            &mut (*wwwa).nonce,
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
            &mut (*wwwa).opaque,
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
            b"stale\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*wwwa).stale,
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
            &mut (*wwwa).algorithm,
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
            b"qop\0" as *const u8 as *const libc::c_char,
            space,
            &mut (*wwwa).qop_options,
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
            &mut (*wwwa).version,
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
            &mut (*wwwa).targetname,
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
            &mut (*wwwa).gssapi_data,
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
pub unsafe extern "C" fn osip_message_get_www_authenticate(
    mut sip: *const osip_message_t,
    mut pos: libc::c_int,
    mut dest: *mut *mut osip_www_authenticate_t,
) -> libc::c_int {
    let mut www_authenticate: *mut osip_www_authenticate_t = 0
        as *mut osip_www_authenticate_t;
    *dest = 0 as *mut osip_www_authenticate_t;
    if osip_list_size(&(*sip).www_authenticates) <= pos {
        return -(1 as libc::c_int);
    }
    www_authenticate = osip_list_get(&(*sip).www_authenticates, pos)
        as *mut osip_www_authenticate_t;
    *dest = www_authenticate;
    return pos;
}
pub unsafe extern "C" fn osip_www_authenticate_get_auth_type(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).auth_type;
}
pub unsafe extern "C" fn osip_www_authenticate_set_auth_type(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut auth_type: *mut libc::c_char,
) {
    (*www_authenticate).auth_type = auth_type;
}
pub unsafe extern "C" fn osip_www_authenticate_get_realm(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).realm;
}
pub unsafe extern "C" fn osip_www_authenticate_set_realm(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut realm: *mut libc::c_char,
) {
    (*www_authenticate).realm = realm;
}
pub unsafe extern "C" fn osip_www_authenticate_get_domain(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).domain;
}
pub unsafe extern "C" fn osip_www_authenticate_set_domain(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut domain: *mut libc::c_char,
) {
    (*www_authenticate).domain = domain;
}
pub unsafe extern "C" fn osip_www_authenticate_get_nonce(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).nonce;
}
pub unsafe extern "C" fn osip_www_authenticate_set_nonce(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut nonce: *mut libc::c_char,
) {
    (*www_authenticate).nonce = nonce;
}
pub unsafe extern "C" fn osip_www_authenticate_get_stale(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).stale;
}
pub unsafe extern "C" fn osip_www_authenticate_set_stale(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut stale: *mut libc::c_char,
) {
    (*www_authenticate).stale = stale;
}
pub unsafe extern "C" fn osip_www_authenticate_get_opaque(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).opaque;
}
pub unsafe extern "C" fn osip_www_authenticate_set_opaque(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut opaque: *mut libc::c_char,
) {
    (*www_authenticate).opaque = opaque;
}
pub unsafe extern "C" fn osip_www_authenticate_get_algorithm(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).algorithm;
}
pub unsafe extern "C" fn osip_www_authenticate_set_algorithm(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut algorithm: *mut libc::c_char,
) {
    (*www_authenticate).algorithm = algorithm;
}
pub unsafe extern "C" fn osip_www_authenticate_get_qop_options(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).qop_options;
}
pub unsafe extern "C" fn osip_www_authenticate_set_qop_options(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut qop_options: *mut libc::c_char,
) {
    (*www_authenticate).qop_options = qop_options;
}
pub unsafe extern "C" fn osip_www_authenticate_get_version(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).version;
}
pub unsafe extern "C" fn osip_www_authenticate_set_version(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut version: *mut libc::c_char,
) {
    (*www_authenticate).version = version;
}
pub unsafe extern "C" fn osip_www_authenticate_get_targetname(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).targetname;
}
pub unsafe extern "C" fn osip_www_authenticate_set_targetname(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut targetname: *mut libc::c_char,
) {
    (*www_authenticate).targetname = targetname;
}
pub unsafe extern "C" fn osip_www_authenticate_get_gssapi_data(
    mut www_authenticate: *mut osip_www_authenticate_t,
) -> *mut libc::c_char {
    return (*www_authenticate).gssapi_data;
}
pub unsafe extern "C" fn osip_www_authenticate_set_gssapi_data(
    mut www_authenticate: *mut osip_www_authenticate_t,
    mut gssapi_data: *mut libc::c_char,
) {
    (*www_authenticate).gssapi_data = gssapi_data;
}
pub unsafe extern "C" fn osip_www_authenticate_to_str(
    mut wwwa: *const osip_www_authenticate_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    *dest = 0 as *mut libc::c_char;
    if wwwa.is_null() || ((*wwwa).auth_type).is_null() {
        return -(2 as libc::c_int);
    }
    len = (strlen((*wwwa).auth_type)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !((*wwwa).realm).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).realm))
            .wrapping_add(7 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).nonce).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).nonce))
            .wrapping_add(8 as libc::c_int as libc::c_ulong);
    }
    len = len.wrapping_add(2 as libc::c_int as libc::c_ulong);
    if !((*wwwa).domain).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).domain))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).opaque).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).opaque))
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).stale).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).stale))
            .wrapping_add(8 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).algorithm).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).algorithm))
            .wrapping_add(12 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).qop_options).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).qop_options))
            .wrapping_add(6 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).version).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).version))
            .wrapping_add(10 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).targetname).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).targetname))
            .wrapping_add(13 as libc::c_int as libc::c_ulong);
    }
    if !((*wwwa).gssapi_data).is_null() {
        len = len
            .wrapping_add(strlen((*wwwa).gssapi_data))
            .wrapping_add(14 as libc::c_int as libc::c_ulong);
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
    tmp = osip_str_append(tmp, (*wwwa).auth_type);
    if !((*wwwa).realm).is_null() {
        tmp = osip_strn_append(
            tmp,
            b" realm=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).realm);
    }
    if !((*wwwa).domain).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", domain=\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).domain);
    }
    if !((*wwwa).nonce).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", nonce=\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).nonce);
    }
    if !((*wwwa).opaque).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", opaque=\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).opaque);
    }
    if !((*wwwa).stale).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", stale=\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).stale);
    }
    if !((*wwwa).algorithm).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", algorithm=\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).algorithm);
    }
    if !((*wwwa).qop_options).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", qop=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).qop_options);
    }
    if !((*wwwa).version).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", version=\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).version);
    }
    if !((*wwwa).targetname).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", targetname=\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).targetname);
    }
    if !((*wwwa).gssapi_data).is_null() {
        tmp = osip_strn_append(
            tmp,
            b", gssapi-data=\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as size_t,
        );
        tmp = osip_str_append(tmp, (*wwwa).gssapi_data);
    }
    if ((*wwwa).realm).is_null() {
        len = strlen((*wwwa).auth_type);
        if *(*dest).offset(len as isize) as libc::c_int == ',' as i32 {
            *(*dest).offset(len as isize) = ' ' as i32 as libc::c_char;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_www_authenticate_free(
    mut www_authenticate: *mut osip_www_authenticate_t,
) {
    if www_authenticate.is_null() {
        return;
    }
    if !((*www_authenticate).auth_type).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).auth_type as *mut libc::c_void);
        } else {
            free((*www_authenticate).auth_type as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).realm).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).realm as *mut libc::c_void);
        } else {
            free((*www_authenticate).realm as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).domain).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).domain as *mut libc::c_void);
        } else {
            free((*www_authenticate).domain as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).nonce).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).nonce as *mut libc::c_void);
        } else {
            free((*www_authenticate).nonce as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).opaque).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).opaque as *mut libc::c_void);
        } else {
            free((*www_authenticate).opaque as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).stale).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).stale as *mut libc::c_void);
        } else {
            free((*www_authenticate).stale as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).algorithm).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).algorithm as *mut libc::c_void);
        } else {
            free((*www_authenticate).algorithm as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).qop_options).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*www_authenticate).qop_options as *mut libc::c_void);
        } else {
            free((*www_authenticate).qop_options as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).version).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).version as *mut libc::c_void);
        } else {
            free((*www_authenticate).version as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).targetname).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*www_authenticate).targetname as *mut libc::c_void);
        } else {
            free((*www_authenticate).targetname as *mut libc::c_void);
        }
    }
    if !((*www_authenticate).gssapi_data).is_null() {
        if osip_free_func.is_some() {
            osip_free_func
                .unwrap()((*www_authenticate).gssapi_data as *mut libc::c_void);
        } else {
            free((*www_authenticate).gssapi_data as *mut libc::c_void);
        }
    }
    if !www_authenticate.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(www_authenticate as *mut libc::c_void);
        } else {
            free(www_authenticate as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_www_authenticate_clone(
    mut wwwa: *const osip_www_authenticate_t,
    mut dest: *mut *mut osip_www_authenticate_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut wa: *mut osip_www_authenticate_t = 0 as *mut osip_www_authenticate_t;
    *dest = 0 as *mut osip_www_authenticate_t;
    if wwwa.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*wwwa).auth_type).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_www_authenticate_init(&mut wa);
    if i != 0 as libc::c_int {
        return i;
    }
    (*wa).auth_type = osip_strdup((*wwwa).auth_type);
    if ((*wa).auth_type).is_null() && !((*wwwa).auth_type).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).realm).is_null() {
        (*wa).realm = osip_strdup((*wwwa).realm);
    }
    if ((*wa).realm).is_null() && !((*wwwa).realm).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).domain).is_null() {
        (*wa).domain = osip_strdup((*wwwa).domain);
    }
    if ((*wa).domain).is_null() && !((*wwwa).domain).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).nonce).is_null() {
        (*wa).nonce = osip_strdup((*wwwa).nonce);
    }
    if ((*wa).nonce).is_null() && !((*wwwa).nonce).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).opaque).is_null() {
        (*wa).opaque = osip_strdup((*wwwa).opaque);
    }
    if ((*wa).opaque).is_null() && !((*wwwa).opaque).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).stale).is_null() {
        (*wa).stale = osip_strdup((*wwwa).stale);
    }
    if ((*wa).stale).is_null() && !((*wwwa).stale).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).algorithm).is_null() {
        (*wa).algorithm = osip_strdup((*wwwa).algorithm);
    }
    if ((*wa).algorithm).is_null() && !((*wwwa).algorithm).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).qop_options).is_null() {
        (*wa).qop_options = osip_strdup((*wwwa).qop_options);
    }
    if ((*wa).qop_options).is_null() && !((*wwwa).qop_options).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).version).is_null() {
        (*wa).version = osip_strdup((*wwwa).version);
    }
    if ((*wa).version).is_null() && !((*wwwa).version).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).targetname).is_null() {
        (*wa).targetname = osip_strdup((*wwwa).targetname);
    }
    if ((*wa).targetname).is_null() && !((*wwwa).targetname).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    if !((*wwwa).gssapi_data).is_null() {
        (*wa).gssapi_data = osip_strdup((*wwwa).gssapi_data);
    }
    if ((*wa).gssapi_data).is_null() && !((*wwwa).gssapi_data).is_null() {
        osip_www_authenticate_free(wa);
        return -(4 as libc::c_int);
    }
    *dest = wa;
    return 0 as libc::c_int;
}
