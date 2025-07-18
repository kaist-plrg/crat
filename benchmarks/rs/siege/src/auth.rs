use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type CREDS_T;
    pub type ARRAY_T;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn base64_encode(
        data: *const libc::c_void,
        size: libc::c_int,
        str: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn base64_decode(str: *const libc::c_char, data: *mut libc::c_void) -> libc::c_int;
    fn md5_init_ctx(ctx: *mut md5_ctx);
    fn md5_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut md5_ctx);
    fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn creds_get_username(this: CREDS) -> *mut libc::c_char;
    static mut CREDSIZE: size_t;
    fn creds_get_scheme(this: CREDS) -> SCHEME;
    fn creds_get_password(this: CREDS) -> *mut libc::c_char;
    fn creds_get_realm(this: CREDS) -> *mut libc::c_char;
    fn new_array() -> ARRAY;
    fn array_destroy(this: ARRAY) -> ARRAY;
    fn array_npush(this: ARRAY, thing: *mut libc::c_void, len: size_t);
    fn array_get(this: ARRAY, index: libc::c_int) -> *mut libc::c_void;
    fn array_length(this: ARRAY) -> size_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pthread_rand_np(ctx: *mut libc::c_uint) -> libc::c_int;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xstrcat(arg1: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn DES_ecb_encrypt(
        input: *mut const_DES_cblock,
        output: *mut DES_cblock,
        ks: *mut DES_key_schedule,
        enc: libc::c_int,
    );
    fn DES_set_odd_parity(key: *mut DES_cblock);
    fn DES_set_key(
        key: *mut const_DES_cblock,
        schedule: *mut DES_key_schedule,
    ) -> libc::c_int;
    fn MD4_Init(c: *mut MD4_CTX) -> libc::c_int;
    fn MD4_Update(
        c: *mut MD4_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn MD4_Final(md: *mut libc::c_uchar, c: *mut MD4_CTX) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type md5_uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub A: md5_uint32,
    pub B: md5_uint32,
    pub C: md5_uint32,
    pub D: md5_uint32,
    pub total: [md5_uint32; 2],
    pub buflen: md5_uint32,
    pub buffer: [libc::c_char; 128],
}
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type SCHEME = libc::c_uint;
pub const PROXY: SCHEME = 4;
pub const FTP: SCHEME = 3;
pub const HTTPS: SCHEME = 2;
pub const HTTP: SCHEME = 1;
pub const UNSUPPORTED: SCHEME = 0;
pub type CREDS = *mut CREDS_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUTH_T {
    pub creds: ARRAY,
    pub okay: BOOLEAN,
    pub basic: C2RustUnnamed_2,
    pub digest: C2RustUnnamed_1,
    pub ntlm: C2RustUnnamed_0,
    pub proxy: C2RustUnnamed,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub required: BOOLEAN,
    pub hostname: *mut libc::c_char,
    pub port: libc::c_int,
    pub encode: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub state: STATE,
    pub encode: *mut libc::c_char,
    pub ready: BOOLEAN,
    pub nonce: [libc::c_uchar; 8],
}
pub type STATE = libc::c_uint;
pub const TYPE_L: STATE = 4;
pub const TYPE_3: STATE = 3;
pub const TYPE_2: STATE = 2;
pub const TYPE_1: STATE = 1;
pub const TYPE_N: STATE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub encode: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub encode: *mut libc::c_char,
}
pub type ARRAY = *mut ARRAY_T;
pub type AUTH = *mut AUTH_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DIGEST_CRED {
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub cnonce_value: *mut libc::c_char,
    pub h_a1: *mut libc::c_char,
    pub nc: [libc::c_char; 9],
    pub nc_value: libc::c_uint,
}
pub type DCRED = DIGEST_CRED;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DIGEST_CHLG {
    pub realm: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub nonce: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub stale: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub qop: *mut libc::c_char,
}
pub type DCHLG = DIGEST_CHLG;
pub type DES_key_schedule = DES_ks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DES_ks {
    pub ks: [C2RustUnnamed_3; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub cblock: DES_cblock,
    pub deslong: [DES_LONG; 2],
}
pub type DES_LONG = libc::c_uint;
pub type DES_cblock = [libc::c_uchar; 8];
pub type const_DES_cblock = [libc::c_uchar; 8];
pub type MD4_CTX = MD4state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD4state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub const QOP: KEY_HEADER_E = 6;
pub const ALGORITHM: KEY_HEADER_E = 5;
pub const STALE: KEY_HEADER_E = 4;
pub const OPAQUE: KEY_HEADER_E = 3;
pub const NONCE: KEY_HEADER_E = 2;
pub const DOMAIN: KEY_HEADER_E = 1;
pub const REALM: KEY_HEADER_E = 0;
pub type KEY_HEADER_E = libc::c_uint;
pub const UNKNOWN: KEY_HEADER_E = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KEYPARSER {
    pub keyname: *const libc::c_char,
    pub keyval: KEY_HEADER_E,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut AUTHSIZE: size_t = ::std::mem::size_of::<AUTH_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_auth() -> AUTH {
    let mut this: AUTH = 0 as *mut AUTH_T;
    this = calloc(AUTHSIZE, 1 as libc::c_int as libc::c_ulong) as AUTH;
    (*this).creds = new_array();
    (*this).basic.encode = 0 as *mut libc::c_char;
    (*this).digest.encode = 0 as *mut libc::c_char;
    (*this).ntlm.encode = 0 as *mut libc::c_char;
    (*this).ntlm.state = TYPE_N;
    (*this).proxy.encode = 0 as *mut libc::c_char;
    return this;
}
pub unsafe extern "C" fn auth_destroy(mut this: AUTH) -> AUTH {
    (*this).creds = array_destroy((*this).creds);
    xfree((*this).basic.encode as *mut libc::c_void);
    xfree((*this).digest.encode as *mut libc::c_void);
    xfree((*this).ntlm.encode as *mut libc::c_void);
    xfree((*this).proxy.encode as *mut libc::c_void);
    xfree(this as *mut libc::c_void);
    return 0 as AUTH;
}
pub unsafe extern "C" fn auth_add(mut this: AUTH, mut creds: CREDS) {
    array_npush((*this).creds, creds as *mut libc::c_void, CREDSIZE);
}
pub unsafe extern "C" fn auth_display(mut this: AUTH, mut scheme: SCHEME) {
    let mut i: size_t = 0;
    let mut space: [libc::c_char; 33] = *::std::mem::transmute::<
        &[u8; 33],
        &mut [libc::c_char; 33],
    >(b"                                \0");
    let mut first: BOOLEAN = boolean_true;
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if creds_get_scheme(tmp) as libc::c_uint == scheme as libc::c_uint {
            printf(
                b"%scredentials:  %s:%s:%s\n\0" as *const u8 as *const libc::c_char,
                if first as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    space.as_mut_ptr() as *const libc::c_char
                },
                creds_get_username(tmp),
                creds_get_password(tmp),
                creds_get_realm(tmp),
            );
            first = boolean_false;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn auth_get_basic_header(
    mut this: AUTH,
    mut scheme: SCHEME,
) -> *mut libc::c_char {
    if scheme as libc::c_uint == PROXY as libc::c_int as libc::c_uint {
        return (*this).proxy.encode
    } else {
        return (*this).basic.encode
    };
}
pub unsafe extern "C" fn auth_set_basic_header(
    mut this: AUTH,
    mut scheme: SCHEME,
    mut realm: *mut libc::c_char,
) -> BOOLEAN {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if realm.is_null() {
            break;
        }
        if strmatch(creds_get_realm(tmp), realm) as u64 != 0 {
            if creds_get_scheme(tmp) as libc::c_uint == scheme as libc::c_uint {
                return __basic_header(this, scheme, tmp);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp_0: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(
            creds_get_realm(tmp_0),
            b"any\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            if creds_get_scheme(tmp_0) as libc::c_uint == scheme as libc::c_uint {
                return __basic_header(this, scheme, tmp_0);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return boolean_false;
}
pub unsafe extern "C" fn auth_set_ntlm_header(
    mut this: AUTH,
    mut scheme: SCHEME,
    mut header: *mut libc::c_char,
    mut realm: *mut libc::c_char,
) -> BOOLEAN {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if realm.is_null() {
            break;
        }
        if strmatch(creds_get_realm(tmp), realm) as u64 != 0 {
            if creds_get_scheme(tmp) as libc::c_uint
                == HTTP as libc::c_int as libc::c_uint
                || creds_get_scheme(tmp) as libc::c_uint
                    == HTTPS as libc::c_int as libc::c_uint
            {
                return __ntlm_header(this, scheme, header, tmp);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp_0: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(
            creds_get_realm(tmp_0),
            b"any\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            if creds_get_scheme(tmp_0) as libc::c_uint
                == HTTP as libc::c_int as libc::c_uint
                || creds_get_scheme(tmp_0) as libc::c_uint
                    == HTTPS as libc::c_int as libc::c_uint
            {
                return __ntlm_header(this, scheme, header, tmp_0);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return boolean_false;
}
pub unsafe extern "C" fn auth_get_ntlm_header(
    mut this: AUTH,
    mut scheme: SCHEME,
) -> *mut libc::c_char {
    if scheme as libc::c_uint == PROXY as libc::c_int as libc::c_uint {
        return (*this).proxy.encode
    } else {
        return (*this).ntlm.encode
    };
}
pub unsafe extern "C" fn auth_get_digest_header(
    mut this: AUTH,
    mut scheme: SCHEME,
    mut chlg: *mut DCHLG,
    mut cred: *mut DCRED,
    mut method: *const libc::c_char,
    mut uri: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut cnonce: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nonce_count: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qop: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut request_digest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut h_a1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut h_a2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opaque: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if chlg.is_null() || cred.is_null() {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !chlg.is_null() && !((*chlg).qop).is_null() {
        nonce_count = xstrcat(
            b", nc=\0" as *const u8 as *const libc::c_char,
            ((*cred).nc).as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        cnonce = xstrcat(
            b", cnonce=\"\0" as *const u8 as *const libc::c_char,
            (*cred).cnonce_value,
            b"\"\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        h_a1 = __get_h_a1(chlg, cred, (*chlg).nonce);
        if h_a1.is_null() {
            fprintf(
                stderr,
                b"error calling __get_h_a1\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut libc::c_char;
        }
        if __str_list_contains(
            (*chlg).qop,
            b"auth\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        ) as u64 != 0
        {
            qop = xstrdup(b", qop=auth\0" as *const u8 as *const libc::c_char);
            tmp = xstrcat(
                method,
                b":\0" as *const u8 as *const libc::c_char,
                uri,
                0 as *mut libc::c_void,
            );
            h_a2 = __get_md5_str(tmp);
            xfree(tmp as *mut libc::c_void);
            tmp = xstrcat(
                h_a1,
                b":\0" as *const u8 as *const libc::c_char,
                (*chlg).nonce,
                b":\0" as *const u8 as *const libc::c_char,
                ((*cred).nc).as_mut_ptr(),
                b":\0" as *const u8 as *const libc::c_char,
                (*cred).cnonce_value,
                b":auth:\0" as *const u8 as *const libc::c_char,
                h_a2,
                0 as *mut libc::c_void,
            );
            request_digest = __get_md5_str(tmp);
            xfree(tmp as *mut libc::c_void);
            response = xstrcat(
                b", response=\"\0" as *const u8 as *const libc::c_char,
                request_digest,
                b"\"\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
        } else {
            fprintf(
                stderr,
                b"error quality of protection not supported: %s\n\0" as *const u8
                    as *const libc::c_char,
                (*chlg).qop,
            );
            return 0 as *mut libc::c_char;
        }
    } else {
        h_a1 = __get_h_a1(chlg, cred, b"\0" as *const u8 as *const libc::c_char);
        if h_a1.is_null() {
            NOTIFY(ERROR, b"__get_h_a1\n\0" as *const u8 as *const libc::c_char);
            return 0 as *mut libc::c_char;
        }
        tmp = xstrcat(
            method,
            b":\0" as *const u8 as *const libc::c_char,
            uri,
            0 as *mut libc::c_void,
        );
        h_a2 = __get_md5_str(tmp);
        xfree(tmp as *mut libc::c_void);
        tmp = xstrcat(
            h_a1,
            b":\0" as *const u8 as *const libc::c_char,
            (*chlg).nonce,
            b":\0" as *const u8 as *const libc::c_char,
            h_a2,
            0 as *mut libc::c_void,
        );
        request_digest = __get_md5_str(tmp);
        xfree(tmp as *mut libc::c_void);
        response = xstrcat(
            b" response=\"\0" as *const u8 as *const libc::c_char,
            request_digest,
            b"\"\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    if !chlg.is_null() && !((*chlg).opaque).is_null() {
        opaque = xstrcat(
            b", opaque=\"\0" as *const u8 as *const libc::c_char,
            (*chlg).opaque,
            b"\"\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    result = xstrcat(
        b"Digest username=\"\0" as *const u8 as *const libc::c_char,
        (*cred).username,
        b"\", realm=\"\0" as *const u8 as *const libc::c_char,
        (*chlg).realm,
        b"\", nonce=\"\0" as *const u8 as *const libc::c_char,
        (*chlg).nonce,
        b"\", uri=\"\0" as *const u8 as *const libc::c_char,
        uri,
        b"\", algorithm=\0" as *const u8 as *const libc::c_char,
        (*chlg).algorithm,
        response,
        if !opaque.is_null() {
            opaque as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !qop.is_null() {
            qop as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !nonce_count.is_null() {
            nonce_count as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !cnonce.is_null() {
            cnonce as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        0 as *mut libc::c_void,
    );
    (*cred).nc_value = ((*cred).nc_value).wrapping_add(1);
    (*cred).nc_value;
    snprintf(
        ((*cred).nc).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"%.8x\0" as *const u8 as *const libc::c_char,
        (*cred).nc_value,
    );
    if 0 as libc::c_int
        == strcasecmp(b"MD5\0" as *const u8 as *const libc::c_char, (*chlg).algorithm)
    {
        xfree(h_a1 as *mut libc::c_void);
    }
    xfree(nonce_count as *mut libc::c_void);
    xfree(cnonce as *mut libc::c_void);
    xfree(qop as *mut libc::c_void);
    xfree(response as *mut libc::c_void);
    xfree(request_digest as *mut libc::c_void);
    xfree(h_a2 as *mut libc::c_void);
    xfree(opaque as *mut libc::c_void);
    len = (strlen(result)).wrapping_add(32 as libc::c_int as libc::c_ulong);
    if scheme as libc::c_uint == PROXY as libc::c_int as libc::c_uint {
        (*this).proxy.encode = xmalloc(len) as *mut libc::c_char;
        memset((*this).proxy.encode as *mut libc::c_void, '\0' as i32, len);
        snprintf(
            (*this).proxy.encode,
            len,
            b"Proxy-Authorization: %s\r\n\0" as *const u8 as *const libc::c_char,
            result,
        );
        xfree(result as *mut libc::c_void);
        return (*this).proxy.encode;
    } else {
        (*this).digest.encode = xmalloc(len) as *mut libc::c_char;
        memset((*this).digest.encode as *mut libc::c_void, '\0' as i32, len);
        snprintf(
            (*this).digest.encode,
            len,
            b"Authorization: %s\r\n\0" as *const u8 as *const libc::c_char,
            result,
        );
        xfree(result as *mut libc::c_void);
        return (*this).digest.encode;
    };
}
pub unsafe extern "C" fn auth_set_digest_header(
    mut this: AUTH,
    mut chlg: *mut *mut DCHLG,
    mut cred: *mut *mut DCRED,
    mut rand: *mut libc::c_uint,
    mut realm: *mut libc::c_char,
    mut str: *mut libc::c_char,
) -> BOOLEAN {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if realm.is_null() {
            break;
        }
        if strmatch(creds_get_realm(tmp), realm) as u64 != 0 {
            *chlg = __digest_challenge(str);
            *cred = __digest_credentials(tmp, rand);
            if (*cred).is_null() || (*chlg).is_null() {
                return boolean_false;
            }
            return boolean_true;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp_0: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(
            creds_get_realm(tmp_0),
            b"any\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            *chlg = __digest_challenge(str);
            *cred = __digest_credentials(tmp_0, rand);
            if (*cred).is_null() || (*chlg).is_null() {
                return boolean_false;
            }
            return boolean_true;
        }
        i = i.wrapping_add(1);
        i;
    }
    return boolean_false;
}
pub unsafe extern "C" fn auth_get_proxy_required(mut this: AUTH) -> BOOLEAN {
    if this.is_null() {
        return boolean_false;
    }
    return (*this).proxy.required;
}
pub unsafe extern "C" fn auth_get_proxy_host(mut this: AUTH) -> *mut libc::c_char {
    return (*this).proxy.hostname;
}
pub unsafe extern "C" fn auth_get_proxy_port(mut this: AUTH) -> libc::c_int {
    return (*this).proxy.port;
}
pub unsafe extern "C" fn auth_set_proxy_required(mut this: AUTH, mut required: BOOLEAN) {
    (*this).proxy.required = required;
}
pub unsafe extern "C" fn auth_set_proxy_host(
    mut this: AUTH,
    mut host: *mut libc::c_char,
) {
    (*this).proxy.hostname = xstrdup(host);
    (*this).proxy.required = boolean_true;
}
pub unsafe extern "C" fn auth_set_proxy_port(mut this: AUTH, mut port: libc::c_int) {
    (*this).proxy.port = port;
}
pub unsafe extern "C" fn auth_get_ftp_username(
    mut this: AUTH,
    mut realm: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(creds_get_realm(tmp), realm) as u64 != 0 {
            if creds_get_scheme(tmp) as libc::c_uint
                == FTP as libc::c_int as libc::c_uint
            {
                return creds_get_username(tmp);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp_0: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(
            creds_get_realm(tmp_0),
            b"any\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            if creds_get_scheme(tmp_0) as libc::c_uint
                == FTP as libc::c_int as libc::c_uint
            {
                return creds_get_username(tmp_0);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn auth_get_ftp_password(
    mut this: AUTH,
    mut realm: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(creds_get_realm(tmp), realm) as u64 != 0 {
            if creds_get_scheme(tmp) as libc::c_uint
                == FTP as libc::c_int as libc::c_uint
            {
                return creds_get_password(tmp);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < array_length((*this).creds) {
        let mut tmp_0: CREDS = array_get((*this).creds, i as libc::c_int) as CREDS;
        if strmatch(
            creds_get_realm(tmp_0),
            b"any\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            if creds_get_scheme(tmp_0) as libc::c_uint
                == FTP as libc::c_int as libc::c_uint
            {
                return creds_get_password(tmp_0);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
unsafe extern "C" fn __basic_header(
    mut this: AUTH,
    mut scheme: SCHEME,
    mut creds: CREDS,
) -> BOOLEAN {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut ret: BOOLEAN = boolean_true;
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    pthread_mutex_lock(&mut (*this).lock);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s:%s\0" as *const u8 as *const libc::c_char,
        creds_get_username(creds),
        creds_get_password(creds),
    );
    if scheme as libc::c_uint == PROXY as libc::c_int as libc::c_uint {
        xfree((*this).proxy.encode as *mut libc::c_void);
        if base64_encode(
            buf.as_mut_ptr() as *const libc::c_void,
            strlen(buf.as_mut_ptr()) as libc::c_int,
            &mut hdr,
        ) < 0 as libc::c_int
        {
            ret = boolean_false;
        } else {
            len = (strlen(hdr)).wrapping_add(32 as libc::c_int as libc::c_ulong);
            (*this).proxy.encode = xmalloc(len) as *mut libc::c_char;
            memset((*this).proxy.encode as *mut libc::c_void, '\0' as i32, len);
            snprintf(
                (*this).proxy.encode,
                len,
                b"Proxy-Authorization: Basic %s\r\n\0" as *const u8
                    as *const libc::c_char,
                hdr,
            );
        }
    } else {
        xfree((*this).basic.encode as *mut libc::c_void);
        if base64_encode(
            buf.as_mut_ptr() as *const libc::c_void,
            strlen(buf.as_mut_ptr()) as libc::c_int,
            &mut hdr,
        ) < 0 as libc::c_int
        {
            ret = boolean_false;
        } else {
            len = (strlen(hdr)).wrapping_add(32 as libc::c_int as libc::c_ulong);
            (*this).basic.encode = xmalloc(len) as *mut libc::c_char;
            memset((*this).basic.encode as *mut libc::c_void, '\0' as i32, len);
            snprintf(
                (*this).basic.encode,
                len,
                b"Authorization: Basic %s\r\n\0" as *const u8 as *const libc::c_char,
                hdr,
            );
        }
    }
    pthread_mutex_unlock(&mut (*this).lock);
    return ret;
}
unsafe extern "C" fn __ntlm_header(
    mut this: AUTH,
    mut scheme: SCHEME,
    mut header: *const libc::c_char,
    mut creds: CREDS,
) -> BOOLEAN {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut final_0: size_t = 0 as libc::c_int as size_t;
    let mut domstr: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut srvstr: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut domlen: size_t = strlen(domstr);
    let mut srvlen: size_t = strlen(srvstr);
    let mut srvoff: size_t = 0;
    let mut domoff: size_t = 0;
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 8192] = [0; 8192];
    let mut buf: [libc::c_char; 256] = [0; 256];
    if strncasecmp(
        header,
        b"NTLM\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return boolean_false;
    }
    NOTIFY(
        DEBUG,
        b"Parsing NTLM header:  %d, %d, %s, %s\0" as *const u8 as *const libc::c_char,
        (*this).okay as libc::c_uint,
        scheme as libc::c_uint,
        header,
        creds_get_username(creds),
    );
    header = header.offset(4 as libc::c_int as isize);
    while *header as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*header as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        header = header.offset(1);
        header;
    }
    if *header != 0 {
        let mut size_0: ssize_t = 0;
        memset(tmp.as_mut_ptr() as *mut libc::c_void, '\0' as i32, strlen(header));
        size_0 = base64_decode(
            header,
            &mut tmp as *mut [libc::c_char; 8192] as *mut libc::c_void,
        ) as ssize_t;
        if size_0 < 0 as libc::c_int as libc::c_long {
            return boolean_false;
        }
        if size_0 >= 48 as libc::c_int as libc::c_long {
            memcpy(
                ((*this).ntlm.nonce).as_mut_ptr() as *mut libc::c_void,
                &mut *tmp.as_mut_ptr().offset(24 as libc::c_int as isize)
                    as *mut libc::c_char as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        }
        (*this).ntlm.state = TYPE_2;
    } else {
        if (*this).ntlm.state as libc::c_uint >= TYPE_1 as libc::c_int as libc::c_uint {
            return boolean_false;
        }
        (*this).ntlm.state = TYPE_1;
    }
    match (*this).ntlm.state as libc::c_uint {
        1 | 0 | 4 => {
            srvoff = 32 as libc::c_int as size_t;
            domoff = srvoff.wrapping_add(srvlen);
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"NTLMSSP%c\x01%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%s%s\0"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) & 0xff as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) >> 8 as libc::c_int
                    & 0xff as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) >> 16 as libc::c_int
                    & 0xff as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) >> 24 as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (srvlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (srvlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (srvlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (srvlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (srvoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (srvoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                srvstr,
                domstr,
            );
            size = (32 as libc::c_int as libc::c_ulong)
                .wrapping_add(srvlen)
                .wrapping_add(domlen);
            if base64_encode(
                buf.as_mut_ptr() as *const libc::c_void,
                size as libc::c_int,
                &mut hdr,
            ) < 0 as libc::c_int
            {
                return boolean_false;
            }
            final_0 = (strlen(hdr)).wrapping_add(23 as libc::c_int as libc::c_ulong);
            (*this).ntlm.encode = xmalloc(final_0) as *mut libc::c_char;
            (*this).ntlm.state = TYPE_2;
            memset((*this).ntlm.encode as *mut libc::c_void, '\0' as i32, final_0);
            snprintf(
                (*this).ntlm.encode,
                final_0,
                b"Authorization: NTLM %s\r\n\0" as *const u8 as *const libc::c_char,
                hdr,
            );
        }
        2 => {
            let mut lmrespoff: size_t = 0;
            let mut ntrespoff: size_t = 0;
            let mut usroff: size_t = 0;
            let mut lmresp: [libc::c_uchar; 24] = [0; 24];
            let mut ntresp: [libc::c_uchar; 24] = [0; 24];
            let mut usrlen: size_t = 0;
            let mut usr: *const libc::c_char = 0 as *const libc::c_char;
            usr = strchr(creds_get_username(creds), '\\' as i32);
            if usr.is_null() {
                usr = strchr(creds_get_username(creds), '/' as i32);
            }
            if !usr.is_null() {
                domstr = creds_get_username(creds);
                domlen = usr.offset_from(domstr) as libc::c_long as size_t;
                usr = usr.offset(1);
                usr;
            } else {
                usr = creds_get_username(creds);
            }
            usrlen = strlen(usr);
            __mkhash(
                creds_get_password(creds),
                &mut *((*this).ntlm.nonce)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
                lmresp.as_mut_ptr(),
                ntresp.as_mut_ptr(),
            );
            domoff = 64 as libc::c_int as size_t;
            usroff = domoff.wrapping_add(domlen);
            srvoff = usroff.wrapping_add(usrlen);
            lmrespoff = srvoff.wrapping_add(srvlen);
            ntrespoff = lmrespoff.wrapping_add(0x18 as libc::c_int as libc::c_ulong);
            size = snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"NTLMSSP%c\x03%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c\xFF\xFF%c%c\x01\x82%c%c\0"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (lmrespoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (lmrespoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (ntrespoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (ntrespoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (usrlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (usrlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (usrlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (usrlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (usroff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (usroff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (srvlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (srvlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (srvlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (srvlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (srvoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (srvoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) as size_t;
            size = 64 as libc::c_int as size_t;
            buf[63 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            buf[62 as libc::c_int as usize] = buf[63 as libc::c_int as usize];
            if size.wrapping_add(usrlen).wrapping_add(domlen)
                >= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
            {
                return boolean_false;
            }
            memcpy(
                &mut *buf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                domstr as *const libc::c_void,
                domlen,
            );
            size = (size as libc::c_ulong).wrapping_add(domlen) as size_t as size_t;
            memcpy(
                &mut *buf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                usr as *const libc::c_void,
                usrlen,
            );
            size = (size as libc::c_ulong).wrapping_add(usrlen) as size_t as size_t;
            if size
                < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(0x18 as libc::c_int as libc::c_ulong)
            {
                memcpy(
                    &mut *buf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    lmresp.as_mut_ptr() as *const libc::c_void,
                    0x18 as libc::c_int as libc::c_ulong,
                );
                size = (size as libc::c_ulong)
                    .wrapping_add(0x18 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
            if size
                < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(0x18 as libc::c_int as libc::c_ulong)
            {
                memcpy(
                    &mut *buf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    ntresp.as_mut_ptr() as *const libc::c_void,
                    0x18 as libc::c_int as libc::c_ulong,
                );
                size = (size as libc::c_ulong)
                    .wrapping_add(0x18 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
            buf[56 as libc::c_int
                as usize] = (size & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_char;
            buf[57 as libc::c_int as usize] = (size >> 8 as libc::c_int) as libc::c_char;
            if base64_encode(
                buf.as_mut_ptr() as *const libc::c_void,
                size as libc::c_int,
                &mut hdr,
            ) < 0 as libc::c_int
            {
                return boolean_false;
            }
            (*this).ntlm.state = TYPE_3;
            (*this).ntlm.ready = boolean_true;
            final_0 = (strlen(hdr)).wrapping_add(23 as libc::c_int as libc::c_ulong);
            (*this)
                .ntlm
                .encode = xrealloc((*this).ntlm.encode as *mut libc::c_void, final_0)
                as *mut libc::c_char;
            memset((*this).ntlm.encode as *mut libc::c_void, '\0' as i32, final_0);
            snprintf(
                (*this).ntlm.encode,
                final_0,
                b"Authorization: NTLM %s\r\n\0" as *const u8 as *const libc::c_char,
                hdr,
            );
        }
        3 => {
            (*this).ntlm.ready = boolean_true;
        }
        _ => {}
    }
    return boolean_true;
}
unsafe extern "C" fn setup_des_key(
    mut key_56: *mut libc::c_uchar,
    mut ks: *mut DES_key_schedule,
) {
    let mut key: DES_cblock = [0; 8];
    key[0 as libc::c_int as usize] = *key_56.offset(0 as libc::c_int as isize);
    key[1 as libc::c_int
        as usize] = ((*key_56.offset(0 as libc::c_int as isize) as libc::c_int)
        << 7 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(1 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as libc::c_uchar;
    key[2 as libc::c_int
        as usize] = ((*key_56.offset(1 as libc::c_int as isize) as libc::c_int)
        << 6 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(2 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int)
        as libc::c_uchar;
    key[3 as libc::c_int
        as usize] = ((*key_56.offset(2 as libc::c_int as isize) as libc::c_int)
        << 5 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(3 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int)
        as libc::c_uchar;
    key[4 as libc::c_int
        as usize] = ((*key_56.offset(3 as libc::c_int as isize) as libc::c_int)
        << 4 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(4 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int)
        as libc::c_uchar;
    key[5 as libc::c_int
        as usize] = ((*key_56.offset(4 as libc::c_int as isize) as libc::c_int)
        << 3 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(5 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int)
        as libc::c_uchar;
    key[6 as libc::c_int
        as usize] = ((*key_56.offset(5 as libc::c_int as isize) as libc::c_int)
        << 2 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(6 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int)
        as libc::c_uchar;
    key[7 as libc::c_int
        as usize] = ((*key_56.offset(6 as libc::c_int as isize) as libc::c_int)
        << 1 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    DES_set_odd_parity(&mut key);
    DES_set_key(&mut key, ks);
}
unsafe extern "C" fn calc_resp(
    mut keys: *mut libc::c_uchar,
    mut plaintext: *mut libc::c_uchar,
    mut results: *mut libc::c_uchar,
) {
    let mut ks: DES_key_schedule = DES_key_schedule {
        ks: [C2RustUnnamed_3 { cblock: [0; 8] }; 16],
    };
    setup_des_key(keys, &mut ks);
    DES_ecb_encrypt(
        plaintext as *mut DES_cblock,
        results as *mut DES_cblock,
        &mut ks,
        1 as libc::c_int,
    );
    setup_des_key(keys.offset(7 as libc::c_int as isize), &mut ks);
    DES_ecb_encrypt(
        plaintext as *mut DES_cblock,
        results.offset(8 as libc::c_int as isize) as *mut DES_cblock,
        &mut ks,
        1 as libc::c_int,
    );
    setup_des_key(keys.offset(14 as libc::c_int as isize), &mut ks);
    DES_ecb_encrypt(
        plaintext as *mut DES_cblock,
        results.offset(16 as libc::c_int as isize) as *mut DES_cblock,
        &mut ks,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn __mkhash(
    mut password: *const libc::c_char,
    mut nonce: *mut libc::c_uchar,
    mut lmresp: *mut libc::c_uchar,
    mut ntresp: *mut libc::c_uchar,
) {
    let mut pw: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut lmbuffer: [libc::c_uchar; 21] = [0; 21];
    let mut ntbuffer: [libc::c_uchar; 21] = [0; 21];
    static mut magic: [libc::c_uchar; 8] = [
        0x4b as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x21 as libc::c_int as libc::c_uchar,
        0x40 as libc::c_int as libc::c_uchar,
        0x23 as libc::c_int as libc::c_uchar,
        0x24 as libc::c_int as libc::c_uchar,
        0x25 as libc::c_int as libc::c_uchar,
    ];
    let mut i: size_t = 0;
    let mut len: size_t = strlen(password);
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (if len < 7 as libc::c_int as libc::c_ulong {
            14 as libc::c_int as libc::c_ulong
        } else {
            len.wrapping_mul(2 as libc::c_int as libc::c_ulong)
        }) as usize,
    );
    pw = fresh0.leak().as_mut_ptr() as *mut libc::c_uchar;
    if len > 14 as libc::c_int as libc::c_ulong {
        len = 14 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < len {
        *pw
            .offset(
                i as isize,
            ) = (if (*password.offset(i as isize) as libc::c_uint)
            .wrapping_sub('a' as i32 as libc::c_uint)
            <= ('z' as i32 - 'a' as i32) as libc::c_uint
        {
            ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *password.offset(i as isize)
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*password.offset(i as isize) as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*password.offset(i as isize) as libc::c_int as isize);
                }
                __res
            })
        } else {
            *password.offset(i as isize) as libc::c_int
        }) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    while i < 14 as libc::c_int as libc::c_ulong {
        *pw.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    let mut ks: DES_key_schedule = DES_key_schedule {
        ks: [C2RustUnnamed_3 { cblock: [0; 8] }; 16],
    };
    setup_des_key(pw, &mut ks);
    DES_ecb_encrypt(
        magic.as_ptr() as *mut DES_cblock,
        lmbuffer.as_mut_ptr() as *mut DES_cblock,
        &mut ks,
        1 as libc::c_int,
    );
    setup_des_key(pw.offset(7 as libc::c_int as isize), &mut ks);
    DES_ecb_encrypt(
        magic.as_ptr() as *mut DES_cblock,
        lmbuffer.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut DES_cblock,
        &mut ks,
        1 as libc::c_int,
    );
    memset(
        lmbuffer.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        5 as libc::c_int as libc::c_ulong,
    );
    calc_resp(lmbuffer.as_mut_ptr(), nonce, lmresp);
    let mut MD4: MD4_CTX = MD4_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    len = strlen(password);
    i = 0 as libc::c_int as size_t;
    while i < len {
        *pw
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
            ) = *password.offset(i as isize) as libc::c_uchar;
        *pw
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    MD4_Init(&mut MD4);
    MD4_Update(
        &mut MD4,
        pw as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(len),
    );
    MD4_Final(ntbuffer.as_mut_ptr(), &mut MD4);
    memset(
        ntbuffer.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        5 as libc::c_int as libc::c_ulong,
    );
    calc_resp(ntbuffer.as_mut_ptr(), nonce, ntresp);
}
static mut keyparser: [KEYPARSER; 8] = [
    {
        let mut init = KEYPARSER {
            keyname: b"realm\0" as *const u8 as *const libc::c_char,
            keyval: REALM,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: b"domain\0" as *const u8 as *const libc::c_char,
            keyval: DOMAIN,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: b"nonce\0" as *const u8 as *const libc::c_char,
            keyval: NONCE,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: b"opaque\0" as *const u8 as *const libc::c_char,
            keyval: OPAQUE,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: b"stale\0" as *const u8 as *const libc::c_char,
            keyval: STALE,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: b"algorithm\0" as *const u8 as *const libc::c_char,
            keyval: ALGORITHM,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: b"qop\0" as *const u8 as *const libc::c_char,
            keyval: QOP,
        };
        init
    },
    {
        let mut init = KEYPARSER {
            keyname: 0 as *const libc::c_char,
            keyval: UNKNOWN,
        };
        init
    },
];
unsafe extern "C" fn __get_keyval(mut key: *const libc::c_char) -> KEY_HEADER_E {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(keyparser[i as usize].keyname).is_null() {
        if strcasecmp(key, keyparser[i as usize].keyname) == 0 {
            return keyparser[i as usize].keyval;
        }
        i += 1;
        i;
    }
    return UNKNOWN;
}
unsafe extern "C" fn __get_random_string(
    mut length: size_t,
    mut randseed: *mut libc::c_uint,
) -> *mut libc::c_char {
    let b64_alphabet: [libc::c_uchar; 65] = *::std::mem::transmute::<
        &[u8; 65],
        &[libc::c_uchar; 65],
    >(b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ./\0");
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: size_t = 0;
    result = xmalloc(
        (::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(length.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < length {
        *result
            .offset(
                i as isize,
            ) = (255.0f64
            * (pthread_rand_np(randseed) as libc::c_double
                / (2147483647 as libc::c_int as libc::c_double + 1.0f64))) as libc::c_int
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *result
            .offset(
                i as isize,
            ) = b64_alphabet[(*result.offset(i as isize) as libc::c_ulong)
            .wrapping_rem(
                (::std::mem::size_of::<[libc::c_uchar; 65]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            ) as usize];
        i = i.wrapping_add(1);
        i;
    }
    *result.offset(length as isize) = '\0' as i32 as libc::c_uchar;
    return result as *mut libc::c_char;
}
unsafe extern "C" fn __digest_credentials(
    mut creds: CREDS,
    mut randseed: *mut libc::c_uint,
) -> *mut DCRED {
    let mut result: *mut DCRED = 0 as *mut DCRED;
    result = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<DIGEST_CRED>() as libc::c_ulong,
    ) as *mut DCRED;
    (*result).username = xstrdup(creds_get_username(creds));
    (*result).password = xstrdup(creds_get_password(creds));
    (*result).cnonce_value = __get_random_string(16 as libc::c_int as size_t, randseed);
    (*result).nc_value = 1 as libc::c_uint;
    snprintf(
        ((*result).nc).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"%.8x\0" as *const u8 as *const libc::c_char,
        (*result).nc_value,
    );
    (*result).h_a1 = 0 as *mut libc::c_char;
    return result;
}
unsafe extern "C" fn __digest_challenge(
    mut challenge: *const libc::c_char,
) -> *mut DCHLG {
    let mut result: *mut DCHLG = 0 as *mut DCHLG;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut keyval: KEY_HEADER_E = REALM;
    result = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<DIGEST_CHLG>() as libc::c_ulong,
    ) as *mut DCHLG;
    end = challenge;
    beg = end;
    while *(*__ctype_b_loc()).offset(*end as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        && *end as libc::c_int != 0
    {
        end = end.offset(1);
        end;
    }
    if strncasecmp(
        b"Digest\0" as *const u8 as *const libc::c_char,
        beg,
        end.offset_from(beg) as libc::c_long as libc::c_ulong,
    ) != 0
    {
        fprintf(
            stderr,
            b"no Digest keyword in challenge [%s]\n\0" as *const u8
                as *const libc::c_char,
            challenge,
        );
        return 0 as *mut DCHLG;
    }
    beg = end;
    while *(*__ctype_b_loc()).offset(*beg as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        beg = beg.offset(1);
        beg;
    }
    while *beg as libc::c_int != '\0' as i32 {
        while *(*__ctype_b_loc()).offset(*beg as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            beg = beg.offset(1);
            beg;
        }
        end = beg;
        while *end as libc::c_int != '=' as i32 && *end as libc::c_int != ',' as i32
            && *end as libc::c_int != '\0' as i32
            && *(*__ctype_b_loc()).offset(*end as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            end = end.offset(1);
            end;
        }
        key = xmalloc(
            (end.offset(1 as libc::c_int as isize).offset_from(beg) as libc::c_long
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            key as *mut libc::c_void,
            beg as *const libc::c_void,
            end.offset_from(beg) as libc::c_long as libc::c_ulong,
        );
        *key
            .offset(
                end.offset_from(beg) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        beg = end;
        while *(*__ctype_b_loc()).offset(*beg as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            beg = beg.offset(1);
            beg;
        }
        val = 0 as *mut libc::c_char;
        if *beg as libc::c_int == '=' as i32 {
            beg = beg.offset(1);
            beg;
            while *(*__ctype_b_loc()).offset(*beg as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                beg = beg.offset(1);
                beg;
            }
            if *beg as libc::c_int == '"' as i32 {
                beg = beg.offset(1);
                beg;
                end = beg;
                while *end as libc::c_int != '"' as i32
                    && *end as libc::c_int != '\0' as i32
                {
                    if *end as libc::c_int == '\\' as i32
                        && *end.offset(1 as libc::c_int as isize) as libc::c_int
                            != '\0' as i32
                    {
                        end = end.offset(1);
                        end;
                    }
                    end = end.offset(1);
                    end;
                }
                val = xmalloc(
                    (end.offset(1 as libc::c_int as isize).offset_from(beg)
                        as libc::c_long as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                memcpy(
                    val as *mut libc::c_void,
                    beg as *const libc::c_void,
                    end.offset_from(beg) as libc::c_long as libc::c_ulong,
                );
                *val
                    .offset(
                        end.offset_from(beg) as libc::c_long as isize,
                    ) = '\0' as i32 as libc::c_char;
                beg = end;
                if *beg as libc::c_int != '\0' as i32 {
                    beg = beg.offset(1);
                    beg;
                }
            } else {
                end = beg;
                while *end as libc::c_int != ',' as i32
                    && *end as libc::c_int != '\0' as i32
                    && *(*__ctype_b_loc()).offset(*end as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    end = end.offset(1);
                    end;
                }
                val = xmalloc(
                    (end.offset(1 as libc::c_int as isize).offset_from(beg)
                        as libc::c_long as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                memcpy(
                    val as *mut libc::c_void,
                    beg as *const libc::c_void,
                    end.offset_from(beg) as libc::c_long as libc::c_ulong,
                );
                *val
                    .offset(
                        end.offset_from(beg) as libc::c_long as isize,
                    ) = '\0' as i32 as libc::c_char;
                beg = end;
            }
        }
        while *beg as libc::c_int != ',' as i32 && *beg as libc::c_int != '\0' as i32 {
            beg = beg.offset(1);
            beg;
        }
        if *beg as libc::c_int != '\0' as i32 {
            beg = beg.offset(1);
            beg;
        }
        keyval = __get_keyval(key);
        match keyval as libc::c_uint {
            0 => {
                (*result).realm = val;
            }
            1 => {
                (*result).domain = val;
            }
            2 => {
                (*result).nonce = val;
            }
            3 => {
                (*result).opaque = val;
            }
            4 => {
                (*result).stale = val;
            }
            5 => {
                (*result).algorithm = val;
            }
            6 => {
                (*result).qop = val;
            }
            _ => {
                fprintf(
                    stderr,
                    b"unknown key [%s]\n\0" as *const u8 as *const libc::c_char,
                    key,
                );
                xfree(val as *mut libc::c_void);
            }
        }
        xfree(key as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn __get_md5_str(mut buf: *const libc::c_char) -> *mut libc::c_char {
    let mut hex: *const libc::c_char = b"0123456789abcdef\0" as *const u8
        as *const libc::c_char;
    let mut ctx: md5_ctx = md5_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 128],
    };
    let mut hash: [libc::c_uchar; 16] = [0; 16];
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    let mut i: libc::c_int = 0;
    length = strlen(buf);
    result = xmalloc(
        (33 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    md5_init_ctx(&mut ctx);
    md5_process_bytes(buf as *const libc::c_void, length, &mut ctx);
    md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
    i = 0 as libc::c_int;
    r = result;
    while i < 16 as libc::c_int {
        let fresh1 = r;
        r = r.offset(1);
        *fresh1 = *hex
            .offset((hash[i as usize] as libc::c_int >> 4 as libc::c_int) as isize);
        let fresh2 = r;
        r = r.offset(1);
        *fresh2 = *hex
            .offset((hash[i as usize] as libc::c_int & 0xf as libc::c_int) as isize);
        i += 1;
        i;
    }
    *r = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn __get_h_a1(
    mut chlg: *const DCHLG,
    mut cred: *mut DCRED,
    mut nonce_value: *const libc::c_char,
) -> *mut libc::c_char {
    let mut h_usrepa: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 as libc::c_int
        == strcasecmp(b"MD5\0" as *const u8 as *const libc::c_char, (*chlg).algorithm)
    {
        tmp = xstrcat(
            (*cred).username,
            b":\0" as *const u8 as *const libc::c_char,
            (*chlg).realm,
            b":\0" as *const u8 as *const libc::c_char,
            (*cred).password,
            0 as *mut libc::c_void,
        );
        h_usrepa = __get_md5_str(tmp);
        xfree(tmp as *mut libc::c_void);
        result = h_usrepa;
    } else if 0 as libc::c_int
        == strcasecmp(
            b"MD5-sess\0" as *const u8 as *const libc::c_char,
            (*chlg).algorithm,
        )
    {
        if ((*cred).h_a1).is_null() {
            tmp = xstrcat(
                (*cred).username,
                b":\0" as *const u8 as *const libc::c_char,
                (*chlg).realm,
                b":\0" as *const u8 as *const libc::c_char,
                (*cred).password,
                0 as *mut libc::c_void,
            );
            h_usrepa = __get_md5_str(tmp);
            xfree(tmp as *mut libc::c_void);
            tmp = xstrcat(
                h_usrepa,
                b":\0" as *const u8 as *const libc::c_char,
                nonce_value,
                b":\0" as *const u8 as *const libc::c_char,
                (*cred).cnonce_value,
                0 as *mut libc::c_void,
            );
            result = __get_md5_str(tmp);
            xfree(tmp as *mut libc::c_void);
            (*cred).h_a1 = result;
        } else {
            return (*cred).h_a1
        }
    } else {
        fprintf(
            stderr,
            b"invalid call to %s algorithm is [%s]\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"__get_h_a1\0"))
                .as_ptr(),
            (*chlg).algorithm,
        );
        return 0 as *mut libc::c_char;
    }
    return result;
}
unsafe extern "C" fn __str_list_contains(
    mut str: *const libc::c_char,
    mut pattern: *const libc::c_char,
    mut pattern_len: size_t,
) -> BOOLEAN {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ptr = str;
    loop {
        if 0 as libc::c_int == strncmp(ptr, pattern, pattern_len)
            && (',' as i32 == *ptr.offset(pattern_len as isize) as libc::c_int
                || '\0' as i32 == *ptr.offset(pattern_len as isize) as libc::c_int)
        {
            return boolean_true;
        }
        ptr = strchr(ptr, ',' as i32);
        if !ptr.is_null() {
            ptr = ptr.offset(1);
            ptr;
        }
        if ptr.is_null() {
            break;
        }
    }
    return boolean_false;
}
