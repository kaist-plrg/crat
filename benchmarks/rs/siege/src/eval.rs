use ::libc;
extern "C" {
    pub type HASH_T;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn hash_get(this: HASH, key: *mut libc::c_char) -> *mut libc::c_void;
    fn hash_contains(this: HASH, key: *mut libc::c_char) -> BOOLEAN;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn substring(
        str: *mut libc::c_char,
        start: libc::c_int,
        len: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type HASH = *mut HASH_T;
pub unsafe extern "C" fn escape(mut buf: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fin: [libc::c_char; 40000] = [0; 40000];
    res = xrealloc(
        buf as *mut libc::c_void,
        (40000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if !res.is_null() {
        buf = res;
    }
    len = strlen(buf);
    while i < len && *buf.offset(i as isize) as libc::c_int != '\\' as i32 {
        i = i.wrapping_add(1);
        i;
    }
    while i < len {
        *buf
            .offset(
                i as isize,
            ) = *buf.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        i = i.wrapping_add(1);
        i;
    }
    *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    strncpy(fin.as_mut_ptr(), buf, len);
    memset(
        res as *mut libc::c_void,
        '\0' as i32,
        (40000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    strncpy(res, fin.as_mut_ptr(), strlen(fin.as_mut_ptr()));
    return res;
}
pub unsafe extern "C" fn evaluate(
    mut hash: HASH,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut ENV: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut final_0: [libc::c_char; 40000] = [0; 40000];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scan: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: *mut libc::c_char = xrealloc(
        buf as *mut libc::c_void,
        (40000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if !result.is_null() {
        buf = result;
    }
    scan = (strchr(buf, '$' as i32)).offset(1 as libc::c_int as isize);
    len = (strlen(buf))
        .wrapping_sub(strlen(scan))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if *scan.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
        || *scan.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
    {
        scan = scan.offset(1);
        scan;
    }
    ptr = scan as *mut libc::c_char;
    while *scan as libc::c_int != 0 && *scan as libc::c_int != '}' as i32
        && *scan as libc::c_int != ')' as i32 && *scan as libc::c_int != '/' as i32
    {
        scan = scan.offset(1);
        scan;
        x += 1;
        x;
    }
    if *scan.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32
        || *scan.offset(0 as libc::c_int as isize) as libc::c_int == ')' as i32
    {
        scan = scan.offset(1);
        scan;
    }
    string = substring(ptr, 0 as libc::c_int, x);
    if hash_contains(hash, string) as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if !(getenv(string)).is_null() {
            ENV = 1 as libc::c_int;
        } else {
            string = 0 as *mut libc::c_char;
        }
    }
    memset(
        final_0.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 40000]>() as libc::c_ulong,
    );
    strncpy(final_0.as_mut_ptr(), buf, len as libc::c_ulong);
    if !string.is_null() {
        strcat(
            final_0.as_mut_ptr(),
            if ENV == 0 as libc::c_int {
                hash_get(hash, string) as *mut libc::c_char
            } else {
                getenv(string)
            },
        );
    }
    strcat(final_0.as_mut_ptr(), scan);
    memset(
        result as *mut libc::c_void,
        '\0' as i32,
        (40000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    strncpy(result, final_0.as_mut_ptr(), strlen(final_0.as_mut_ptr()));
    xfree(string as *mut libc::c_void);
    return result;
}
