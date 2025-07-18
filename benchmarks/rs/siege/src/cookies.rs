use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type HASH_T;
    pub type COOKIE_T;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn hash_destroy(this: HASH) -> HASH;
    static mut HASHSIZE: size_t;
    fn new_hash() -> HASH;
    fn hash_add(this: HASH, key: *mut libc::c_char, value: *mut libc::c_void);
    fn hash_nadd(
        this: HASH,
        key: *mut libc::c_char,
        val: *mut libc::c_void,
        len: size_t,
    );
    fn hash_get(this: HASH, key: *mut libc::c_char) -> *mut libc::c_void;
    fn chomp(str: *mut libc::c_char) -> *mut libc::c_char;
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn split(
        pattern: libc::c_char,
        s: *mut libc::c_char,
        n_words: *mut libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn split_free(split_0: *mut *mut libc::c_char, length: libc::c_int);
    fn cookie_to_string(this: COOKIE) -> *mut libc::c_char;
    fn cookie_expires_string(this: COOKIE) -> *mut libc::c_char;
    fn cookie_get_session(this: COOKIE) -> BOOLEAN;
    fn cookie_get_expires(this: COOKIE) -> time_t;
    fn cookie_get_domain(this: COOKIE) -> *mut libc::c_char;
    fn cookie_get_value(this: COOKIE) -> *mut libc::c_char;
    fn new_cookie(str: *mut libc::c_char, host: *mut libc::c_char) -> COOKIE;
    fn cookie_destroy(this: COOKIE) -> COOKIE;
    fn cookie_reset_value(this: COOKIE, str: *mut libc::c_char);
    fn cookie_get_name(this: COOKIE) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type HASH = *mut HASH_T;
pub type COOKIE = *mut COOKIE_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COOKIES_T {
    pub head: *mut NODE,
    pub size: size_t,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NODE {
    pub threadID: size_t,
    pub cookie: COOKIE,
    pub next: *mut NODE,
}
pub type COOKIES = *mut COOKIES_T;
pub unsafe extern "C" fn new_cookies() -> COOKIES {
    let mut len: libc::c_int = 0;
    let mut this: COOKIES = 0 as *mut COOKIES_T;
    let mut name: [libc::c_char; 20] = *::std::mem::transmute::<
        &[u8; 20],
        &mut [libc::c_char; 20],
    >(b"/.siege/cookies.txt\0");
    this = calloc(
        ::std::mem::size_of::<COOKIES_T>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as COOKIES;
    (*this).size = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    len = (if !p.is_null() { strlen(p) } else { 60 as libc::c_int as libc::c_ulong })
        as libc::c_int;
    len = (len as libc::c_ulong)
        .wrapping_add(
            (strlen(name.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    (*this)
        .file = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong),
    ) as *mut libc::c_char;
    memset((*this).file as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
    snprintf(
        (*this).file,
        len as libc::c_ulong,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        getenv(b"HOME\0" as *const u8 as *const libc::c_char),
        name.as_mut_ptr(),
    );
    return this;
}
pub unsafe extern "C" fn cookies_destroy(mut this: COOKIES) -> COOKIES {
    let mut cur: *mut NODE = 0 as *mut NODE;
    __save_cookies(this);
    cur = (*this).head;
    while !cur.is_null() {
        cur = __delete_node(cur);
    }
    xfree((*this).file as *mut libc::c_void);
    free(this as *mut libc::c_void);
    return 0 as COOKIES;
}
pub unsafe extern "C" fn cookies_add(
    mut this: COOKIES,
    mut str: *mut libc::c_char,
    mut host: *mut libc::c_char,
) -> BOOLEAN {
    let mut id: size_t = pthread_self();
    let mut hlen: libc::c_int = 0 as libc::c_int;
    let mut dlen: libc::c_int = 0 as libc::c_int;
    let mut cur: *mut NODE = 0 as *mut NODE;
    let mut pre: *mut NODE = 0 as *mut NODE;
    let mut new: *mut NODE = 0 as *mut NODE;
    let mut found: BOOLEAN = boolean_false;
    let mut valid: BOOLEAN = boolean_false;
    let mut oreo: COOKIE = new_cookie(str, host);
    if oreo.is_null() {
        return boolean_false;
    }
    if (cookie_get_name(oreo)).is_null() || (cookie_get_value(oreo)).is_null() {
        return boolean_false;
    }
    pre = (*this).head;
    cur = pre;
    while !cur.is_null() {
        let mut domainptr: *const libc::c_char = cookie_get_domain((*cur).cookie);
        if *domainptr as libc::c_int == '.' as i32 {
            domainptr = domainptr.offset(1);
            domainptr;
        }
        hlen = (if !host.is_null() {
            strlen(host)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        dlen = (if !domainptr.is_null() {
            strlen(domainptr)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        if strcasecmp(host, domainptr) == 0 {
            valid = boolean_true;
        }
        if valid as u64 == 0 && dlen < hlen
            && strcasecmp(host.offset((hlen - dlen) as isize), domainptr) == 0
        {
            valid = boolean_true;
        }
        if valid as libc::c_uint != 0 && (*cur).threadID == id
            && strcasecmp(cookie_get_name((*cur).cookie), cookie_get_name(oreo)) == 0
        {
            cookie_reset_value((*cur).cookie, cookie_get_value(oreo));
            oreo = cookie_destroy(oreo);
            found = boolean_true;
            break;
        } else {
            pre = cur;
            cur = (*cur).next;
        }
    }
    if found as u64 == 0 {
        new = malloc(::std::mem::size_of::<NODE>() as libc::c_ulong) as *mut NODE;
        (*new).threadID = id;
        (*new).cookie = oreo;
        (*new).next = cur;
        if cur == (*this).head {
            (*this).head = new;
        } else {
            (*pre).next = new;
        }
    }
    return boolean_true;
}
pub unsafe extern "C" fn cookies_delete(
    mut this: COOKIES,
    mut str: *mut libc::c_char,
) -> BOOLEAN {
    let mut cur: *mut NODE = 0 as *mut NODE;
    let mut pre: *mut NODE = 0 as *mut NODE;
    let mut ret: BOOLEAN = boolean_false;
    let mut id: pthread_t = pthread_self();
    pre = (*this).head;
    cur = pre;
    while !cur.is_null() {
        if (*cur).threadID == id {
            let mut name: *mut libc::c_char = cookie_get_name((*cur).cookie);
            if strcasecmp(name, str) == 0 {
                (*cur).cookie = cookie_destroy((*cur).cookie);
                (*pre).next = (*cur).next;
                if cur == (*this).head {
                    (*this).head = (*cur).next;
                    pre = (*this).head;
                } else {
                    (*pre).next = (*cur).next;
                }
                ret = boolean_true;
                break;
            }
        }
        pre = cur;
        cur = (*cur).next;
    }
    return ret;
}
pub unsafe extern "C" fn cookies_delete_all(mut this: COOKIES) -> BOOLEAN {
    let mut cur: *mut NODE = 0 as *mut NODE;
    let mut pre: *mut NODE = 0 as *mut NODE;
    let mut id: pthread_t = pthread_self();
    pre = (*this).head;
    cur = pre;
    while !cur.is_null() {
        if (*cur).threadID == id {
            (*cur).cookie = cookie_destroy((*cur).cookie);
            (*pre).next = (*cur).next;
            if cur == (*this).head {
                (*this).head = (*cur).next;
                pre = (*this).head;
            } else {
                (*pre).next = (*cur).next;
            }
        }
        pre = cur;
        cur = (*cur).next;
    }
    return boolean_true;
}
pub unsafe extern "C" fn cookies_header(
    mut this: COOKIES,
    mut host: *mut libc::c_char,
    mut newton: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut dlen: libc::c_int = 0;
    let mut hlen: libc::c_int = 0;
    let mut pre: *mut NODE = 0 as *mut NODE;
    let mut cur: *mut NODE = 0 as *mut NODE;
    let mut tmp: time_t = 0;
    let mut now: time_t = 0;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut oreo: [libc::c_char; 81920] = [0; 81920];
    let mut id: size_t = pthread_self();
    memset(
        oreo.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 81920]>() as libc::c_ulong,
    );
    hlen = strlen(host) as libc::c_int;
    tmp = time(0 as *mut time_t);
    gmtime_r(&mut tmp, &mut tm);
    tm.tm_isdst = -(1 as libc::c_int);
    now = mktime(&mut tm);
    let mut current_block_21: u64;
    pre = (*this).head;
    cur = pre;
    while !cur.is_null() {
        let mut domainptr: *const libc::c_char = cookie_get_domain((*cur).cookie);
        if *domainptr as libc::c_int == '.' as i32 {
            domainptr = domainptr.offset(1);
            domainptr;
        }
        dlen = (if !domainptr.is_null() {
            strlen(domainptr)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        if (*cur).threadID == id {
            if strcasecmp(domainptr, host) == 0 {
                if cookie_get_expires((*cur).cookie) <= now
                    && cookie_get_session((*cur).cookie) as libc::c_uint
                        != boolean_true as libc::c_int as libc::c_uint
                {
                    cookies_delete(this, cookie_get_name((*cur).cookie));
                    current_block_21 = 6937071982253665452;
                } else {
                    if strlen(oreo.as_mut_ptr()) > 0 as libc::c_int as libc::c_ulong {
                        strncat(
                            oreo.as_mut_ptr(),
                            b";\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 81920]>()
                                as libc::c_ulong)
                                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(strlen(oreo.as_mut_ptr())),
                        );
                    }
                    strncat(
                        oreo.as_mut_ptr(),
                        cookie_get_name((*cur).cookie),
                        (::std::mem::size_of::<[libc::c_char; 81920]>() as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(strlen(oreo.as_mut_ptr())),
                    );
                    strncat(
                        oreo.as_mut_ptr(),
                        b"=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 81920]>() as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(strlen(oreo.as_mut_ptr())),
                    );
                    strncat(
                        oreo.as_mut_ptr(),
                        cookie_get_value((*cur).cookie),
                        (::std::mem::size_of::<[libc::c_char; 81920]>() as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(strlen(oreo.as_mut_ptr())),
                    );
                    current_block_21 = 6057473163062296781;
                }
            } else {
                current_block_21 = 6057473163062296781;
            }
            match current_block_21 {
                6937071982253665452 => {}
                _ => {
                    if dlen < hlen
                        && strcasecmp(host.offset((hlen - dlen) as isize), domainptr)
                            == 0
                    {
                        if cookie_get_expires((*cur).cookie) <= now
                            && cookie_get_session((*cur).cookie) as libc::c_uint
                                != boolean_true as libc::c_int as libc::c_uint
                        {
                            cookies_delete(this, cookie_get_name((*cur).cookie));
                        } else {
                            if strlen(oreo.as_mut_ptr())
                                > 0 as libc::c_int as libc::c_ulong
                            {
                                strncat(
                                    oreo.as_mut_ptr(),
                                    b";\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 81920]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(strlen(oreo.as_mut_ptr())),
                                );
                            }
                            strncat(
                                oreo.as_mut_ptr(),
                                cookie_get_name((*cur).cookie),
                                (::std::mem::size_of::<[libc::c_char; 81920]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(strlen(oreo.as_mut_ptr())),
                            );
                            strncat(
                                oreo.as_mut_ptr(),
                                b"=\0" as *const u8 as *const libc::c_char,
                                (::std::mem::size_of::<[libc::c_char; 81920]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(strlen(oreo.as_mut_ptr())),
                            );
                            strncat(
                                oreo.as_mut_ptr(),
                                cookie_get_value((*cur).cookie),
                                (::std::mem::size_of::<[libc::c_char; 81920]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(strlen(oreo.as_mut_ptr())),
                            );
                        }
                    }
                }
            }
        }
        pre = cur;
        cur = (*cur).next;
    }
    if strlen(oreo.as_mut_ptr()) > 0 as libc::c_int as libc::c_ulong {
        strncpy(
            newton,
            b"Cookie: \0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        );
        strncat(newton, oreo.as_mut_ptr(), 4096 as libc::c_int as libc::c_ulong);
        strncat(
            newton,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        );
    }
    return newton;
}
pub unsafe extern "C" fn cookies_list(mut this: COOKIES) {
    let mut cur: *mut NODE = 0 as *mut NODE;
    let mut pre: *mut NODE = 0 as *mut NODE;
    pre = (*this).head;
    cur = pre;
    while !cur.is_null() {
        let mut tmp: COOKIE = (*cur).cookie;
        if !tmp.is_null() {
            printf(
                b"%lld: NAME: %s\n   VALUE: %s\n   Expires: %s\n\0" as *const u8
                    as *const libc::c_char,
                (*cur).threadID as libc::c_longlong,
                cookie_get_name(tmp),
                cookie_get_value(tmp),
                cookie_expires_string(tmp),
            );
        }
        pre = cur;
        cur = (*cur).next;
    }
}
unsafe extern "C" fn __delete_node(mut node: *mut NODE) -> *mut NODE {
    if node.is_null() {
        return 0 as *mut NODE;
    }
    let mut tmp: *mut NODE = (*node).next;
    (*node).cookie = cookie_destroy((*node).cookie);
    free(node as *mut libc::c_void);
    node = tmp;
    return node;
}
unsafe extern "C" fn __strip(mut str: *mut libc::c_char) {
    let mut ch: *mut libc::c_char = 0 as *mut libc::c_char;
    ch = strstr(str, b"#\0" as *const u8 as *const libc::c_char);
    if !ch.is_null() {
        *ch = '\0' as i32 as libc::c_char;
    }
    ch = strstr(str, b"\n\0" as *const u8 as *const libc::c_char);
    if !ch.is_null() {
        *ch = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn load_cookies(mut this: COOKIES) -> HASH {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut n: libc::c_int = -(1 as libc::c_int);
    let mut HOH: HASH = 0 as *mut HASH_T;
    let mut IDX: HASH = 0 as *mut HASH_T;
    let len: size_t = 4096 as libc::c_int as size_t;
    let mut line: [libc::c_char; 4096] = [0; 4096];
    if __exists((*this).file) as u64 == 0 {
        return 0 as HASH;
    }
    fp = fopen((*this).file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as HASH;
    }
    HOH = new_hash();
    IDX = new_hash();
    memset(line.as_mut_ptr() as *mut libc::c_void, '\0' as i32, len);
    while !(fgets(line.as_mut_ptr(), len as libc::c_int, fp)).is_null() {
        let mut p: *mut libc::c_char = strchr(line.as_mut_ptr(), '\n' as i32);
        if !p.is_null() {
            *p = '\0' as i32 as libc::c_char;
        } else {
            let mut i: libc::c_int = 0;
            i = fgetc(fp);
            if i != -(1 as libc::c_int) {
                loop {
                    i = fgetc(fp);
                    if !(i != -(1 as libc::c_int) && i != '\n' as i32) {
                        break;
                    }
                }
                line[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            }
        }
        __strip(line.as_mut_ptr());
        chomp(line.as_mut_ptr());
        if strlen(line.as_mut_ptr()) > 1 as libc::c_int as libc::c_ulong {
            let mut num: libc::c_int = 2 as libc::c_int;
            let mut pair: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            pair = split('|' as i32 as libc::c_char, line.as_mut_ptr(), &mut num);
            trim(*pair.offset(0 as libc::c_int as isize));
            trim(*pair.offset(1 as libc::c_int as isize));
            if !(*pair.offset(0 as libc::c_int as isize)).is_null()
                && !(*pair.offset(1 as libc::c_int as isize)).is_null()
            {
                if (hash_get(IDX, *pair.offset(0 as libc::c_int as isize))).is_null() {
                    let mut tmp: [libc::c_char; 1024] = [0; 1024];
                    n += 1 as libc::c_int;
                    memset(
                        tmp.as_mut_ptr() as *mut libc::c_void,
                        '\0' as i32,
                        1024 as libc::c_int as libc::c_ulong,
                    );
                    snprintf(
                        tmp.as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        n,
                    );
                    hash_add(
                        IDX,
                        *pair.offset(0 as libc::c_int as isize),
                        tmp.as_mut_ptr() as *mut libc::c_void,
                    );
                }
                let mut tmp_0: HASH = hash_get(
                    HOH,
                    hash_get(IDX, *pair.offset(0 as libc::c_int as isize))
                        as *mut libc::c_char,
                ) as HASH;
                if tmp_0.is_null() {
                    tmp_0 = new_hash();
                    hash_add(
                        tmp_0,
                        *pair.offset(1 as libc::c_int as isize),
                        *pair.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    );
                    hash_nadd(
                        HOH,
                        hash_get(IDX, *pair.offset(0 as libc::c_int as isize))
                            as *mut libc::c_char,
                        tmp_0 as *mut libc::c_void,
                        HASHSIZE,
                    );
                } else {
                    hash_add(
                        tmp_0,
                        *pair.offset(1 as libc::c_int as isize),
                        *pair.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    );
                }
            }
            split_free(pair, num);
        }
        memset(line.as_mut_ptr() as *mut libc::c_void, '\0' as i32, len);
    }
    fclose(fp);
    hash_destroy(IDX);
    return HOH;
}
unsafe extern "C" fn __save_cookies(mut this: COOKIES) -> BOOLEAN {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = (4096 as libc::c_int + 24 as libc::c_int) as size_t;
    let mut now: time_t = 0;
    let mut cur: *mut NODE = 0 as *mut NODE;
    now = time(0 as *mut time_t);
    fp = fopen((*this).file, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"ERROR: Unable to open cookies file: %s\n\0" as *const u8
                as *const libc::c_char,
            (*this).file,
        );
        return boolean_false;
    }
    fputs(b"#\n\0" as *const u8 as *const libc::c_char, fp);
    fputs(
        b"# Siege cookies file. You may edit this file to add cookies\n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    fputs(
        b"# manually but comments and formatting will be removed.    \n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    fputs(
        b"# All cookies that expire in the future will be preserved. \n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    fputs(
        b"# ---------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    line = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(len),
    ) as *mut libc::c_char;
    cur = (*this).head;
    while !cur.is_null() {
        let mut tmp: COOKIE = (*cur).cookie;
        if !tmp.is_null()
            && cookie_get_session(tmp) as libc::c_uint
                != boolean_true as libc::c_int as libc::c_uint
            && cookie_get_expires((*cur).cookie) >= now
        {
            memset(line as *mut libc::c_void, '\0' as i32, len);
            if !(cookie_to_string(tmp)).is_null() {
                snprintf(
                    line,
                    len,
                    b"%ld | %s\n\0" as *const u8 as *const libc::c_char,
                    (*cur).threadID,
                    cookie_to_string(tmp),
                );
            }
            fputs(line, fp);
        }
        cur = (*cur).next;
    }
    free(line as *mut libc::c_void);
    fclose(fp);
    return boolean_true;
}
unsafe extern "C" fn __exists(mut file: *mut libc::c_char) -> BOOLEAN {
    let mut fd: libc::c_int = 0;
    fd = open(file, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return boolean_false
    } else {
        close(fd);
        return boolean_true;
    };
}
