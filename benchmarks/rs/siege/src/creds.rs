use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type SCHEME = libc::c_uint;
pub const PROXY: SCHEME = 4;
pub const FTP: SCHEME = 3;
pub const HTTPS: SCHEME = 2;
pub const HTTP: SCHEME = 1;
pub const UNSUPPORTED: SCHEME = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CREDS_T {
    pub scheme: SCHEME,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub realm: *mut libc::c_char,
}
pub type CREDS = *mut CREDS_T;
pub static mut CREDSIZE: size_t = ::std::mem::size_of::<CREDS_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_creds(
    mut scheme: SCHEME,
    mut str: *mut libc::c_char,
) -> CREDS {
    let mut this: CREDS = 0 as *mut CREDS_T;
    this = calloc(
        ::std::mem::size_of::<CREDS_T>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as CREDS;
    (*this).scheme = scheme;
    (*this).username = 0 as *mut libc::c_char;
    (*this).password = 0 as *mut libc::c_char;
    (*this).realm = 0 as *mut libc::c_char;
    __parse_input(this, str);
    return this;
}
pub unsafe extern "C" fn creds_destroy(mut this: CREDS) -> CREDS {
    xfree((*this).username as *mut libc::c_void);
    xfree((*this).password as *mut libc::c_void);
    xfree((*this).realm as *mut libc::c_void);
    xfree(this as *mut libc::c_void);
    return 0 as CREDS;
}
pub unsafe extern "C" fn creds_get_scheme(mut this: CREDS) -> SCHEME {
    return (*this).scheme;
}
pub unsafe extern "C" fn creds_get_username(mut this: CREDS) -> *mut libc::c_char {
    return (*this).username;
}
pub unsafe extern "C" fn creds_get_password(mut this: CREDS) -> *mut libc::c_char {
    return (*this).password;
}
pub unsafe extern "C" fn creds_get_realm(mut this: CREDS) -> *mut libc::c_char {
    return (*this).realm;
}
pub unsafe extern "C" fn creds_set_username(
    mut this: CREDS,
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
pub unsafe extern "C" fn creds_set_password(
    mut this: CREDS,
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
pub unsafe extern "C" fn creds_set_realm(mut this: CREDS, mut realm: *mut libc::c_char) {
    let mut len: size_t = strlen(realm);
    (*this)
        .realm = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memset(
        (*this).realm as *mut libc::c_void,
        '\0' as i32,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy((*this).realm as *mut libc::c_void, realm as *const libc::c_void, len);
}
unsafe extern "C" fn __parse_input(mut this: CREDS, mut str: *mut libc::c_char) {
    let mut usr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rlm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut any: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"any\0\0");
    tmp = str;
    usr = tmp;
    while *tmp as libc::c_int != 0 && *tmp as libc::c_int != ':' as i32
        && *tmp as libc::c_int != '\0' as i32
    {
        tmp = tmp.offset(1);
        tmp;
    }
    let fresh0 = tmp;
    tmp = tmp.offset(1);
    *fresh0 = 0 as libc::c_int as libc::c_char;
    pwd = tmp;
    while *tmp as libc::c_int != 0 && *tmp as libc::c_int != ':' as i32
        && *tmp as libc::c_int != '\0' as i32
    {
        tmp = tmp.offset(1);
        tmp;
    }
    if '\0' as i32 != *tmp as libc::c_int {
        let fresh1 = tmp;
        tmp = tmp.offset(1);
        *fresh1 = 0 as libc::c_int as libc::c_char;
        rlm = tmp;
    } else {
        rlm = 0 as *mut libc::c_char;
    }
    creds_set_username(this, usr);
    creds_set_password(this, pwd);
    creds_set_realm(this, if rlm.is_null() { any.as_mut_ptr() } else { rlm });
}
