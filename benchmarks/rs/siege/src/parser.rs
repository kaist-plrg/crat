use ::libc;
extern "C" {
    pub type URL_T;
    pub type ARRAY_T;
    fn url_normalize(req: URL, location: *mut libc::c_char) -> URL;
    fn url_get_hostname(this: URL) -> *mut libc::c_char;
    fn url_get_absolute(this: URL) -> *mut libc::c_char;
    static mut URLSIZE: size_t;
    fn url_set_redirect(this: URL, redir: BOOLEAN);
    fn array_length(this: ARRAY) -> size_t;
    fn array_npush(this: ARRAY, thing: *mut libc::c_void, len: size_t);
    fn array_get(this: ARRAY, index: libc::c_int) -> *mut libc::c_void;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn endswith(suffix: *const libc::c_char, str: *const libc::c_char) -> BOOLEAN;
    fn startswith(pre: *const libc::c_char, str: *const libc::c_char) -> BOOLEAN;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type URL = *mut URL_T;
pub type ARRAY = *mut ARRAY_T;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn html_parser(
    mut array: ARRAY,
    mut base: URL,
    mut page: *mut libc::c_char,
) -> BOOLEAN {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    memset(
        tmp.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        4096 as libc::c_int as libc::c_ulong,
    );
    str = __xstrip(page, b"\\\0" as *const u8 as *const libc::c_char);
    ptr = str;
    if page.is_null() {
        return boolean_false;
    }
    if strlen(page) < 1 as libc::c_int as libc::c_ulong {
        return boolean_false;
    }
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr as libc::c_int == '<' as i32 {
            ptr = ptr.offset(1);
            ptr;
            if startswith(b"!--\0" as *const u8 as *const libc::c_char, ptr)
                as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
            {
                ptr = ptr.offset(3 as libc::c_int as isize);
                while *ptr as libc::c_int != '\0' as i32 {
                    if startswith(b"-->\0" as *const u8 as *const libc::c_char, ptr)
                        as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
                    {
                        ptr = ptr.offset(3 as libc::c_int as isize);
                        break;
                    } else {
                        ptr = ptr.offset(1);
                        ptr;
                    }
                }
            } else {
                i = 0 as libc::c_int;
                memset(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                );
                while *ptr as libc::c_int != '\0' as i32
                    && *ptr as libc::c_int != '>' as i32
                    && i < 4096 as libc::c_int - 1 as libc::c_int
                {
                    tmp[i as usize] = *ptr;
                    i += 1;
                    i;
                    ptr = ptr.offset(1);
                    ptr;
                }
                __parse_control(array, base, tmp.as_mut_ptr());
            }
        }
        ptr = ptr.offset(1);
        ptr;
    }
    xfree(str as *mut libc::c_void);
    return boolean_true;
}
unsafe extern "C" fn __add_url(mut array: ARRAY, mut U: URL) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut found: BOOLEAN = boolean_false;
    if U.is_null() || (url_get_hostname(U)).is_null()
        || strlen(url_get_hostname(U)) < 2 as libc::c_int as libc::c_ulong
    {
        return;
    }
    if !array.is_null() {
        i = 0 as libc::c_int;
        while i < array_length(array) as libc::c_int {
            let mut url: URL = array_get(array, i) as URL;
            if strmatch(url_get_absolute(U), url_get_absolute(url)) as u64 != 0 {
                found = boolean_true;
            }
            i += 1;
            i;
        }
    }
    if found as u64 == 0 {
        array_npush(array, U as *mut libc::c_void, URLSIZE);
    }
}
unsafe extern "C" fn __parse_control(
    mut array: ARRAY,
    mut base: URL,
    mut html: *mut libc::c_char,
) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    let mut top: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut debug: BOOLEAN = boolean_false;
    top = strtok_r(html, b" =\0" as *const u8 as *const libc::c_char, &mut aid);
    ptr = top;
    while !ptr.is_null() {
        if strncasecmp(
            ptr,
            b"href\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\"'\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            if !ptr.is_null() {
                memset(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    4096 as libc::c_int as libc::c_ulong,
                );
                strncpy(
                    tmp.as_mut_ptr(),
                    ptr,
                    (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                );
            }
        } else if strncasecmp(
            ptr,
            b"meta\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            while !ptr.is_null() {
                if strncasecmp(
                    ptr,
                    b"content\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    ptr = strtok_r(
                        0 as *mut libc::c_char,
                        b" =\0" as *const u8 as *const libc::c_char,
                        &mut aid,
                    );
                    while !ptr.is_null() {
                        if !(__strcasestr(
                            ptr,
                            b"url\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        {
                            ptr = strtok_r(
                                0 as *mut libc::c_char,
                                b" \"'\0" as *const u8 as *const libc::c_char,
                                &mut aid,
                            );
                            if !ptr.is_null() {
                                let mut U: URL = url_normalize(base, ptr);
                                url_set_redirect(U, boolean_true);
                                if debug as u64 != 0 {
                                    printf(
                                        b"1.) Adding: %s\n\0" as *const u8 as *const libc::c_char,
                                        url_get_absolute(U),
                                    );
                                }
                                __add_url(array, U);
                            }
                        }
                        ptr = strtok_r(
                            0 as *mut libc::c_char,
                            b" =\0" as *const u8 as *const libc::c_char,
                            &mut aid,
                        );
                    }
                }
                ptr = strtok_r(
                    0 as *mut libc::c_char,
                    b" =\0" as *const u8 as *const libc::c_char,
                    &mut aid,
                );
            }
        } else if strncasecmp(
            ptr,
            b"img\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            if !ptr.is_null() && !aid.is_null() {
                if strncasecmp(
                    aid,
                    b"\"\"\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    continue;
                }
                if strncasecmp(
                    ptr,
                    b"src\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    ptr = strtok_r(
                        0 as *mut libc::c_char,
                        b" \"'\0" as *const u8 as *const libc::c_char,
                        &mut aid,
                    );
                    if !ptr.is_null() {
                        if strncasecmp(
                            ptr,
                            b"data:image\0" as *const u8 as *const libc::c_char,
                            10 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                            continue;
                        }
                        let mut U_0: URL = url_normalize(base, ptr);
                        if debug as u64 != 0 {
                            printf(
                                b"2.) Adding: %s\n\0" as *const u8 as *const libc::c_char,
                                url_get_absolute(U_0),
                            );
                        }
                        if endswith(
                            b"+\0" as *const u8 as *const libc::c_char,
                            url_get_absolute(U_0),
                        ) as u64 == 0
                        {
                            __add_url(array, U_0);
                        }
                    }
                } else {
                    ptr = strtok_r(
                        0 as *mut libc::c_char,
                        b" =\0" as *const u8 as *const libc::c_char,
                        &mut aid,
                    );
                    while !ptr.is_null() {
                        if !ptr.is_null()
                            && strncasecmp(
                                ptr,
                                b"src\0" as *const u8 as *const libc::c_char,
                                3 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                        {
                            ptr = strtok_r(
                                0 as *mut libc::c_char,
                                b" \"'\0" as *const u8 as *const libc::c_char,
                                &mut aid,
                            );
                            if !ptr.is_null()
                                && strlen(ptr) > 1 as libc::c_int as libc::c_ulong
                                && strncasecmp(
                                    ptr,
                                    b"data:image\0" as *const u8 as *const libc::c_char,
                                    10 as libc::c_int as libc::c_ulong,
                                ) != 0
                            {
                                let mut U_1: URL = url_normalize(base, ptr);
                                if debug as u64 != 0 {
                                    printf(
                                        b"3.) Adding: %s\n\0" as *const u8 as *const libc::c_char,
                                        url_get_absolute(U_1),
                                    );
                                }
                                __add_url(array, U_1);
                            }
                        }
                        ptr = strtok_r(
                            0 as *mut libc::c_char,
                            b" =\0" as *const u8 as *const libc::c_char,
                            &mut aid,
                        );
                    }
                }
            }
        } else if strncasecmp(
            ptr,
            b"link\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut okay: BOOLEAN = boolean_false;
            let mut buf: [libc::c_char; 2048] = [0; 2048];
            let mut current_block_61: u64;
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            while !ptr.is_null() {
                if strncasecmp(
                    ptr,
                    b"rel\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    ptr = strtok_r(
                        0 as *mut libc::c_char,
                        b" =\"'\0" as *const u8 as *const libc::c_char,
                        &mut aid,
                    );
                    if ptr.is_null() {
                        current_block_61 = 7990025728955927862;
                    } else {
                        if strncasecmp(
                            ptr,
                            b"stylesheet\0" as *const u8 as *const libc::c_char,
                            10 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            okay = boolean_true;
                        }
                        if strncasecmp(
                            ptr,
                            b"next\0" as *const u8 as *const libc::c_char,
                            4 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            okay = boolean_false;
                        }
                        if strncasecmp(
                            ptr,
                            b"alternate\0" as *const u8 as *const libc::c_char,
                            9 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            okay = boolean_false;
                        }
                        current_block_61 = 15594603006322722090;
                    }
                } else {
                    current_block_61 = 15594603006322722090;
                }
                match current_block_61 {
                    15594603006322722090 => {
                        if strncasecmp(
                            ptr,
                            b"href\0" as *const u8 as *const libc::c_char,
                            4 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            ptr = strtok_r(
                                0 as *mut libc::c_char,
                                b" \"'\0" as *const u8 as *const libc::c_char,
                                &mut aid,
                            );
                            if !ptr.is_null() {
                                memset(
                                    buf.as_mut_ptr() as *mut libc::c_void,
                                    '\0' as i32,
                                    ::std::mem::size_of::<[libc::c_char; 2048]>()
                                        as libc::c_ulong,
                                );
                                strncpy(buf.as_mut_ptr(), ptr, strlen(ptr));
                            }
                        }
                    }
                    _ => {}
                }
                ptr = strtok_r(
                    0 as *mut libc::c_char,
                    b" =\0" as *const u8 as *const libc::c_char,
                    &mut aid,
                );
            }
            if okay as u64 != 0 {
                let mut U_2: URL = url_normalize(base, buf.as_mut_ptr());
                if debug as u64 != 0 {
                    printf(
                        b"4.) Adding: %s\n\0" as *const u8 as *const libc::c_char,
                        url_get_absolute(U_2),
                    );
                }
                __add_url(array, U_2);
            }
        } else if strncasecmp(
            ptr,
            b"script\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            while !ptr.is_null() {
                if strncasecmp(
                    ptr,
                    b"src\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    ptr = strtok_r(
                        0 as *mut libc::c_char,
                        b" \"'\0" as *const u8 as *const libc::c_char,
                        &mut aid,
                    );
                    if !ptr.is_null() {
                        if !(startswith(b"+\0" as *const u8 as *const libc::c_char, ptr)
                            as u64 != 0)
                        {
                            memset(
                                tmp.as_mut_ptr() as *mut libc::c_void,
                                0 as libc::c_int,
                                4096 as libc::c_int as libc::c_ulong,
                            );
                            strncpy(
                                tmp.as_mut_ptr(),
                                ptr,
                                (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                            );
                            let mut U_3: URL = url_normalize(base, tmp.as_mut_ptr());
                            if debug as u64 != 0 {
                                printf(
                                    b"5.) Adding: %s\n\0" as *const u8 as *const libc::c_char,
                                    url_get_absolute(U_3),
                                );
                            }
                            __add_url(array, U_3);
                        }
                    }
                }
                ptr = strtok_r(
                    0 as *mut libc::c_char,
                    b" =\0" as *const u8 as *const libc::c_char,
                    &mut aid,
                );
            }
        } else if strncasecmp(
            ptr,
            b"location.href\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\"'\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            if !ptr.is_null() {
                memset(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    4096 as libc::c_int as libc::c_ulong,
                );
                strncpy(
                    tmp.as_mut_ptr(),
                    ptr,
                    (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                );
            }
        } else if strncasecmp(
            ptr,
            b"frame\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" =\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            while !ptr.is_null() {
                if strncasecmp(
                    ptr,
                    b"src\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    ptr = strtok_r(
                        0 as *mut libc::c_char,
                        b" =\"'\0" as *const u8 as *const libc::c_char,
                        &mut aid,
                    );
                    if !ptr.is_null() {
                        memset(
                            tmp.as_mut_ptr() as *mut libc::c_void,
                            '\0' as i32,
                            4096 as libc::c_int as libc::c_ulong,
                        );
                        strncpy(
                            tmp.as_mut_ptr(),
                            ptr,
                            (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                        );
                    }
                }
                ptr = strtok_r(
                    0 as *mut libc::c_char,
                    b" =\0" as *const u8 as *const libc::c_char,
                    &mut aid,
                );
            }
        } else if strncasecmp(
            ptr,
            b"background\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = strtok_r(
                0 as *mut libc::c_char,
                b" \"'\0" as *const u8 as *const libc::c_char,
                &mut aid,
            );
            if !ptr.is_null()
                && strmatch(
                    b"body\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    top,
                ) as libc::c_uint != 0
            {
                memset(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    4096 as libc::c_int as libc::c_ulong,
                );
                strncpy(
                    tmp.as_mut_ptr(),
                    ptr,
                    (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                );
                let mut U_4: URL = url_normalize(base, tmp.as_mut_ptr());
                if debug as u64 != 0 {
                    printf(
                        b"6.) Adding: %s\n\0" as *const u8 as *const libc::c_char,
                        url_get_absolute(U_4),
                    );
                }
                __add_url(array, U_4);
            }
        }
        ptr = strtok_r(
            0 as *mut libc::c_char,
            b" =\0" as *const u8 as *const libc::c_char,
            &mut aid,
        );
    }
}
unsafe extern "C" fn __strcasestr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut sc: libc::c_char = 0;
    let mut len: size_t = 0;
    let fresh0 = find;
    find = find.offset(1);
    c = *fresh0;
    if c as libc::c_int != 0 as libc::c_int {
        c = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(c as libc::c_uchar as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(c as libc::c_uchar as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        len = strlen(find);
        loop {
            loop {
                let fresh1 = s;
                s = s.offset(1);
                sc = *fresh1;
                if sc as libc::c_int == 0 as libc::c_int {
                    return 0 as *mut libc::c_char;
                }
                if !(({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = sc as libc::c_uchar
                                as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(sc as libc::c_uchar as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(sc as libc::c_uchar as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char as libc::c_int != c as libc::c_int)
                {
                    break;
                }
            }
            if !(strncasecmp(s, find, len) != 0 as libc::c_int) {
                break;
            }
        }
        s = s.offset(-1);
        s;
    }
    return s as *mut libc::c_char;
}
unsafe extern "C" fn __xstrip(
    mut str: *const libc::c_char,
    mut pat: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut tbl: [libc::c_int; 128] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    while *pat as libc::c_int != '\0' as i32 {
        let fresh2 = pat;
        pat = pat.offset(1);
        tbl[*fresh2 as libc::c_int as usize] = 1 as libc::c_int;
    }
    let mut ret: *mut libc::c_char = xmalloc(
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    loop {
        if tbl[*str as libc::c_int as usize] == 0 {
            let fresh3 = i;
            i = i + 1;
            *ret.offset(fresh3 as isize) = *str;
        }
        let fresh4 = str;
        str = str.offset(1);
        if !(*fresh4 as libc::c_int != '\0' as i32) {
            break;
        }
    }
    return xrealloc(ret as *mut libc::c_void, i as size_t) as *mut libc::c_char;
}
