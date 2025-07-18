use ::libc;
extern "C" {
    pub type URL_T;
    pub type AUTH_T;
    pub type ARRAY_T;
    pub type HASH_T;
    pub type COOKIES_T;
    pub type DATE_T;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut my: CONFIG;
    fn hash_set_destroyer(this: HASH, m: method);
    fn hash_destroy(this: HASH) -> HASH;
    fn hash_contains(this: HASH, key: *mut libc::c_char) -> BOOLEAN;
    fn hash_remove(this: HASH, key: *mut libc::c_char);
    fn hash_get(this: HASH, key: *mut libc::c_char) -> *mut libc::c_void;
    fn hash_nadd(
        this: HASH,
        key: *mut libc::c_char,
        val: *mut libc::c_void,
        len: size_t,
    );
    fn new_hash() -> HASH;
    fn url_get_request(this: URL) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut DATESIZE: size_t;
    fn new_date(_: *mut libc::c_char) -> DATE;
    fn new_etag(etag: *mut libc::c_char) -> DATE;
    fn date_destroy(this: DATE) -> DATE;
    fn date_expired(this: DATE) -> BOOLEAN;
    fn date_get_etag(this: DATE) -> *mut libc::c_char;
    fn date_get_rfc850(this: DATE) -> *mut libc::c_char;
    fn empty(s: *const libc::c_char) -> BOOLEAN;
}
pub type size_t = libc::c_ulong;
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
pub type method = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type URL = *mut URL_T;
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
pub type AUTH = *mut AUTH_T;
pub type ARRAY = *mut ARRAY_T;
pub type HASH = *mut HASH_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
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
pub type DATE = *mut DATE_T;
pub type CTYPE = libc::c_uint;
pub const C_EXPIRES: CTYPE = 2;
pub const C_LAST: CTYPE = 1;
pub const C_ETAG: CTYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CACHE_T {
    pub cache: HASH,
    pub hlen: libc::c_int,
    pub header: *mut libc::c_char,
}
pub type CACHE = *mut CACHE_T;
static mut keys: [*const libc::c_char; 4] = [
    b"ET_\0" as *const u8 as *const libc::c_char,
    b"LM_\0" as *const u8 as *const libc::c_char,
    b"EX_\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub static mut CACHESIZE: size_t = ::std::mem::size_of::<CACHE_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_cache() -> CACHE {
    let mut this: CACHE = calloc(CACHESIZE, 1 as libc::c_int as libc::c_ulong) as CACHE;
    (*this).cache = new_hash();
    (*this).header = 0 as *mut libc::c_char;
    hash_set_destroyer(
        (*this).cache,
        ::std::mem::transmute::<
            *mut libc::c_void,
            method,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(DATE) -> DATE>,
                *mut libc::c_void,
            >(Some(date_destroy as unsafe extern "C" fn(DATE) -> DATE)),
        ),
    );
    return this;
}
pub unsafe extern "C" fn cache_destroy(mut this: CACHE) -> CACHE {
    if !this.is_null() {
        (*this).cache = hash_destroy((*this).cache);
        free(this as *mut libc::c_void);
        this = 0 as CACHE;
    }
    return this;
}
pub unsafe extern "C" fn cache_contains(
    mut this: CACHE,
    mut type_0: CTYPE,
    mut U: URL,
) -> BOOLEAN {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: BOOLEAN = boolean_false;
    if my.cache as u64 == 0 {
        return boolean_false;
    }
    key = __build_key(type_0, U);
    if key.is_null() {
        return boolean_false;
    }
    found = hash_contains((*this).cache, key);
    free(key as *mut libc::c_void);
    return found;
}
pub unsafe extern "C" fn is_cached(mut this: CACHE, mut U: URL) -> BOOLEAN {
    let mut day: DATE = 0 as DATE;
    let mut key: *mut libc::c_char = __build_key(C_EXPIRES, U);
    if hash_contains((*this).cache, key) as u64 != 0 {
        day = hash_get((*this).cache, key) as DATE;
        if date_expired(day) as libc::c_uint
            == boolean_false as libc::c_int as libc::c_uint
        {
            return boolean_true
        } else {
            hash_remove((*this).cache, key);
            return boolean_false;
        }
    }
    return boolean_false;
}
pub unsafe extern "C" fn cache_add(
    mut this: CACHE,
    mut type_0: CTYPE,
    mut U: URL,
    mut date: *mut libc::c_char,
) {
    let mut key: *mut libc::c_char = __build_key(type_0, U);
    if key.is_null() {
        return;
    }
    if type_0 as libc::c_uint != C_EXPIRES as libc::c_int as libc::c_uint
        && hash_contains((*this).cache, key) as libc::c_uint != 0
    {
        hash_remove((*this).cache, key);
    }
    match type_0 as libc::c_uint {
        0 => {
            hash_nadd((*this).cache, key, new_etag(date) as *mut libc::c_void, DATESIZE);
        }
        2 => {
            if !(hash_contains((*this).cache, key) as libc::c_uint
                == boolean_true as libc::c_int as libc::c_uint)
            {
                hash_nadd(
                    (*this).cache,
                    key,
                    new_date(date) as *mut libc::c_void,
                    DATESIZE,
                );
            }
        }
        _ => {
            hash_nadd((*this).cache, key, new_date(date) as *mut libc::c_void, DATESIZE);
        }
    }
    free(key as *mut libc::c_void);
}
pub unsafe extern "C" fn cache_get(
    mut this: CACHE,
    mut type_0: CTYPE,
    mut U: URL,
) -> DATE {
    let mut date: DATE = 0 as *mut DATE_T;
    let mut key: *mut libc::c_char = __build_key(type_0, U);
    if key.is_null() {
        return 0 as DATE;
    }
    date = hash_get((*this).cache, key) as DATE;
    free(key as *mut libc::c_void);
    return date;
}
pub unsafe extern "C" fn cache_get_header(
    mut this: CACHE,
    mut type_0: CTYPE,
    mut U: URL,
) -> *mut libc::c_char {
    let mut d: DATE = 0 as DATE;
    let mut e: DATE = 0 as DATE;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut exp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    if cache_contains(this, type_0, U) as u64 == 0 {
        return 0 as *mut libc::c_char;
    }
    exp = __build_key(C_EXPIRES, U);
    if !exp.is_null() {
        e = hash_get((*this).cache, exp) as DATE;
        free(exp as *mut libc::c_void);
        if date_expired(e) as u64 != 0 {
            return 0 as *mut libc::c_char;
        }
    }
    key = __build_key(type_0, U);
    if key.is_null() {
        return 0 as *mut libc::c_char;
    }
    d = hash_get((*this).cache, key) as DATE;
    if d.is_null() {
        return 0 as *mut libc::c_char;
    }
    memset(
        tmp.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        256 as libc::c_int as libc::c_ulong,
    );
    match type_0 as libc::c_uint {
        0 => {
            ptr = strdup(date_get_etag(d));
            if empty(ptr) as u64 != 0 {
                return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            snprintf(
                tmp.as_mut_ptr(),
                256 as libc::c_int as libc::c_ulong,
                b"If-None-Match: %s\r\n\0" as *const u8 as *const libc::c_char,
                ptr,
            );
            (*this).header = strdup(tmp.as_mut_ptr());
            free(ptr as *mut libc::c_void);
            return (*this).header;
        }
        _ => {
            ptr = strdup(date_get_rfc850(d));
            if empty(ptr) as u64 != 0 {
                return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            snprintf(
                tmp.as_mut_ptr(),
                256 as libc::c_int as libc::c_ulong,
                b"If-Modified-Since: %s\r\n\0" as *const u8 as *const libc::c_char,
                ptr,
            );
            (*this).header = strdup(tmp.as_mut_ptr());
            free(ptr as *mut libc::c_void);
            return (*this).header;
        }
    };
}
unsafe extern "C" fn __build_key(mut type_0: CTYPE, mut U: URL) -> *mut libc::c_char {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    if U.is_null() || (url_get_request(U)).is_null()
        || strlen(url_get_request(U)) < 1 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut libc::c_char;
    }
    len = (strlen(url_get_request(U)))
        .wrapping_add(strlen(keys[type_0 as usize]))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    key = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    memset(
        key as *mut libc::c_void,
        '\0' as i32,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    snprintf(
        key,
        len as libc::c_ulong,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        keys[type_0 as usize],
        url_get_request(U),
    );
    return key;
}
