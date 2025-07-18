use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn base64enc(dst: *mut libc::c_char, src: *const libc::c_void, count: size_t);
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub unsafe extern "C" fn basicauth_string(
    mut user: *const libc::c_char,
    mut pass: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> ssize_t {
    let mut tmp: [libc::c_char; 258] = [0; 258];
    let mut l: libc::c_int = 0;
    if user.is_null() || pass.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    l = snprintf(
        tmp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 258]>() as libc::c_ulong,
        b"%s:%s\0" as *const u8 as *const libc::c_char,
        user,
        pass,
    );
    if l < 0 as libc::c_int
        || l as libc::c_long
            >= ::std::mem::size_of::<[libc::c_char; 258]>() as libc::c_ulong as ssize_t
    {
        return 0 as libc::c_int as ssize_t;
    }
    if bufsize
        < (l as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint)
            .wrapping_div(3 as libc::c_int as libc::c_uint)
            .wrapping_mul(4 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        return 0 as libc::c_int as ssize_t;
    }
    base64enc(buf, tmp.as_mut_ptr() as *const libc::c_void, l as size_t);
    return ((l + 2 as libc::c_int) / 3 as libc::c_int * 4 as libc::c_int) as ssize_t;
}
pub unsafe extern "C" fn basicauth_add(
    mut authlist: *mut sblist,
    mut user: *const libc::c_char,
    mut pass: *const libc::c_char,
) {
    let mut b: [libc::c_char; 345] = [0; 345];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: ssize_t = 0;
    ret = basicauth_string(
        user,
        pass,
        b.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 345]>() as libc::c_ulong,
    );
    if ret == -(1 as libc::c_int) as libc::c_long {
        log_message(
            4 as libc::c_int,
            b"Illegal basicauth rule: missing user or pass\0" as *const u8
                as *const libc::c_char,
        );
        return;
    } else if ret == 0 as libc::c_int as libc::c_long {
        log_message(
            4 as libc::c_int,
            b"User / pass in basicauth rule too long\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    s = strdup(b.as_mut_ptr());
    if s.is_null()
        || sblist_add(authlist, &mut s as *mut *mut libc::c_char as *mut libc::c_void)
            == 0
    {
        free(s as *mut libc::c_void);
        s = 0 as *mut libc::c_char;
        log_message(
            3 as libc::c_int,
            b"Unable to allocate memory in basicauth_add()\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    log_message(
        6 as libc::c_int,
        b"Added basic auth user : %s\0" as *const u8 as *const libc::c_char,
        user,
    );
}
pub unsafe extern "C" fn basicauth_check(
    mut authlist: *mut sblist,
    mut authstring: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut entry: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if authlist.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*authlist).count {
        entry = sblist_get(authlist, i) as *mut *mut libc::c_char;
        if !entry.is_null() && strcmp(authstring, *entry) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
