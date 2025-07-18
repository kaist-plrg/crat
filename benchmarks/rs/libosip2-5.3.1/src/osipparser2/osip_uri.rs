use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_realloc_func: Option::<osip_realloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
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
    fn osip_clrspace(word: *mut libc::c_char) -> libc::c_int;
    fn osip_clrncpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
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
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_remove(li: *mut osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_list_get_first(
        li: *const osip_list_t,
        it: *mut osip_list_iterator_t,
    ) -> *mut libc::c_void;
    fn osip_list_get_next(it: *mut osip_list_iterator_t) -> *mut libc::c_void;
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_list_eol(li: *const osip_list_t, pos: libc::c_int) -> libc::c_int;
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
pub struct osip_list_iterator {
    pub actual: *mut __node_t,
    pub prev: *mut *mut __node_t,
    pub li: *mut osip_list_t,
    pub pos: libc::c_int,
}
pub type osip_list_iterator_t = osip_list_iterator;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type osip_realloc_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_uri_param {
    pub gname: *mut libc::c_char,
    pub gvalue: *mut libc::c_char,
}
pub type osip_uri_param_t = osip_uri_param;
pub type osip_uri_header_t = osip_uri_param_t;
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
pub unsafe extern "C" fn osip_uri_init(mut url: *mut *mut osip_uri_t) -> libc::c_int {
    *url = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_uri_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_uri_t>() as libc::c_ulong)
    }) as *mut osip_uri_t;
    if (*url).is_null() {
        return -(4 as libc::c_int);
    }
    (**url).scheme = 0 as *mut libc::c_char;
    (**url).username = 0 as *mut libc::c_char;
    (**url).password = 0 as *mut libc::c_char;
    (**url).host = 0 as *mut libc::c_char;
    (**url).port = 0 as *mut libc::c_char;
    osip_list_init(&mut (**url).url_params);
    osip_list_init(&mut (**url).url_headers);
    (**url).string = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn next_separator(
    mut ch: *const libc::c_char,
    mut separator_osip_to_find: libc::c_int,
    mut before_separator: libc::c_int,
) -> *const libc::c_char {
    let mut ind: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    ind = strchr(ch, separator_osip_to_find);
    if ind.is_null() {
        return 0 as *const libc::c_char;
    }
    tmp = 0 as *const libc::c_char;
    if before_separator != 0 as libc::c_int {
        tmp = strchr(ch, before_separator);
    }
    if !tmp.is_null() {
        if ind < tmp {
            return ind;
        }
    } else {
        return ind
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn osip_uri_parse(
    mut url: *mut osip_uri_t,
    mut buf: *const libc::c_char,
) -> libc::c_int {
    let mut username: *const libc::c_char = 0 as *const libc::c_char;
    let mut password: *const libc::c_char = 0 as *const libc::c_char;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: *const libc::c_char = 0 as *const libc::c_char;
    let mut params: *const libc::c_char = 0 as *const libc::c_char;
    let mut headers: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if buf.is_null()
        || *buf.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return -(2 as libc::c_int);
    }
    tmp = strchr(buf, ':' as i32);
    if tmp.is_null() {
        return -(5 as libc::c_int);
    }
    if (tmp.offset_from(buf) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return -(5 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while buf.offset(i as isize) < tmp {
        if !(*buf.offset(i as isize) as libc::c_int >= 'a' as i32
            && *buf.offset(i as isize) as libc::c_int <= 'z' as i32
            || *buf.offset(i as isize) as libc::c_int >= 'A' as i32
                && *buf.offset(i as isize) as libc::c_int <= 'Z' as i32)
        {
            return -(5 as libc::c_int);
        }
        i += 1;
        i;
    }
    (*url)
        .scheme = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (tmp.offset_from(buf) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        )
    } else {
        malloc(
            (tmp.offset_from(buf) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if ((*url).scheme).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy((*url).scheme, buf, tmp.offset_from(buf) as libc::c_long as size_t);
    if !(strchr((*url).scheme, ' ' as i32)).is_null() {
        return -(5 as libc::c_int);
    }
    if strlen((*url).scheme) < 3 as libc::c_int as libc::c_ulong
        || 0 as libc::c_int
            != osip_strncasecmp(
                (*url).scheme,
                b"sip\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            )
            && 0 as libc::c_int
                != osip_strncasecmp(
                    (*url).scheme,
                    b"sips\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                )
    {
        let mut i_0: size_t = strlen(tmp.offset(1 as libc::c_int as isize));
        if i_0 < 2 as libc::c_int as libc::c_ulong {
            return -(5 as libc::c_int);
        }
        (*url)
            .string = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(i_0.wrapping_add(1 as libc::c_int as libc::c_ulong))
        } else {
            malloc(i_0.wrapping_add(1 as libc::c_int as libc::c_ulong))
        }) as *mut libc::c_char;
        if ((*url).string).is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy((*url).string, tmp.offset(1 as libc::c_int as isize), i_0);
        return 0 as libc::c_int;
    }
    username = strchr(buf, ':' as i32);
    if username.is_null() {
        return -(5 as libc::c_int);
    }
    host = strchr(buf, '@' as i32);
    if host.is_null() {
        host = username;
    } else if *username.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32 {
        host = username.offset(1 as libc::c_int as isize);
    } else {
        password = next_separator(
            username.offset(1 as libc::c_int as isize),
            ':' as i32,
            '@' as i32,
        );
        if password.is_null() {
            password = host;
        } else {
            if (host.offset_from(password) as libc::c_long)
                < 2 as libc::c_int as libc::c_long
            {
                return -(5 as libc::c_int);
            }
            (*url)
                .password = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(host.offset_from(password) as libc::c_long as size_t)
            } else {
                malloc(host.offset_from(password) as libc::c_long as libc::c_ulong)
            }) as *mut libc::c_char;
            if ((*url).password).is_null() {
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                (*url).password,
                password.offset(1 as libc::c_int as isize),
                (host.offset_from(password) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
            __osip_uri_unescape((*url).password);
        }
        if (password.offset_from(username) as libc::c_long)
            < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        (*url)
            .username = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(password.offset_from(username) as libc::c_long as size_t)
        } else {
            malloc(password.offset_from(username) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if ((*url).username).is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            (*url).username,
            username.offset(1 as libc::c_int as isize),
            (password.offset_from(username) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        __osip_uri_unescape((*url).username);
    }
    headers = strchr(host, '?' as i32);
    if headers.is_null() {
        headers = buf.offset(strlen(buf) as isize);
    } else {
        osip_uri_parse_headers(url, headers);
    }
    params = strchr(host, ';' as i32);
    if params.is_null() {
        params = headers;
    } else {
        let mut tmpbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        if (headers.offset_from(params) as libc::c_long
            + 1 as libc::c_int as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        tmpbuf = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (headers.offset_from(params) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            )
        } else {
            malloc(
                (headers.offset_from(params) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        if tmpbuf.is_null() {
            return -(4 as libc::c_int);
        }
        tmpbuf = osip_strncpy(
            tmpbuf,
            params,
            headers.offset_from(params) as libc::c_long as size_t,
        );
        osip_uri_parse_params(url, tmpbuf);
        if !tmpbuf.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmpbuf as *mut libc::c_void);
            } else {
                free(tmpbuf as *mut libc::c_void);
            }
        }
    }
    port = params.offset(-(1 as libc::c_int as isize));
    while port > host && *port as libc::c_int != ']' as i32
        && *port as libc::c_int != ':' as i32
    {
        port = port.offset(-1);
        port;
    }
    if *port as libc::c_int == ':' as i32 {
        if host == port {
            port = params;
        } else {
            if (params.offset_from(port) as libc::c_long)
                < 2 as libc::c_int as libc::c_long
                || params.offset_from(port) as libc::c_long
                    > 8 as libc::c_int as libc::c_long
            {
                return -(5 as libc::c_int);
            }
            (*url)
                .port = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(params.offset_from(port) as libc::c_long as size_t)
            } else {
                malloc(params.offset_from(port) as libc::c_long as libc::c_ulong)
            }) as *mut libc::c_char;
            if ((*url).port).is_null() {
                return -(4 as libc::c_int);
            }
            osip_clrncpy(
                (*url).port,
                port.offset(1 as libc::c_int as isize),
                (params.offset_from(port) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
    } else {
        port = params;
    }
    tmp = port;
    while tmp > host && *tmp as libc::c_int != ']' as i32 {
        tmp = tmp.offset(-1);
        tmp;
    }
    if *tmp as libc::c_int == ']' as i32 {
        port = tmp;
        while host < port && *host as libc::c_int != '[' as i32 {
            host = host.offset(1);
            host;
        }
        if host >= port {
            return -(5 as libc::c_int);
        }
    }
    if (port.offset_from(host) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return -(5 as libc::c_int);
    }
    (*url)
        .host = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(port.offset_from(host) as libc::c_long as size_t)
    } else {
        malloc(port.offset_from(host) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if ((*url).host).is_null() {
        return -(4 as libc::c_int);
    }
    osip_clrncpy(
        (*url).host,
        host.offset(1 as libc::c_int as isize),
        (port.offset_from(host) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_set_scheme(
    mut url: *mut osip_uri_t,
    mut scheme: *mut libc::c_char,
) {
    if url.is_null() {
        return;
    }
    (*url).scheme = scheme;
}
pub unsafe extern "C" fn osip_uri_get_scheme(
    mut url: *mut osip_uri_t,
) -> *mut libc::c_char {
    if url.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*url).scheme;
}
pub unsafe extern "C" fn osip_uri_set_username(
    mut url: *mut osip_uri_t,
    mut username: *mut libc::c_char,
) {
    if url.is_null() {
        return;
    }
    (*url).username = username;
}
pub unsafe extern "C" fn osip_uri_get_username(
    mut url: *mut osip_uri_t,
) -> *mut libc::c_char {
    if url.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*url).username;
}
pub unsafe extern "C" fn osip_uri_set_password(
    mut url: *mut osip_uri_t,
    mut password: *mut libc::c_char,
) {
    if url.is_null() {
        return;
    }
    (*url).password = password;
}
pub unsafe extern "C" fn osip_uri_get_password(
    mut url: *mut osip_uri_t,
) -> *mut libc::c_char {
    if url.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*url).password;
}
pub unsafe extern "C" fn osip_uri_set_host(
    mut url: *mut osip_uri_t,
    mut host: *mut libc::c_char,
) {
    if url.is_null() {
        return;
    }
    (*url).host = host;
}
pub unsafe extern "C" fn osip_uri_get_host(
    mut url: *mut osip_uri_t,
) -> *mut libc::c_char {
    if url.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*url).host;
}
pub unsafe extern "C" fn osip_uri_set_port(
    mut url: *mut osip_uri_t,
    mut port: *mut libc::c_char,
) {
    if url.is_null() {
        return;
    }
    (*url).port = port;
}
pub unsafe extern "C" fn osip_uri_get_port(
    mut url: *mut osip_uri_t,
) -> *mut libc::c_char {
    if url.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*url).port;
}
pub unsafe extern "C" fn osip_uri_parse_headers(
    mut url: *mut osip_uri_t,
    mut headers: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut _and: *const libc::c_char = 0 as *const libc::c_char;
    let mut equal: *const libc::c_char = 0 as *const libc::c_char;
    if *headers.offset(0 as libc::c_int as isize) as libc::c_int != '?' as i32 {
        return -(5 as libc::c_int);
    }
    equal = strchr(headers, '=' as i32);
    _and = strchr(headers.offset(1 as libc::c_int as isize), '&' as i32);
    if equal.is_null() {
        return -(5 as libc::c_int);
    }
    loop {
        let mut hname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut hvalue: *mut libc::c_char = 0 as *mut libc::c_char;
        hname = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(equal.offset_from(headers) as libc::c_long as size_t)
        } else {
            malloc(equal.offset_from(headers) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if hname.is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            hname,
            headers.offset(1 as libc::c_int as isize),
            (equal.offset_from(headers) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        __osip_uri_unescape(hname);
        if !_and.is_null() {
            if (_and.offset_from(equal) as libc::c_long)
                < 2 as libc::c_int as libc::c_long
            {
                if !hname.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(hname as *mut libc::c_void);
                    } else {
                        free(hname as *mut libc::c_void);
                    }
                }
                return -(5 as libc::c_int);
            }
            hvalue = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(_and.offset_from(equal) as libc::c_long as size_t)
            } else {
                malloc(_and.offset_from(equal) as libc::c_long as libc::c_ulong)
            }) as *mut libc::c_char;
            if hvalue.is_null() {
                if !hname.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(hname as *mut libc::c_void);
                    } else {
                        free(hname as *mut libc::c_void);
                    }
                }
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                hvalue,
                equal.offset(1 as libc::c_int as isize),
                (_and.offset_from(equal) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
            __osip_uri_unescape(hvalue);
        } else {
            if (headers.offset(strlen(headers) as isize).offset_from(equal)
                as libc::c_long + 1 as libc::c_int as libc::c_long)
                < 2 as libc::c_int as libc::c_long
            {
                if !hname.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(hname as *mut libc::c_void);
                    } else {
                        free(hname as *mut libc::c_void);
                    }
                }
                return -(5 as libc::c_int);
            }
            hvalue = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(
                    (headers.offset(strlen(headers) as isize).offset_from(equal)
                        as libc::c_long + 1 as libc::c_int as libc::c_long) as size_t,
                )
            } else {
                malloc(
                    (headers.offset(strlen(headers) as isize).offset_from(equal)
                        as libc::c_long + 1 as libc::c_int as libc::c_long)
                        as libc::c_ulong,
                )
            }) as *mut libc::c_char;
            if hvalue.is_null() {
                if !hname.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(hname as *mut libc::c_void);
                    } else {
                        free(hname as *mut libc::c_void);
                    }
                }
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                hvalue,
                equal.offset(1 as libc::c_int as isize),
                headers.offset(strlen(headers) as isize).offset_from(equal)
                    as libc::c_long as size_t,
            );
            __osip_uri_unescape(hvalue);
        }
        i = osip_uri_param_add(&mut (*url).url_headers, hname, hvalue);
        if i != 0 as libc::c_int {
            if !hname.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(hname as *mut libc::c_void);
                } else {
                    free(hname as *mut libc::c_void);
                }
            }
            if !hvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(hvalue as *mut libc::c_void);
                } else {
                    free(hvalue as *mut libc::c_void);
                }
            }
            return i;
        }
        if _and.is_null() {
            equal = 0 as *const libc::c_char;
        } else {
            headers = _and;
            equal = strchr(headers, '=' as i32);
            _and = strchr(headers.offset(1 as libc::c_int as isize), '&' as i32);
            if equal.is_null() {
                return -(5 as libc::c_int);
            }
        }
        if equal.is_null() {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_parse_params(
    mut url: *mut osip_uri_t,
    mut params: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comma: *const libc::c_char = 0 as *const libc::c_char;
    let mut equal: *const libc::c_char = 0 as *const libc::c_char;
    if *params.offset(0 as libc::c_int as isize) as libc::c_int != ';' as i32 {
        return -(5 as libc::c_int);
    }
    equal = next_separator(
        params.offset(1 as libc::c_int as isize),
        '=' as i32,
        ';' as i32,
    );
    comma = strchr(params.offset(1 as libc::c_int as isize), ';' as i32);
    while !comma.is_null() {
        if equal.is_null() {
            equal = comma;
            pvalue = 0 as *mut libc::c_char;
        } else {
            if (comma.offset_from(equal) as libc::c_long)
                < 2 as libc::c_int as libc::c_long
            {
                return -(5 as libc::c_int);
            }
            pvalue = (if osip_malloc_func.is_some() {
                osip_malloc_func
                    .unwrap()(comma.offset_from(equal) as libc::c_long as size_t)
            } else {
                malloc(comma.offset_from(equal) as libc::c_long as libc::c_ulong)
            }) as *mut libc::c_char;
            if pvalue.is_null() {
                return -(4 as libc::c_int);
            }
            osip_strncpy(
                pvalue,
                equal.offset(1 as libc::c_int as isize),
                (comma.offset_from(equal) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
            __osip_uri_unescape(pvalue);
        }
        if (equal.offset_from(params) as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        pname = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(equal.offset_from(params) as libc::c_long as size_t)
        } else {
            malloc(equal.offset_from(params) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if pname.is_null() {
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            pname,
            params.offset(1 as libc::c_int as isize),
            (equal.offset_from(params) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        __osip_uri_unescape(pname);
        i = osip_uri_param_add(&mut (*url).url_params, pname, pvalue);
        if i != 0 as libc::c_int {
            if !pname.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pname as *mut libc::c_void);
                } else {
                    free(pname as *mut libc::c_void);
                }
            }
            if !pvalue.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(pvalue as *mut libc::c_void);
                } else {
                    free(pvalue as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        params = comma;
        equal = next_separator(
            params.offset(1 as libc::c_int as isize),
            '=' as i32,
            ';' as i32,
        );
        comma = strchr(params.offset(1 as libc::c_int as isize), ';' as i32);
    }
    comma = params.offset(strlen(params) as isize);
    if equal.is_null() {
        equal = comma;
        pvalue = 0 as *mut libc::c_char;
    } else {
        if (comma.offset_from(equal) as libc::c_long) < 2 as libc::c_int as libc::c_long
        {
            return -(5 as libc::c_int);
        }
        pvalue = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(comma.offset_from(equal) as libc::c_long as size_t)
        } else {
            malloc(comma.offset_from(equal) as libc::c_long as libc::c_ulong)
        }) as *mut libc::c_char;
        if pvalue.is_null() {
            return -(4 as libc::c_int);
        }
        osip_strncpy(
            pvalue,
            equal.offset(1 as libc::c_int as isize),
            (comma.offset_from(equal) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as size_t,
        );
        __osip_uri_unescape(pvalue);
    }
    if (equal.offset_from(params) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        if !pvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pvalue as *mut libc::c_void);
            } else {
                free(pvalue as *mut libc::c_void);
            }
        }
        return -(5 as libc::c_int);
    }
    pname = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(equal.offset_from(params) as libc::c_long as size_t)
    } else {
        malloc(equal.offset_from(params) as libc::c_long as libc::c_ulong)
    }) as *mut libc::c_char;
    if pname.is_null() {
        if !pvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pvalue as *mut libc::c_void);
            } else {
                free(pvalue as *mut libc::c_void);
            }
        }
        return -(4 as libc::c_int);
    }
    osip_strncpy(
        pname,
        params.offset(1 as libc::c_int as isize),
        (equal.offset_from(params) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    __osip_uri_unescape(pname);
    i = osip_uri_param_add(&mut (*url).url_params, pname, pvalue);
    if i != 0 as libc::c_int {
        if !pname.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pname as *mut libc::c_void);
            } else {
                free(pname as *mut libc::c_void);
            }
        }
        if !pvalue.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(pvalue as *mut libc::c_void);
            } else {
                free(pvalue as *mut libc::c_void);
            }
        }
        return -(4 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_to_str(
    mut url: *const osip_uri_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut plen: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scheme: *const libc::c_char = 0 as *const libc::c_char;
    *dest = 0 as *mut libc::c_char;
    if url.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*url).host).is_null() && ((*url).string).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*url).scheme).is_null() && !((*url).string).is_null() {
        return -(2 as libc::c_int);
    }
    if ((*url).string).is_null() && ((*url).scheme).is_null() {
        scheme = b"sip\0" as *const u8 as *const libc::c_char;
    } else {
        scheme = (*url).scheme;
    }
    if !((*url).string).is_null() {
        buf = (if osip_malloc_func.is_some() {
            osip_malloc_func
                .unwrap()(
                (strlen(scheme))
                    .wrapping_add(strlen((*url).string))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
            )
        } else {
            malloc(
                (strlen(scheme))
                    .wrapping_add(strlen((*url).string))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
            )
        }) as *mut libc::c_char;
        if buf.is_null() {
            return -(4 as libc::c_int);
        }
        *dest = buf;
        sprintf(buf, b"%s:\0" as *const u8 as *const libc::c_char, scheme);
        buf = buf.offset(strlen(scheme) as isize).offset(1 as libc::c_int as isize);
        sprintf(buf, b"%s\0" as *const u8 as *const libc::c_char, (*url).string);
        return 0 as libc::c_int;
    }
    len = (strlen(scheme))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen((*url).host))
        .wrapping_add(5 as libc::c_int as libc::c_ulong);
    if !((*url).username).is_null() {
        len = len
            .wrapping_add(
                (strlen((*url).username)).wrapping_mul(3 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if !((*url).password).is_null() {
        len = len
            .wrapping_add(
                (strlen((*url).password)).wrapping_mul(3 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if !((*url).port).is_null() {
        len = len
            .wrapping_add(strlen((*url).port))
            .wrapping_add(3 as libc::c_int as libc::c_ulong);
    }
    buf = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(len)
    } else {
        malloc(len)
    }) as *mut libc::c_char;
    if buf.is_null() {
        return -(4 as libc::c_int);
    }
    tmp = buf;
    sprintf(tmp, b"%s:\0" as *const u8 as *const libc::c_char, scheme);
    tmp = tmp.offset(strlen(tmp) as isize);
    if !((*url).username).is_null() {
        let mut tmp2: *mut libc::c_char = __osip_uri_escape_userinfo((*url).username);
        if tmp2.is_null() {
            if !buf.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(buf as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        sprintf(tmp, b"%s\0" as *const u8 as *const libc::c_char, tmp2);
        if !tmp2.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp2 as *mut libc::c_void);
            } else {
                free(tmp2 as *mut libc::c_void);
            }
        }
        tmp = tmp.offset(strlen(tmp) as isize);
    }
    if !((*url).password).is_null() && !((*url).username).is_null() {
        let mut tmp2_0: *mut libc::c_char = __osip_uri_escape_password((*url).password);
        if tmp2_0.is_null() {
            if !buf.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(buf as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        sprintf(tmp, b":%s\0" as *const u8 as *const libc::c_char, tmp2_0);
        if !tmp2_0.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp2_0 as *mut libc::c_void);
            } else {
                free(tmp2_0 as *mut libc::c_void);
            }
        }
        tmp = tmp.offset(strlen(tmp) as isize);
    }
    if !((*url).username).is_null() {
        sprintf(tmp, b"@\0" as *const u8 as *const libc::c_char);
        tmp = tmp.offset(1);
        tmp;
    }
    if !(strchr((*url).host, ':' as i32)).is_null() {
        sprintf(tmp, b"[%s]\0" as *const u8 as *const libc::c_char, (*url).host);
        tmp = tmp.offset(strlen(tmp) as isize);
    } else {
        sprintf(tmp, b"%s\0" as *const u8 as *const libc::c_char, (*url).host);
        tmp = tmp.offset(strlen(tmp) as isize);
    }
    if !((*url).port).is_null() {
        sprintf(tmp, b":%s\0" as *const u8 as *const libc::c_char, (*url).port);
        tmp = tmp.offset(strlen(tmp) as isize);
    }
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_param: *mut osip_uri_param_t = osip_list_get_first(
        &(*url).url_params,
        &mut it,
    ) as *mut osip_uri_param_t;
    while !u_param.is_null() {
        let mut tmp1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmp2_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut previous_buf: *mut libc::c_char = 0 as *mut libc::c_char;
        if osip_strcasecmp(
            (*u_param).gname,
            b"x-obr\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || osip_strcasecmp(
                (*u_param).gname,
                b"x-obp\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            u_param = osip_list_get_next(&mut it) as *mut osip_uri_param_t;
        } else {
            tmp1 = __osip_uri_escape_uri_param((*u_param).gname);
            if tmp1.is_null() {
                if !buf.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(buf as *mut libc::c_void);
                    } else {
                        free(buf as *mut libc::c_void);
                    }
                }
                return -(5 as libc::c_int);
            }
            if ((*u_param).gvalue).is_null() {
                plen = (strlen(tmp1)).wrapping_add(2 as libc::c_int as libc::c_ulong);
            } else {
                tmp2_1 = __osip_uri_escape_uri_param((*u_param).gvalue);
                if tmp2_1.is_null() {
                    if !tmp1.is_null() {
                        if osip_free_func.is_some() {
                            osip_free_func.unwrap()(tmp1 as *mut libc::c_void);
                        } else {
                            free(tmp1 as *mut libc::c_void);
                        }
                    }
                    if !buf.is_null() {
                        if osip_free_func.is_some() {
                            osip_free_func.unwrap()(buf as *mut libc::c_void);
                        } else {
                            free(buf as *mut libc::c_void);
                        }
                    }
                    return -(5 as libc::c_int);
                }
                plen = (strlen(tmp1))
                    .wrapping_add(strlen(tmp2_1))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong);
            }
            len = len.wrapping_add(plen);
            previous_buf = buf;
            buf = (if osip_realloc_func.is_some() {
                osip_realloc_func.unwrap()(buf as *mut libc::c_void, len)
            } else {
                realloc(buf as *mut libc::c_void, len)
            }) as *mut libc::c_char;
            if buf.is_null() {
                if !previous_buf.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(previous_buf as *mut libc::c_void);
                    } else {
                        free(previous_buf as *mut libc::c_void);
                    }
                }
                if !tmp1.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(tmp1 as *mut libc::c_void);
                    } else {
                        free(tmp1 as *mut libc::c_void);
                    }
                }
                if !tmp2_1.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(tmp2_1 as *mut libc::c_void);
                    } else {
                        free(tmp2_1 as *mut libc::c_void);
                    }
                }
                return -(4 as libc::c_int);
            }
            tmp = buf;
            tmp = tmp.offset(strlen(tmp) as isize);
            if ((*u_param).gvalue).is_null() {
                sprintf(tmp, b";%s\0" as *const u8 as *const libc::c_char, tmp1);
            } else {
                sprintf(
                    tmp,
                    b";%s=%s\0" as *const u8 as *const libc::c_char,
                    tmp1,
                    tmp2_1,
                );
                if !tmp2_1.is_null() {
                    if osip_free_func.is_some() {
                        osip_free_func.unwrap()(tmp2_1 as *mut libc::c_void);
                    } else {
                        free(tmp2_1 as *mut libc::c_void);
                    }
                }
            }
            if !tmp1.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp1 as *mut libc::c_void);
                } else {
                    free(tmp1 as *mut libc::c_void);
                }
            }
            u_param = osip_list_get_next(&mut it) as *mut osip_uri_param_t;
        }
    }
    let mut it_0: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    let mut u_header: *mut osip_uri_header_t = osip_list_get_first(
        &(*url).url_headers,
        &mut it_0,
    ) as *mut osip_uri_header_t;
    while !u_header.is_null() {
        let mut tmp1_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmp2_2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut previous_buf_0: *mut libc::c_char = 0 as *mut libc::c_char;
        tmp1_0 = __osip_uri_escape_header_param((*u_header).gname);
        if tmp1_0.is_null() {
            if !buf.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(buf as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        tmp2_2 = __osip_uri_escape_header_param((*u_header).gvalue);
        if tmp2_2.is_null() {
            if !tmp1_0.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp1_0 as *mut libc::c_void);
                } else {
                    free(tmp1_0 as *mut libc::c_void);
                }
            }
            if !buf.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(buf as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                }
            }
            return -(5 as libc::c_int);
        }
        plen = (strlen(tmp1_0))
            .wrapping_add(strlen(tmp2_2))
            .wrapping_add(4 as libc::c_int as libc::c_ulong);
        len = len.wrapping_add(plen);
        previous_buf_0 = buf;
        buf = (if osip_realloc_func.is_some() {
            osip_realloc_func.unwrap()(buf as *mut libc::c_void, len)
        } else {
            realloc(buf as *mut libc::c_void, len)
        }) as *mut libc::c_char;
        if buf.is_null() {
            if !previous_buf_0.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(previous_buf_0 as *mut libc::c_void);
                } else {
                    free(previous_buf_0 as *mut libc::c_void);
                }
            }
            if !tmp1_0.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp1_0 as *mut libc::c_void);
                } else {
                    free(tmp1_0 as *mut libc::c_void);
                }
            }
            if !tmp2_2.is_null() {
                if osip_free_func.is_some() {
                    osip_free_func.unwrap()(tmp2_2 as *mut libc::c_void);
                } else {
                    free(tmp2_2 as *mut libc::c_void);
                }
            }
            return -(4 as libc::c_int);
        }
        tmp = buf;
        tmp = tmp.offset(strlen(tmp) as isize);
        if it_0.pos == 0 as libc::c_int {
            snprintf(
                tmp,
                len.wrapping_sub(tmp.offset_from(buf) as libc::c_long as libc::c_ulong),
                b"?%s=%s\0" as *const u8 as *const libc::c_char,
                tmp1_0,
                tmp2_2,
            );
        } else {
            snprintf(
                tmp,
                len.wrapping_sub(tmp.offset_from(buf) as libc::c_long as libc::c_ulong),
                b"&%s=%s\0" as *const u8 as *const libc::c_char,
                tmp1_0,
                tmp2_2,
            );
        }
        if !tmp1_0.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp1_0 as *mut libc::c_void);
            } else {
                free(tmp1_0 as *mut libc::c_void);
            }
        }
        if !tmp2_2.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(tmp2_2 as *mut libc::c_void);
            } else {
                free(tmp2_2 as *mut libc::c_void);
            }
        }
        u_header = osip_list_get_next(&mut it_0) as *mut osip_uri_header_t;
    }
    *dest = buf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_free(mut url: *mut osip_uri_t) {
    if url.is_null() {
        return;
    }
    if !((*url).scheme).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url).scheme as *mut libc::c_void);
        } else {
            free((*url).scheme as *mut libc::c_void);
        }
    }
    if !((*url).username).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url).username as *mut libc::c_void);
        } else {
            free((*url).username as *mut libc::c_void);
        }
    }
    if !((*url).password).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url).password as *mut libc::c_void);
        } else {
            free((*url).password as *mut libc::c_void);
        }
    }
    if !((*url).host).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url).host as *mut libc::c_void);
        } else {
            free((*url).host as *mut libc::c_void);
        }
    }
    if !((*url).port).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url).port as *mut libc::c_void);
        } else {
            free((*url).port as *mut libc::c_void);
        }
    }
    osip_uri_param_freelist(&mut (*url).url_params);
    let mut u_header: *mut osip_uri_header_t = 0 as *mut osip_uri_header_t;
    while osip_list_eol(&mut (*url).url_headers, 0 as libc::c_int) == 0 {
        u_header = osip_list_get(&mut (*url).url_headers, 0 as libc::c_int)
            as *mut osip_uri_header_t;
        osip_list_remove(&mut (*url).url_headers, 0 as libc::c_int);
        osip_uri_param_free(u_header);
    }
    if !((*url).string).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url).string as *mut libc::c_void);
        } else {
            free((*url).string as *mut libc::c_void);
        }
    }
    if !url.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(url as *mut libc::c_void);
        } else {
            free(url as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_uri_clone(
    mut url: *const osip_uri_t,
    mut dest: *mut *mut osip_uri_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ur: *mut osip_uri_t = 0 as *mut osip_uri_t;
    *dest = 0 as *mut osip_uri_t;
    if url.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*url).host).is_null() && ((*url).string).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_uri_init(&mut ur);
    if i != 0 as libc::c_int {
        return i;
    }
    if !((*url).scheme).is_null() {
        (*ur).scheme = osip_strdup((*url).scheme);
    }
    if !((*url).username).is_null() {
        (*ur).username = osip_strdup((*url).username);
    }
    if !((*url).password).is_null() {
        (*ur).password = osip_strdup((*url).password);
    }
    if !((*url).host).is_null() {
        (*ur).host = osip_strdup((*url).host);
    }
    if !((*url).port).is_null() {
        (*ur).port = osip_strdup((*url).port);
    }
    if !((*url).string).is_null() {
        (*ur).string = osip_strdup((*url).string);
    }
    i = osip_list_clone(
        &(*url).url_params,
        &mut (*ur).url_params,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_uri_param_t,
                    *mut *mut osip_uri_param_t,
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
                osip_uri_param_clone
                    as unsafe extern "C" fn(
                        *const osip_uri_param_t,
                        *mut *mut osip_uri_param_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_uri_free(ur);
        return i;
    }
    i = osip_list_clone(
        &(*url).url_headers,
        &mut (*ur).url_headers,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const osip_uri_param_t,
                    *mut *mut osip_uri_param_t,
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
                osip_uri_param_clone
                    as unsafe extern "C" fn(
                        *const osip_uri_param_t,
                        *mut *mut osip_uri_param_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if i != 0 as libc::c_int {
        osip_uri_free(ur);
        return i;
    }
    *dest = ur;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_param_init(
    mut url_param: *mut *mut osip_uri_param_t,
) -> libc::c_int {
    *url_param = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_uri_param_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_uri_param_t>() as libc::c_ulong)
    }) as *mut osip_uri_param_t;
    if (*url_param).is_null() {
        return -(4 as libc::c_int);
    }
    (**url_param).gname = 0 as *mut libc::c_char;
    (**url_param).gvalue = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_param_free(mut url_param: *mut osip_uri_param_t) {
    if !((*url_param).gname).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url_param).gname as *mut libc::c_void);
        } else {
            free((*url_param).gname as *mut libc::c_void);
        }
    }
    if !((*url_param).gvalue).is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()((*url_param).gvalue as *mut libc::c_void);
        } else {
            free((*url_param).gvalue as *mut libc::c_void);
        }
    }
    if !url_param.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(url_param as *mut libc::c_void);
        } else {
            free(url_param as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_uri_param_set(
    mut url_param: *mut osip_uri_param_t,
    mut pname: *mut libc::c_char,
    mut pvalue: *mut libc::c_char,
) -> libc::c_int {
    (*url_param).gname = pname;
    osip_clrspace((*url_param).gname);
    (*url_param).gvalue = pvalue;
    if !((*url_param).gvalue).is_null() {
        osip_clrspace((*url_param).gvalue);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_param_add(
    mut url_params: *mut osip_list_t,
    mut pname: *mut libc::c_char,
    mut pvalue: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut url_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
    i = osip_uri_param_init(&mut url_param);
    if i != 0 as libc::c_int {
        return i;
    }
    i = osip_uri_param_set(url_param, pname, pvalue);
    if i != 0 as libc::c_int {
        osip_uri_param_free(url_param);
        return i;
    }
    osip_list_add(url_params, url_param as *mut libc::c_void, -(1 as libc::c_int));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_uri_param_freelist(mut params: *mut osip_list_t) {
    let mut u_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
    while osip_list_eol(params, 0 as libc::c_int) == 0 {
        u_param = osip_list_get(params, 0 as libc::c_int) as *mut osip_uri_param_t;
        osip_list_remove(params, 0 as libc::c_int);
        osip_uri_param_free(u_param);
    }
}
pub unsafe extern "C" fn osip_uri_param_get_byname(
    mut params: *mut osip_list_t,
    mut pname: *mut libc::c_char,
    mut url_param: *mut *mut osip_uri_param_t,
) -> libc::c_int {
    let mut pname_len: size_t = 0;
    let mut u_param: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
    let mut it: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    *url_param = 0 as *mut osip_uri_param_t;
    if pname.is_null() {
        return -(2 as libc::c_int);
    }
    pname_len = strlen(pname);
    if pname_len <= 0 as libc::c_int as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    u_param = osip_list_get_first(params, &mut it) as *mut osip_uri_param_t;
    while !u_param.is_null() {
        let mut len: size_t = 0;
        len = strlen((*u_param).gname);
        if pname_len == len
            && osip_strncasecmp((*u_param).gname, pname, strlen(pname))
                == 0 as libc::c_int
        {
            *url_param = u_param;
            return 0 as libc::c_int;
        }
        u_param = osip_list_get_next(&mut it) as *mut osip_uri_param_t;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn osip_uri_param_clone(
    mut uparam: *const osip_uri_param_t,
    mut dest: *mut *mut osip_uri_param_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut up: *mut osip_uri_param_t = 0 as *mut osip_uri_param_t;
    *dest = 0 as *mut osip_uri_param_t;
    if uparam.is_null() {
        return -(2 as libc::c_int);
    }
    if ((*uparam).gname).is_null() {
        return -(2 as libc::c_int);
    }
    i = osip_uri_param_init(&mut up);
    if i != 0 as libc::c_int {
        return i;
    }
    (*up).gname = osip_strdup((*uparam).gname);
    if !((*uparam).gvalue).is_null() {
        (*up).gvalue = osip_strdup((*uparam).gvalue);
    } else {
        (*up).gvalue = 0 as *mut libc::c_char;
    }
    *dest = up;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_uri_escape_nonascii_and_nondef(
    mut string: *const libc::c_char,
    mut def: *const libc::c_char,
) -> *mut libc::c_char {
    let mut alloc: size_t = (strlen(string))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut length: size_t = 0;
    let mut ns: *mut libc::c_char = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(alloc)
    } else {
        malloc(alloc)
    }) as *mut libc::c_char;
    let mut in_0: libc::c_uchar = 0;
    let mut newlen: size_t = alloc;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if ns.is_null() {
        return 0 as *mut libc::c_char;
    }
    length = alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let fresh0 = length;
        length = length.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        in_0 = *string as libc::c_uchar;
        i = 0 as libc::c_int;
        tmp = 0 as *const libc::c_char;
        if in_0 as libc::c_int >= 'a' as i32 && in_0 as libc::c_int <= 'z' as i32
            || in_0 as libc::c_int >= 'A' as i32 && in_0 as libc::c_int <= 'Z' as i32
            || in_0 as libc::c_int >= '0' as i32 && in_0 as libc::c_int <= '9' as i32
        {
            tmp = string;
        } else {
            while *def.offset(i as isize) as libc::c_int != '\0' as i32
                && *def.offset(i as isize) as libc::c_int != in_0 as libc::c_int
            {
                i += 1;
                i;
            }
            if *def.offset(i as isize) as libc::c_int != '\0' as i32 {
                tmp = string;
            }
        }
        if tmp.is_null() {
            newlen = (newlen as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if newlen > alloc {
                let mut previous_ns: *mut libc::c_char = 0 as *mut libc::c_char;
                alloc = (alloc as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                previous_ns = ns;
                ns = (if osip_realloc_func.is_some() {
                    osip_realloc_func.unwrap()(ns as *mut libc::c_void, alloc)
                } else {
                    realloc(ns as *mut libc::c_void, alloc)
                }) as *mut libc::c_char;
                if ns.is_null() {
                    if !previous_ns.is_null() {
                        if osip_free_func.is_some() {
                            osip_free_func.unwrap()(previous_ns as *mut libc::c_void);
                        } else {
                            free(previous_ns as *mut libc::c_void);
                        }
                    }
                    return 0 as *mut libc::c_char;
                }
            }
            sprintf(
                &mut *ns.offset(index as isize) as *mut libc::c_char,
                b"%%%02X\0" as *const u8 as *const libc::c_char,
                in_0 as libc::c_int,
            );
            index += 3 as libc::c_int;
        } else {
            let fresh1 = index;
            index = index + 1;
            *ns.offset(fresh1 as isize) = in_0 as libc::c_char;
        }
        string = string.offset(1);
        string;
    }
    *ns.offset(index as isize) = 0 as libc::c_int as libc::c_char;
    return ns;
}
pub static mut userinfo_def: *const libc::c_char = b"-_.!~*'()&=+$,;?/\0\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn __osip_uri_escape_userinfo(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    return __osip_uri_escape_nonascii_and_nondef(string, userinfo_def);
}
pub static mut password_def: *const libc::c_char = b"-_.!~*'()&=+$,\0\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn __osip_uri_escape_password(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    return __osip_uri_escape_nonascii_and_nondef(string, password_def);
}
pub static mut uri_param_def: *const libc::c_char = b"-_.!~*'()[]/:&+$\0\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn __osip_uri_escape_uri_param(
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    return __osip_uri_escape_nonascii_and_nondef(string, uri_param_def);
}
pub static mut header_param_def: *const libc::c_char = b"-_.!~*'()[]/?:+$\0\0"
    as *const u8 as *const libc::c_char;
pub unsafe extern "C" fn __osip_uri_escape_header_param(
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    return __osip_uri_escape_nonascii_and_nondef(string, header_param_def);
}
pub unsafe extern "C" fn __osip_uri_unescape(mut string: *mut libc::c_char) {
    let mut alloc: size_t = (strlen(string))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut in_0: libc::c_uchar = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut hex: libc::c_uint = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = string;
    loop {
        alloc = alloc.wrapping_sub(1);
        if !(alloc > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        in_0 = *ptr as libc::c_uchar;
        if '%' as i32 == in_0 as libc::c_int {
            if !(alloc > 2 as libc::c_int as libc::c_ulong
                && sscanf(
                    ptr.offset(1 as libc::c_int as isize),
                    b"%02X\0" as *const u8 as *const libc::c_char,
                    &mut hex as *mut libc::c_uint,
                ) == 1 as libc::c_int)
            {
                break;
            }
            in_0 = hex as libc::c_uchar;
            if *ptr.offset(2 as libc::c_int as isize) as libc::c_int != 0
                && (*ptr.offset(2 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *ptr.offset(2 as libc::c_int as isize) as libc::c_int
                        <= '9' as i32
                    || *ptr.offset(2 as libc::c_int as isize) as libc::c_int
                        >= 'a' as i32
                        && *ptr.offset(2 as libc::c_int as isize) as libc::c_int
                            <= 'f' as i32
                    || *ptr.offset(2 as libc::c_int as isize) as libc::c_int
                        >= 'A' as i32
                        && *ptr.offset(2 as libc::c_int as isize) as libc::c_int
                            <= 'F' as i32)
            {
                alloc = (alloc as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                ptr = ptr.offset(2 as libc::c_int as isize);
            } else {
                alloc = (alloc as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                ptr = ptr.offset(1 as libc::c_int as isize);
            }
        }
        let fresh2 = index;
        index = index + 1;
        *string.offset(fresh2 as isize) = in_0 as libc::c_char;
        ptr = ptr.offset(1);
        ptr;
    }
    *string.offset(index as isize) = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn osip_uri_to_str_canonical(
    mut url: *const osip_uri_t,
    mut dest: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    *dest = 0 as *mut libc::c_char;
    result = osip_uri_to_str(url, dest);
    if result == 0 as libc::c_int {
        __osip_uri_unescape(*dest);
    }
    return result;
}
