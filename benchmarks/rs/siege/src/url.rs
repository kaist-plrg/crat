use ::libc;
extern "C" {
    pub type ARRAY_T;
    pub type AUTH_T;
    pub type COOKIES_T;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut my: CONFIG;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn load_file(U_0: URL, filename: *mut libc::c_char);
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn empty(s: *const libc::c_char) -> BOOLEAN;
    fn startswith(pre: *const libc::c_char, str: *const libc::c_char) -> BOOLEAN;
    fn endswith(suffix: *const libc::c_char, str: *const libc::c_char) -> BOOLEAN;
    fn stristr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xstrcat(arg1: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct URL_T {
    pub ID: libc::c_int,
    pub url: *mut libc::c_char,
    pub scheme: SCHEME,
    pub method: METHOD,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub hostname: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub params: *mut libc::c_char,
    pub hasparams: BOOLEAN,
    pub query: *mut libc::c_char,
    pub frag: *mut libc::c_char,
    pub request: *mut libc::c_char,
    pub postlen: size_t,
    pub postdata: *mut libc::c_char,
    pub posttemp: *mut libc::c_char,
    pub conttype: *mut libc::c_char,
    pub cached: BOOLEAN,
    pub redir: BOOLEAN,
}
pub type METHOD = libc::c_uint;
pub const PATCH: METHOD = 9;
pub const CONNECT: METHOD = 8;
pub const OPTIONS: METHOD = 7;
pub const TRACE: METHOD = 6;
pub const DELETE: METHOD = 5;
pub const PUT: METHOD = 4;
pub const POST: METHOD = 3;
pub const GET: METHOD = 2;
pub const HEAD: METHOD = 1;
pub const NOMETHOD: METHOD = 0;
pub type SCHEME = libc::c_uint;
pub const PROXY: SCHEME = 4;
pub const FTP: SCHEME = 3;
pub const HTTPS: SCHEME = 2;
pub const HTTP: SCHEME = 1;
pub const UNSUPPORTED: SCHEME = 0;
pub type URL = *mut URL_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONFIG {
    pub logging: BOOLEAN,
    pub shlog: BOOLEAN,
    pub limit: libc::c_int,
    pub url: *mut libc::c_char,
    pub logfile: [libc::c_char; 128],
    pub verbose: BOOLEAN,
    pub quiet: BOOLEAN,
    pub parser: BOOLEAN,
    pub csv: BOOLEAN,
    pub fullurl: BOOLEAN,
    pub display: BOOLEAN,
    pub config: BOOLEAN,
    pub color: BOOLEAN,
    pub cusers: libc::c_int,
    pub delay: libc::c_float,
    pub timeout: libc::c_int,
    pub bench: BOOLEAN,
    pub internet: BOOLEAN,
    pub timestamp: BOOLEAN,
    pub time: libc::c_int,
    pub secs: libc::c_int,
    pub reps: libc::c_int,
    pub file: [libc::c_char; 128],
    pub length: libc::c_int,
    pub nomap: *mut LINES,
    pub debug: BOOLEAN,
    pub chunked: BOOLEAN,
    pub unique: BOOLEAN,
    pub get: BOOLEAN,
    pub print: BOOLEAN,
    pub mark: BOOLEAN,
    pub markstr: *mut libc::c_char,
    pub protocol: libc::c_int,
    pub cookies: COOKIES,
    pub uagent: [libc::c_char; 256],
    pub encoding: [libc::c_char; 256],
    pub conttype: [libc::c_char; 256],
    pub bids: libc::c_int,
    pub auth: AUTH,
    pub keepalive: BOOLEAN,
    pub signaled: libc::c_int,
    pub extra: [libc::c_char; 2048],
    pub login: BOOLEAN,
    pub loginurl: *mut libc::c_char,
    pub lurl: ARRAY,
    pub failures: libc::c_int,
    pub failed: libc::c_int,
    pub escape: BOOLEAN,
    pub expire: BOOLEAN,
    pub follow: BOOLEAN,
    pub zero_ok: BOOLEAN,
    pub spinner: BOOLEAN,
    pub cache: BOOLEAN,
    pub rc: [libc::c_char; 256],
    pub ssl_timeout: libc::c_int,
    pub ssl_cert: *mut libc::c_char,
    pub ssl_key: *mut libc::c_char,
    pub ssl_ciphers: *mut libc::c_char,
    pub method: METHOD,
    pub json_output: BOOLEAN,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
}
pub type ARRAY = *mut ARRAY_T;
pub type AUTH = *mut AUTH_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
pub const CM_PASSTHROUGH: copy_method = 2;
pub const CM_DECODE: copy_method = 0;
pub const CM_ENCODE: copy_method = 1;
pub type copy_method = libc::c_uint;
pub const urlchr_reserved: C2RustUnnamed_4 = 1;
pub const urlchr_unsafe: C2RustUnnamed_4 = 2;
pub type C2RustUnnamed_4 = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut URLSIZE: size_t = ::std::mem::size_of::<URL_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_url(mut str: *mut libc::c_char) -> URL {
    let mut this: URL = 0 as *mut URL_T;
    this = xmalloc(URLSIZE) as URL;
    (*this).ID = 0 as libc::c_int;
    (*this).scheme = HTTP;
    (*this).hasparams = boolean_false;
    (*this).params = 0 as *mut libc::c_char;
    (*this).redir = boolean_false;
    (*this).method = GET;
    (*this).username = 0 as *mut libc::c_char;
    (*this).password = 0 as *mut libc::c_char;
    (*this).hostname = 0 as *mut libc::c_char;
    (*this).port = 80 as libc::c_int;
    (*this).path = 0 as *mut libc::c_char;
    (*this).file = 0 as *mut libc::c_char;
    (*this).params = 0 as *mut libc::c_char;
    (*this).hasparams = boolean_false;
    (*this).query = 0 as *mut libc::c_char;
    (*this).frag = 0 as *mut libc::c_char;
    (*this).request = 0 as *mut libc::c_char;
    (*this).postlen = 0 as libc::c_int as size_t;
    (*this).postdata = 0 as *mut libc::c_char;
    (*this).posttemp = 0 as *mut libc::c_char;
    (*this).conttype = 0 as *mut libc::c_char;
    (*this).cached = boolean_false;
    (*this).redir = boolean_false;
    __url_parse(this, str);
    return this;
}
pub unsafe extern "C" fn url_destroy(mut this: URL) -> URL {
    if !this.is_null() {
        xfree((*this).url as *mut libc::c_void);
        xfree((*this).username as *mut libc::c_void);
        xfree((*this).password as *mut libc::c_void);
        xfree((*this).hostname as *mut libc::c_void);
        if !((*this).path).is_null()
            && *((*this).path).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
        {
            xfree((*this).path as *mut libc::c_void);
        }
        xfree((*this).file as *mut libc::c_void);
        xfree((*this).query as *mut libc::c_void);
        xfree((*this).frag as *mut libc::c_void);
        xfree((*this).request as *mut libc::c_void);
        xfree((*this).conttype as *mut libc::c_void);
        xfree((*this).postdata as *mut libc::c_void);
        xfree((*this).posttemp as *mut libc::c_void);
        if (*this).hasparams as libc::c_uint
            == boolean_true as libc::c_int as libc::c_uint
        {
            xfree((*this).params as *mut libc::c_void);
        }
        xfree(this as *mut libc::c_void);
    }
    return 0 as URL;
}
pub unsafe extern "C" fn url_set_ID(mut this: URL, mut ID: libc::c_int) {
    (*this).ID = ID;
}
pub unsafe extern "C" fn url_set_scheme(mut this: URL, mut scheme: SCHEME) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    (*this).scheme = scheme;
    str = strdup(url_get_scheme_name(this));
    if !((*this).url).is_null() {
        tmp = xstrdup((*this).url);
        if strncasecmp(
            tmp,
            b"http:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            n = 7 as libc::c_int;
        }
        if strncasecmp(
            tmp,
            b"https:\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            n = 8 as libc::c_int;
        }
        if strncasecmp(
            tmp,
            b"ftp:\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            n = 6 as libc::c_int;
        }
        len = strlen(tmp) as libc::c_int;
        memmove(
            tmp as *mut libc::c_void,
            tmp.offset(n as isize) as *const libc::c_void,
            (len - n + 1 as libc::c_int) as libc::c_ulong,
        );
        xfree((*this).url as *mut libc::c_void);
        len = (strlen(tmp))
            .wrapping_add(strlen(str))
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int;
        (*this).url = xmalloc(len as size_t) as *mut libc::c_char;
        memset((*this).url as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
        snprintf(
            (*this).url,
            len as libc::c_ulong,
            b"%s://%s\0" as *const u8 as *const libc::c_char,
            str,
            tmp,
        );
        xfree(tmp as *mut libc::c_void);
        xfree(str as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn url_set_hostname(
    mut this: URL,
    mut hostname: *mut libc::c_char,
) {
    let mut len: size_t = 0;
    if empty(hostname) as u64 != 0 {
        return;
    }
    xfree((*this).hostname as *mut libc::c_void);
    len = (strlen(hostname)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*this).hostname = xmalloc(len) as *mut libc::c_char;
    memset((*this).hostname as *mut libc::c_void, '\0' as i32, len);
    strncpy((*this).hostname, hostname, len);
}
pub unsafe extern "C" fn url_set_redirect(mut this: URL, mut redir: BOOLEAN) {
    (*this).redir = redir;
}
pub unsafe extern "C" fn url_set_conttype(mut this: URL, mut type_0: *mut libc::c_char) {
    (*this).conttype = xstrdup(type_0);
}
pub unsafe extern "C" fn url_set_method(mut this: URL, mut method: METHOD) {
    (*this).method = method;
}
pub unsafe extern "C" fn url_set_postdata(
    mut this: URL,
    mut postdata: *mut libc::c_char,
    mut postlen: size_t,
) {
    (*this).postlen = postlen;
    (*this)
        .postdata = xmalloc(
        ((*this).postlen).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*this).postdata as *mut libc::c_void,
        postdata as *const libc::c_void,
        (*this).postlen,
    );
    *((*this).postdata).offset((*this).postlen as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn url_get_ID(mut this: URL) -> libc::c_int {
    return (*this).ID;
}
pub unsafe extern "C" fn url_get_absolute(mut this: URL) -> *mut libc::c_char {
    return (if this.is_null() {
        b"NULL\0" as *const u8 as *const libc::c_char
    } else {
        (*this).url as *const libc::c_char
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn url_get_scheme(mut this: URL) -> SCHEME {
    return (*this).scheme;
}
pub unsafe extern "C" fn url_get_display(mut this: URL) -> *mut libc::c_char {
    if my.fullurl as u64 != 0 {
        return url_get_absolute(this);
    }
    if (*this).method as libc::c_uint == GET as libc::c_int as libc::c_uint {
        return url_get_request(this);
    }
    return url_get_absolute(this);
}
pub unsafe extern "C" fn url_get_scheme_name(mut this: URL) -> *mut libc::c_char {
    match (*this).scheme as libc::c_uint {
        1 => return b"http\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"https\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => return b"ftp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 => return b"proxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 | _ => {
            return b"unsupported\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    };
}
pub unsafe extern "C" fn url_get_username(mut this: URL) -> *mut libc::c_char {
    return (*this).username;
}
pub unsafe extern "C" fn url_get_password(mut this: URL) -> *mut libc::c_char {
    return (*this).password;
}
pub unsafe extern "C" fn url_get_hostname(mut this: URL) -> *mut libc::c_char {
    return (*this).hostname;
}
pub unsafe extern "C" fn url_get_port(mut this: URL) -> libc::c_int {
    return (*this).port;
}
pub unsafe extern "C" fn url_get_path(mut this: URL) -> *mut libc::c_char {
    return (*this).path;
}
pub unsafe extern "C" fn url_get_file(mut this: URL) -> *mut libc::c_char {
    return (*this).file;
}
pub unsafe extern "C" fn url_get_request(mut this: URL) -> *mut libc::c_char {
    return (*this).request;
}
pub unsafe extern "C" fn url_get_parameters(mut this: URL) -> *mut libc::c_char {
    return (*this).params;
}
pub unsafe extern "C" fn url_get_query(mut this: URL) -> *mut libc::c_char {
    return (*this).query;
}
pub unsafe extern "C" fn url_get_fragment(mut this: URL) -> *mut libc::c_char {
    return (*this).frag;
}
pub unsafe extern "C" fn url_get_postlen(mut this: URL) -> size_t {
    return (*this).postlen;
}
pub unsafe extern "C" fn url_get_postdata(mut this: URL) -> *mut libc::c_char {
    return (*this).postdata;
}
pub unsafe extern "C" fn url_get_posttemp(mut this: URL) -> *mut libc::c_char {
    return (*this).posttemp;
}
pub unsafe extern "C" fn url_get_conttype(mut this: URL) -> *mut libc::c_char {
    if ((*this).conttype).is_null() {
        if empty((my.conttype).as_mut_ptr()) as u64 == 0 {
            (*this).conttype = xstrdup((my.conttype).as_mut_ptr());
        } else {
            (*this)
                .conttype = xstrdup(
                b"application/x-www-form-urlencoded\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return (*this).conttype;
}
pub unsafe extern "C" fn url_get_method(mut this: URL) -> METHOD {
    return (*this).method;
}
pub unsafe extern "C" fn url_get_method_name(mut this: URL) -> *mut libc::c_char {
    match (*this).method as libc::c_uint {
        3 => return b"POST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        9 => return b"PATCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 => return b"PUT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 => return b"DELETE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 => return b"OPTIONS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"HEAD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 | _ => return b"GET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    };
}
pub unsafe extern "C" fn url_is_redirect(mut this: URL) -> BOOLEAN {
    return (*this).redir;
}
pub unsafe extern "C" fn url_set_username(
    mut this: URL,
    mut username: *mut libc::c_char,
) {
    let mut len: size_t = strlen(username);
    (*this)
        .username = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memset(
        (*this).username as *mut libc::c_void,
        '\0' as i32,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy((*this).username as *mut libc::c_void, username as *const libc::c_void, len);
}
pub unsafe extern "C" fn url_set_password(
    mut this: URL,
    mut password: *mut libc::c_char,
) {
    let mut len: size_t = strlen(password);
    (*this)
        .password = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memset(
        (*this).password as *mut libc::c_void,
        '\0' as i32,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy((*this).password as *mut libc::c_void, password as *const libc::c_void, len);
}
pub unsafe extern "C" fn url_dump(mut this: URL) {
    printf(b"URL ID:    %d\n\0" as *const u8 as *const libc::c_char, (*this).ID);
    printf(b"Abolute:   %s\n\0" as *const u8 as *const libc::c_char, (*this).url);
    printf(
        b"Scheme:    %s\n\0" as *const u8 as *const libc::c_char,
        url_get_scheme_name(this),
    );
    printf(
        b"Method:    %s\n\0" as *const u8 as *const libc::c_char,
        url_get_method_name(this),
    );
    printf(
        b"Username:  %s\n\0" as *const u8 as *const libc::c_char,
        url_get_username(this),
    );
    printf(
        b"Password:  %s\n\0" as *const u8 as *const libc::c_char,
        url_get_password(this),
    );
    printf(
        b"Hostname:  %s\n\0" as *const u8 as *const libc::c_char,
        url_get_hostname(this),
    );
    printf(b"Port:      %d\n\0" as *const u8 as *const libc::c_char, url_get_port(this));
    printf(b"Path:      %s\n\0" as *const u8 as *const libc::c_char, url_get_path(this));
    printf(b"File:      %s\n\0" as *const u8 as *const libc::c_char, url_get_file(this));
    printf(
        b"Request:   %s\n\0" as *const u8 as *const libc::c_char,
        url_get_request(this),
    );
    if (*this).hasparams as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        printf(
            b"Params:   %s\n\0" as *const u8 as *const libc::c_char,
            url_get_parameters(this),
        );
    }
    printf(
        b"Query:     %s\n\0" as *const u8 as *const libc::c_char,
        url_get_query(this),
    );
    printf(
        b"Fragment:  %s\n\0" as *const u8 as *const libc::c_char,
        url_get_fragment(this),
    );
    printf(
        b"Post Len:  %d\n\0" as *const u8 as *const libc::c_char,
        url_get_postlen(this) as libc::c_int,
    );
    printf(
        b"Post Data: %s\n\0" as *const u8 as *const libc::c_char,
        url_get_postdata(this),
    );
    printf(
        b"Cont Type: %s\n\0" as *const u8 as *const libc::c_char,
        url_get_conttype(this),
    );
}
pub unsafe extern "C" fn url_normalize(
    mut req: URL,
    mut location: *mut libc::c_char,
) -> URL {
    let mut ret: URL = 0 as *mut URL_T;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    __url_replace(
        location,
        b"&amp;\0" as *const u8 as *const libc::c_char,
        b"&\0" as *const u8 as *const libc::c_char,
    );
    __url_replace(
        location,
        b"&#038;\0" as *const u8 as *const libc::c_char,
        b"&\0" as *const u8 as *const libc::c_char,
    );
    len = (strlen(url_get_absolute(req)))
        .wrapping_add(strlen(location))
        .wrapping_add(32 as libc::c_int as libc::c_ulong);
    if !(stristr(location, b"data:image/gif\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        return 0 as URL;
    }
    if !(stristr(location, b"://\0" as *const u8 as *const libc::c_char)).is_null() {
        ret = new_url(location);
        if strlen(url_get_hostname(ret)) > 1 as libc::c_int as libc::c_ulong {
            return ret;
        }
    }
    if *location.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
        && *location.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
        && (!(strchr(location, '.' as i32)).is_null()
            && !(strchr(location, '/' as i32)).is_null())
    {
        ret = new_url(location);
        url_set_scheme(ret, url_get_scheme(req));
        if !(strchr(url_get_hostname(ret), '.' as i32)).is_null() {
            return ret;
        }
    }
    if !(strstr(location, b"localhost\0" as *const u8 as *const libc::c_char)).is_null()
    {
        ret = new_url(location);
        url_set_scheme(ret, url_get_scheme(req));
        if strlen(url_get_hostname(ret)) == 9 as libc::c_int as libc::c_ulong {
            return ret;
        }
    }
    url = malloc(len) as *mut libc::c_char;
    memset(url as *mut libc::c_void, '\0' as i32, len);
    if *location.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        if strlen(location) > 1 as libc::c_int as libc::c_ulong
            && *location.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            snprintf(
                url,
                len,
                b"%s:%s\0" as *const u8 as *const libc::c_char,
                url_get_scheme_name(req),
                location,
            );
        } else {
            snprintf(
                url,
                len,
                b"%s://%s:%d%s\0" as *const u8 as *const libc::c_char,
                url_get_scheme_name(req),
                url_get_hostname(req),
                url_get_port(req),
                location,
            );
        }
    } else if endswith(b"/\0" as *const u8 as *const libc::c_char, url_get_path(req))
        as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
    {
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        if *location.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && strlen(location) > 1 as libc::c_int as libc::c_ulong
        {
            tmp = location.offset(2 as libc::c_int as isize);
        } else {
            tmp = location;
        }
        snprintf(
            url,
            len,
            b"%s://%s:%d%s%s\0" as *const u8 as *const libc::c_char,
            url_get_scheme_name(req),
            url_get_hostname(req),
            url_get_port(req),
            url_get_path(req),
            tmp,
        );
    } else {
        snprintf(
            url,
            len,
            b"%s://%s:%d%s/%s\0" as *const u8 as *const libc::c_char,
            url_get_scheme_name(req),
            url_get_hostname(req),
            url_get_port(req),
            url_get_path(req),
            location,
        );
    }
    ret = new_url(url);
    url_set_scheme(ret, url_get_scheme(req));
    free(url as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn url_normalize_string(
    mut req: URL,
    mut location: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut u: URL = 0 as *mut URL_T;
    u = url_normalize(req, location);
    t = strdup(url_get_absolute(u));
    u = url_destroy(u);
    return t;
}
unsafe extern "C" fn __url_parse(mut this: URL, mut url: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut post: *mut libc::c_char = 0 as *mut libc::c_char;
    esc = __url_escape(url);
    if my.escape as u64 != 0 {
        ptr = __url_set_absolute(this, esc);
    } else {
        ptr = __url_set_absolute(this, url);
    }
    ptr = __url_set_scheme(this, ptr);
    post = strstr((*this).url, b" POST\0" as *const u8 as *const libc::c_char);
    if post.is_null() {
        post = strstr((*this).url, b" PUT\0" as *const u8 as *const libc::c_char);
    }
    if post.is_null() {
        post = strstr((*this).url, b" PATCH\0" as *const u8 as *const libc::c_char);
    }
    if post.is_null() {
        post = strstr((*this).url, b" OPTIONS\0" as *const u8 as *const libc::c_char);
    }
    if post.is_null() {
        post = strstr((*this).url, b" DELETE\0" as *const u8 as *const libc::c_char);
    }
    if !post.is_null() {
        if strncasecmp(
            post,
            b" PUT\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*this).method = PUT;
            *post = '\0' as i32 as libc::c_char;
            post = post.offset(4 as libc::c_int as isize);
        } else if strncasecmp(
            post,
            b" POST\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*this).method = POST;
            *post = '\0' as i32 as libc::c_char;
            post = post.offset(5 as libc::c_int as isize);
        } else if strncasecmp(
            post,
            b" DELETE\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*this).method = DELETE;
            *post = '\0' as i32 as libc::c_char;
            post = post.offset(7 as libc::c_int as isize);
        } else if strncasecmp(
            post,
            b" OPTIONS\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*this).method = OPTIONS;
            *post = '\0' as i32 as libc::c_char;
            post = post.offset(8 as libc::c_int as isize);
        } else {
            (*this).method = PATCH;
            *post = '\0' as i32 as libc::c_char;
            post = post.offset(6 as libc::c_int as isize);
        }
        __parse_post_data(this, post);
    } else {
        (*this).method = GET;
        (*this).postdata = 0 as *mut libc::c_char;
        (*this).posttemp = 0 as *mut libc::c_char;
        (*this).postlen = 0 as libc::c_int as size_t;
    }
    if __url_has_credentials(ptr) as u64 != 0 {
        ptr = __url_set_username(this, ptr);
        ptr = __url_set_password(this, ptr);
    }
    ptr = __url_set_hostname(this, ptr);
    ptr = __url_set_port(this, ptr);
    ptr = __url_set_path(this, ptr);
    ptr = __url_set_file(this, ptr);
    ptr = __url_set_parameters(this, ptr);
    ptr = __url_set_query(this, ptr);
    ptr = __url_set_fragment(this, ptr);
}
unsafe extern "C" fn __parse_post_data(mut this: URL, mut datap: *mut libc::c_char) {
    while *(*__ctype_b_loc()).offset(*datap as libc::c_uint as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        datap = datap.offset(1);
        datap;
    }
    if *datap as libc::c_int == '<' as i32 {
        datap = datap.offset(1);
        datap;
        load_file(this, datap);
        datap = __url_set_path(this, datap);
        datap = __url_set_file(this, datap);
        return;
    } else {
        (*this).postdata = xstrdup(datap);
        (*this).postlen = strlen((*this).postdata);
        if empty((my.conttype).as_mut_ptr()) as u64 == 0 {
            (*this).conttype = xstrdup((my.conttype).as_mut_ptr());
        } else {
            (*this)
                .conttype = xstrdup(
                b"application/x-www-form-urlencoded\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return;
    };
}
unsafe extern "C" fn __url_set_absolute(
    mut this: URL,
    mut url: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scheme: [libc::c_char; 16] = [0; 16];
    if empty(url) as u64 != 0 {
        return 0 as *mut libc::c_char;
    }
    memset(
        scheme.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        16 as libc::c_int as libc::c_ulong,
    );
    if strncasecmp(
        url,
        b"http:\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        n = 7 as libc::c_int;
        strncpy(
            scheme.as_mut_ptr(),
            b"http\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        );
    }
    if strncasecmp(
        url,
        b"https:\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        n = 8 as libc::c_int;
        strncpy(
            scheme.as_mut_ptr(),
            b"https\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        );
    }
    if strncasecmp(
        url,
        b"ftp:\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        n = 6 as libc::c_int;
        strncpy(
            scheme.as_mut_ptr(),
            b"ftp\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        );
    }
    if strlen(scheme.as_mut_ptr()) < 3 as libc::c_int as libc::c_ulong {
        n = 7 as libc::c_int;
        strncpy(
            scheme.as_mut_ptr(),
            b"http\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        );
    }
    len = (strlen(url)).wrapping_add(5 as libc::c_int as libc::c_ulong);
    if __url_has_scheme(url) as u64 == 0 {
        (*this).url = xmalloc(len.wrapping_add(n as libc::c_ulong)) as *mut libc::c_char;
        memset(
            (*this).url as *mut libc::c_void,
            '\0' as i32,
            len.wrapping_add(n as libc::c_ulong),
        );
        slash = strstr(url, b"/\0" as *const u8 as *const libc::c_char);
        if !slash.is_null() {
            snprintf(
                (*this).url,
                len.wrapping_add(n as libc::c_ulong),
                b"%s://%s\0" as *const u8 as *const libc::c_char,
                scheme.as_mut_ptr(),
                url,
            );
        } else {
            snprintf(
                (*this).url,
                len.wrapping_add(n as libc::c_ulong),
                b"%s://%s/\0" as *const u8 as *const libc::c_char,
                scheme.as_mut_ptr(),
                url,
            );
        }
    } else {
        (*this).url = xmalloc(len) as *mut libc::c_char;
        memset((*this).url as *mut libc::c_void, '\0' as i32, len);
        snprintf((*this).url, len, b"%s\0" as *const u8 as *const libc::c_char, url);
    }
    return (*this).url;
}
unsafe extern "C" fn __url_has_scheme(mut url: *mut libc::c_char) -> BOOLEAN {
    let mut p: *const libc::c_char = url;
    if *p == 0
        || !(*(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
    {
        return boolean_false;
    }
    p = p.offset(1);
    p;
    while *p as libc::c_int != 0
        && (*(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
    {
        p = p.offset(1);
        p;
    }
    return (*p as libc::c_int == ':' as i32) as libc::c_int as BOOLEAN;
}
unsafe extern "C" fn __url_has_credentials(mut url: *mut libc::c_char) -> BOOLEAN {
    let mut p: *const libc::c_char = strpbrk(
        url,
        b"@/?#;\0" as *const u8 as *const libc::c_char,
    ) as *const libc::c_char;
    if p.is_null() || *p as libc::c_int != '@' as i32 {
        return boolean_false;
    }
    return boolean_true;
}
unsafe extern "C" fn __url_default_port(mut this: URL) -> libc::c_int {
    match (*this).scheme as libc::c_uint {
        3 => return 21 as libc::c_int,
        1 => return 80 as libc::c_int,
        2 => return 443 as libc::c_int,
        0 | _ => return 80 as libc::c_int,
    };
}
unsafe extern "C" fn __url_set_scheme(
    mut this: URL,
    mut url: *mut libc::c_char,
) -> *mut libc::c_char {
    if strncasecmp(
        (*this).url,
        b"http:\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        (*this).scheme = HTTP;
        return url.offset(7 as libc::c_int as isize);
    }
    if strncasecmp(
        (*this).url,
        b"https:\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        (*this).scheme = HTTPS;
        return url.offset(8 as libc::c_int as isize);
    }
    if strncasecmp(
        (*this).url,
        b"ftp:\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        (*this).scheme = FTP;
        return url.offset(6 as libc::c_int as isize);
    }
    (*this).scheme = UNSUPPORTED;
    return url;
}
unsafe extern "C" fn __url_set_username(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    a = strchr(str, '@' as i32);
    s = strchr(str, '/' as i32);
    if a.is_null() || !s.is_null() && a >= s {
        return str;
    }
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0
        && *str.offset(i as isize) as libc::c_int != ':' as i32
        && *str.offset(i as isize) as libc::c_int != '@' as i32
        && *str.offset(i as isize) as libc::c_int != '/' as i32
    {
        i += 1;
        i;
    }
    if *str.offset(i as isize) as libc::c_int != '@' as i32
        && *str.offset(i as isize) as libc::c_int != ':' as i32
    {
        return str;
    }
    (*this)
        .username = malloc((i + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    memcpy(
        (*this).username as *mut libc::c_void,
        str as *const libc::c_void,
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    *((*this).username).offset(i as isize) = '\0' as i32 as libc::c_char;
    str = str.offset((i + 1 as libc::c_int) as isize);
    return str;
}
unsafe extern "C" fn __url_set_password(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    a = strchr(str, '@' as i32);
    s = strchr(str, '/' as i32);
    if a.is_null() || !s.is_null() && a >= s {
        return str;
    }
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != '@' as i32 {
        i += 1;
        i;
    }
    (*this).password = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        (*this).password as *mut libc::c_void,
        str as *const libc::c_void,
        i as libc::c_ulong,
    );
    *((*this).password).offset(i as isize) = '\0' as i32 as libc::c_char;
    str = str.offset((i + 1 as libc::c_int) as isize);
    return str;
}
unsafe extern "C" fn __url_set_hostname(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if startswith(b"//\0" as *const u8 as *const libc::c_char, str) as u64 != 0 {
        n = 2 as libc::c_int;
        len = strlen(str) as libc::c_int;
        memmove(
            str as *mut libc::c_void,
            str.offset(n as isize) as *const libc::c_void,
            (len - n + 1 as libc::c_int) as libc::c_ulong,
        );
    }
    if startswith(b"[\0" as *const u8 as *const libc::c_char, str) as u64 != 0 {
        i = 0 as libc::c_int;
        while *str.offset(i as isize) as libc::c_int != 0
            && *str.offset(i as isize) as libc::c_int != ']' as i32
        {
            i += 1;
            i;
        }
        if *str.offset(i as isize) as libc::c_int == ']' as i32 {
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while *str.offset(i as isize) as libc::c_int != 0
            && *str.offset(i as isize) as libc::c_int != '/' as i32
            && *str.offset(i as isize) as libc::c_int != '#' as i32
            && *str.offset(i as isize) as libc::c_int != ':' as i32
        {
            i += 1;
            i;
        }
    }
    (*this).hostname = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memset(
        (*this).hostname as *mut libc::c_void,
        '\0' as i32,
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        (*this).hostname as *mut libc::c_void,
        str as *const libc::c_void,
        i as libc::c_ulong,
    );
    if *str.offset(i as isize) as libc::c_int == ':' as i32 {
        str = str.offset((i + 1 as libc::c_int) as isize);
    } else {
        str = str.offset(i as isize);
    }
    return str;
}
unsafe extern "C" fn __url_set_port(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut portstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    (*this).port = __url_default_port(this);
    i = 0 as libc::c_int;
    while *(*__ctype_b_loc()).offset(*str.offset(i as isize) as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        i += 1;
        i;
    }
    if i == 0 as libc::c_int {
        return str;
    }
    portstr = malloc((i + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    memcpy(
        portstr as *mut libc::c_void,
        str as *const libc::c_void,
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    *portstr.offset(i as isize) = '\0' as i32 as libc::c_char;
    (*this).port = atoi(portstr);
    xfree(portstr as *mut libc::c_void);
    str = str.offset(i as isize);
    return str;
}
unsafe extern "C" fn __url_set_path(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    if !str.is_null()
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        (*this).request = xstrdup(b"/\0" as *const u8 as *const libc::c_char);
        return str;
    }
    (*this).request = xstrdup(str);
    c = strstr((*this).request, b"#\0" as *const u8 as *const libc::c_char);
    if !c.is_null() {
        *c = '\0' as i32 as libc::c_char;
    }
    i = strlen(str) as libc::c_int;
    while i > 0 as libc::c_int && *str.offset(i as isize) as libc::c_int != '/' as i32 {
        i -= 1;
        i;
    }
    j = 0 as libc::c_int;
    while *str.offset(j as isize) as libc::c_int != 0
        && (*str.offset(j as isize) as libc::c_int != '#' as i32
            && *(*__ctype_b_loc())
                .offset(*str.offset(j as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        j += 1;
        j;
    }
    if *str.offset(i as isize) as libc::c_int != '/' as i32 {
        if (*this).scheme as libc::c_uint == FTP as libc::c_int as libc::c_uint {
            (*this)
                .path = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            (*this).path = xmalloc(2 as libc::c_int as size_t) as *mut libc::c_char;
            (*this).request = xmalloc(2 as libc::c_int as size_t) as *mut libc::c_char;
            strncpy(
                (*this).path,
                b"/\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            );
            strncpy(
                (*this).request,
                b"/\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            );
            *((*this).path)
                .offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            *((*this).request)
                .offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
    } else {
        (*this).path = xmalloc((i + 2 as libc::c_int) as size_t) as *mut libc::c_char;
        memcpy(
            (*this).path as *mut libc::c_void,
            str as *const libc::c_void,
            (i + 1 as libc::c_int) as libc::c_ulong,
        );
        *((*this).path).offset(i as isize) = '/' as i32 as libc::c_char;
        *((*this).path)
            .offset((i + 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        if (*this).scheme as libc::c_uint == FTP as libc::c_int as libc::c_uint
            && *((*this).path).offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32
        {
            memmove(
                (*this).path as *mut libc::c_void,
                ((*this).path).offset(1 as libc::c_int as isize) as *const libc::c_void,
                strlen((*this).path),
            );
        }
    }
    trim((*this).request);
    str = str.offset((i + 1 as libc::c_int) as isize);
    return str;
}
unsafe extern "C" fn __url_set_file(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !((*this).file).is_null()
        && strlen((*this).file) > 1 as libc::c_int as libc::c_ulong
    {
        return str;
    }
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0
        && (*str.offset(i as isize) as libc::c_int != ';' as i32
            && *str.offset(i as isize) as libc::c_int != '?' as i32
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        i += 1;
        i;
    }
    (*this).file = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memset(
        (*this).file as *mut libc::c_void,
        '\0' as i32,
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        (*this).file as *mut libc::c_void,
        str as *const libc::c_void,
        i as libc::c_ulong,
    );
    trim((*this).file);
    if *str.offset(i as isize) as libc::c_int == ';' as i32 {
        (*this).hasparams = boolean_true;
        str = str.offset((i + 1 as libc::c_int) as isize);
    } else if *str.offset(i as isize) as libc::c_int == '?' as i32 {
        str = str.offset((i + 1 as libc::c_int) as isize);
    } else {
        str = str.offset(i as isize);
    }
    return str;
}
unsafe extern "C" fn __url_set_parameters(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !((*this).params).is_null()
        && strlen((*this).params) > 1 as libc::c_int as libc::c_ulong
    {
        return str;
    }
    if (*this).hasparams as libc::c_uint == boolean_false as libc::c_int as libc::c_uint
    {
        (*this).params = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        return str;
    }
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0
        && (*str.offset(i as isize) as libc::c_int != '?' as i32
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        i += 1;
        i;
    }
    (*this).params = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memset(
        (*this).params as *mut libc::c_void,
        '\0' as i32,
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        (*this).params as *mut libc::c_void,
        str as *const libc::c_void,
        i as libc::c_ulong,
    );
    if *str.offset(i as isize) as libc::c_int == '?' as i32 {
        str = str.offset((i + 1 as libc::c_int) as isize);
    } else {
        str = str.offset(i as isize);
    }
    return str;
}
unsafe extern "C" fn __url_set_query(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if str.is_null() {
        (*this).query = xstrcat(b"\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    if !((*this).query).is_null()
        && strlen((*this).query) > 1 as libc::c_int as libc::c_ulong
    {
        return str;
    }
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0
        && (*str.offset(i as isize) as libc::c_int != '#' as i32
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        i += 1;
        i;
    }
    (*this).query = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memset(
        (*this).query as *mut libc::c_void,
        '\0' as i32,
        (i + 1 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        (*this).query as *mut libc::c_void,
        str as *const libc::c_void,
        i as libc::c_ulong,
    );
    if *str.offset(i as isize) as libc::c_int == '#' as i32 {
        str = str.offset((i + 1 as libc::c_int) as isize);
    } else {
        str = str.offset(i as isize);
    }
    return str;
}
unsafe extern "C" fn __url_set_fragment(
    mut this: URL,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !((*this).frag).is_null()
        && strlen((*this).frag) > 1 as libc::c_int as libc::c_ulong
    {
        return str;
    }
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*str.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        i += 1;
        i;
    }
    (*this).frag = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        (*this).frag as *mut libc::c_void,
        str as *const libc::c_void,
        i as libc::c_ulong,
    );
    str = str.offset((i + 1 as libc::c_int) as isize);
    return str;
}
static mut urlchr_table: [libc::c_uchar; 256] = [
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern "C" fn decide_copy_method(mut p: *const libc::c_char) -> copy_method {
    if *p as libc::c_int == '%' as i32 {
        if (((*p.offset(1 as libc::c_int as isize) as libc::c_uint)
            .wrapping_sub(48 as libc::c_uint) & 255 as libc::c_int as libc::c_uint)
            .wrapping_mul(18 as libc::c_int as libc::c_uint)
            .wrapping_div(17 as libc::c_int as libc::c_uint)
            .wrapping_mul(52 as libc::c_int as libc::c_uint)
            .wrapping_div(51 as libc::c_int as libc::c_uint)
            .wrapping_mul(58 as libc::c_int as libc::c_uint)
            .wrapping_div(114 as libc::c_int as libc::c_uint)
            .wrapping_mul(13 as libc::c_int as libc::c_uint)
            .wrapping_div(11 as libc::c_int as libc::c_uint)
            .wrapping_mul(14 as libc::c_int as libc::c_uint)
            .wrapping_div(13 as libc::c_int as libc::c_uint)
            .wrapping_mul(35 as libc::c_int as libc::c_uint)
            .wrapping_add(35 as libc::c_int as libc::c_uint)
            .wrapping_div(36 as libc::c_int as libc::c_uint)
            .wrapping_mul(35 as libc::c_int as libc::c_uint)
            .wrapping_div(33 as libc::c_int as libc::c_uint)
            .wrapping_mul(34 as libc::c_int as libc::c_uint)
            .wrapping_div(33 as libc::c_int as libc::c_uint)
            .wrapping_mul(35 as libc::c_int as libc::c_uint)
            .wrapping_div(170 as libc::c_int as libc::c_uint)
            ^ 4 as libc::c_int as libc::c_uint)
            .wrapping_sub(3 as libc::c_int as libc::c_uint)
            & 255 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint
            <= 2 as libc::c_uint
            && (((*p.offset(2 as libc::c_int as isize) as libc::c_uint)
                .wrapping_sub(48 as libc::c_uint) & 255 as libc::c_int as libc::c_uint)
                .wrapping_mul(18 as libc::c_int as libc::c_uint)
                .wrapping_div(17 as libc::c_int as libc::c_uint)
                .wrapping_mul(52 as libc::c_int as libc::c_uint)
                .wrapping_div(51 as libc::c_int as libc::c_uint)
                .wrapping_mul(58 as libc::c_int as libc::c_uint)
                .wrapping_div(114 as libc::c_int as libc::c_uint)
                .wrapping_mul(13 as libc::c_int as libc::c_uint)
                .wrapping_div(11 as libc::c_int as libc::c_uint)
                .wrapping_mul(14 as libc::c_int as libc::c_uint)
                .wrapping_div(13 as libc::c_int as libc::c_uint)
                .wrapping_mul(35 as libc::c_int as libc::c_uint)
                .wrapping_add(35 as libc::c_int as libc::c_uint)
                .wrapping_div(36 as libc::c_int as libc::c_uint)
                .wrapping_mul(35 as libc::c_int as libc::c_uint)
                .wrapping_div(33 as libc::c_int as libc::c_uint)
                .wrapping_mul(34 as libc::c_int as libc::c_uint)
                .wrapping_div(33 as libc::c_int as libc::c_uint)
                .wrapping_mul(35 as libc::c_int as libc::c_uint)
                .wrapping_div(170 as libc::c_int as libc::c_uint)
                ^ 4 as libc::c_int as libc::c_uint)
                .wrapping_sub(3 as libc::c_int as libc::c_uint)
                & 255 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint
                <= 2 as libc::c_uint
        {
            let mut preempt: libc::c_char = (((if (*p.offset(1 as libc::c_int as isize)
                as libc::c_int) < 'A' as i32
            {
                *p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32
            } else {
                ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *p
                                .offset(1 as libc::c_int as isize) as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(
                                *p.offset(1 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(
                                *p.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                            );
                    }
                    __res
                }) - 'A' as i32 + 10 as libc::c_int
            }) << 4 as libc::c_int)
                + (if (*p.offset(2 as libc::c_int as isize) as libc::c_int) < 'A' as i32
                {
                    *p.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32
                } else {
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *p
                                    .offset(2 as libc::c_int as isize) as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = toupper(
                                    *p.offset(2 as libc::c_int as isize) as libc::c_int,
                                );
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(
                                    *p.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                                );
                        }
                        __res
                    }) - 'A' as i32 + 10 as libc::c_int
                })) as libc::c_char;
            if urlchr_table[preempt as libc::c_uchar as usize] as libc::c_int
                & urlchr_unsafe as libc::c_int != 0
                || urlchr_table[preempt as libc::c_uchar as usize] as libc::c_int
                    & urlchr_reserved as libc::c_int != 0
            {
                return CM_PASSTHROUGH
            } else {
                return CM_DECODE
            }
        } else {
            return CM_ENCODE
        }
    } else if urlchr_table[*p as libc::c_uchar as usize] as libc::c_int
        & urlchr_unsafe as libc::c_int != 0
        && urlchr_table[*p as libc::c_uchar as usize] as libc::c_int
            & urlchr_reserved as libc::c_int == 0
    {
        return CM_ENCODE
    } else {
        return CM_PASSTHROUGH
    };
}
unsafe extern "C" fn __url_has_method(mut url: *const libc::c_char) -> METHOD {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    static mut methods: [*const libc::c_char; 9] = [
        b" GET\0" as *const u8 as *const libc::c_char,
        b" HEAD\0" as *const u8 as *const libc::c_char,
        b" POST\0" as *const u8 as *const libc::c_char,
        b" PUT\0" as *const u8 as *const libc::c_char,
        b" TRACE\0" as *const u8 as *const libc::c_char,
        b" DELETE\0" as *const u8 as *const libc::c_char,
        b" OPTIONS\0" as *const u8 as *const libc::c_char,
        b" CONNECT\0" as *const u8 as *const libc::c_char,
        b" PATCH\0" as *const u8 as *const libc::c_char,
    ];
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        r = strstr(url, methods[i as usize]);
        if !r.is_null() {
            return i as METHOD;
        }
        i = i.wrapping_add(1);
        i;
    }
    return NOMETHOD;
}
unsafe extern "C" fn __url_escape(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldlen: libc::c_int = 0;
    let mut newlen: libc::c_int = 0;
    let mut host_len: libc::c_int = 0;
    let mut path_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut encode_count: libc::c_int = 0 as libc::c_int;
    let mut decode_count: libc::c_int = 0 as libc::c_int;
    if __url_has_method(s) as libc::c_uint != NOMETHOD as libc::c_int as libc::c_uint {
        return s as *mut libc::c_char;
    }
    host_start = strstr(s, b"//\0" as *const u8 as *const libc::c_char);
    if !host_start.is_null() {
        host_start = host_start.offset(2 as libc::c_int as isize);
    } else {
        host_start = s as *mut libc::c_char;
    }
    path_start = strstr(host_start, b"/\0" as *const u8 as *const libc::c_char);
    if !path_start.is_null() {
        path_start = path_start.offset(1 as libc::c_int as isize);
    } else {
        return s as *mut libc::c_char
    }
    p1 = path_start;
    while *p1 != 0 {
        match decide_copy_method(p1) as libc::c_uint {
            1 => {
                encode_count += 1;
                encode_count;
            }
            0 => {
                decode_count += 1;
                decode_count;
            }
            2 | _ => {}
        }
        p1 = p1.offset(1);
        p1;
    }
    if encode_count == 0 && decode_count == 0 {
        return s as *mut libc::c_char;
    }
    oldlen = p1.offset_from(s) as libc::c_long as libc::c_int;
    host_len = path_start.offset_from(s) as libc::c_long as libc::c_int;
    newlen = oldlen + 2 as libc::c_int * (encode_count - decode_count);
    newstr = xmalloc((newlen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        newstr as *mut libc::c_void,
        s as *const libc::c_void,
        host_len as libc::c_ulong,
    );
    p1 = path_start;
    p2 = newstr.offset(host_len as isize);
    while *p1 != 0 {
        match decide_copy_method(p1) as libc::c_uint {
            1 => {
                let fresh0 = p1;
                p1 = p1.offset(1);
                let mut c: libc::c_uchar = *fresh0 as libc::c_uchar;
                let fresh1 = p2;
                p2 = p2.offset(1);
                *fresh1 = '%' as i32 as libc::c_char;
                let fresh2 = p2;
                p2 = p2.offset(1);
                *fresh2 = (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789ABCDEF\0",
                ))[(c as libc::c_int >> 4 as libc::c_int) as usize];
                let fresh3 = p2;
                p2 = p2.offset(1);
                *fresh3 = (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789ABCDEF\0",
                ))[(c as libc::c_int & 0xf as libc::c_int) as usize];
            }
            0 => {
                let fresh4 = p2;
                p2 = p2.offset(1);
                *fresh4 = (((if (*p1.offset(1 as libc::c_int as isize) as libc::c_int)
                    < 'A' as i32
                {
                    *p1.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32
                } else {
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *p1
                                    .offset(1 as libc::c_int as isize) as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = toupper(
                                    *p1.offset(1 as libc::c_int as isize) as libc::c_int,
                                );
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(
                                    *p1.offset(1 as libc::c_int as isize) as libc::c_int
                                        as isize,
                                );
                        }
                        __res
                    }) - 'A' as i32 + 10 as libc::c_int
                }) << 4 as libc::c_int)
                    + (if (*p1.offset(2 as libc::c_int as isize) as libc::c_int)
                        < 'A' as i32
                    {
                        *p1.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32
                    } else {
                        ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = *p1
                                        .offset(2 as libc::c_int as isize) as libc::c_int;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = toupper(
                                        *p1.offset(2 as libc::c_int as isize) as libc::c_int,
                                    );
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc())
                                    .offset(
                                        *p1.offset(2 as libc::c_int as isize) as libc::c_int
                                            as isize,
                                    );
                            }
                            __res
                        }) - 'A' as i32 + 10 as libc::c_int
                    })) as libc::c_char;
                p1 = p1.offset(3 as libc::c_int as isize);
            }
            2 => {
                let fresh5 = p1;
                p1 = p1.offset(1);
                let fresh6 = p2;
                p2 = p2.offset(1);
                *fresh6 = *fresh5;
            }
            _ => {}
        }
    }
    *p2 = '\0' as i32 as libc::c_char;
    return newstr;
}
unsafe extern "C" fn __url_replace(
    mut url: *mut libc::c_char,
    mut needle: *const libc::c_char,
    mut replacement: *const libc::c_char,
) {
    let mut buf: [libc::c_char; 4096] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ins: *mut libc::c_char = &mut *buf
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *const libc::c_char = url;
    let mut nlen: size_t = strlen(needle);
    let mut rlen: size_t = strlen(replacement);
    loop {
        let mut p: *const libc::c_char = strstr(tmp, needle);
        if p.is_null() {
            strcpy(ins, tmp);
            break;
        } else {
            memcpy(
                ins as *mut libc::c_void,
                tmp as *const libc::c_void,
                p.offset_from(tmp) as libc::c_long as libc::c_ulong,
            );
            ins = ins.offset(p.offset_from(tmp) as libc::c_long as isize);
            memcpy(ins as *mut libc::c_void, replacement as *const libc::c_void, rlen);
            ins = ins.offset(rlen as isize);
            tmp = p.offset(nlen as isize);
        }
    }
    if strlen(buf.as_mut_ptr()) > strlen(url) {
        str = realloc(
            url as *mut libc::c_void,
            (strlen(buf.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if str.is_null() {
            return;
        }
        url = str;
        memset(
            url as *mut libc::c_void,
            '\0' as i32,
            (strlen(buf.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        memset(url as *mut libc::c_void, '\0' as i32, strlen(url));
    }
    strncpy(url, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
}
