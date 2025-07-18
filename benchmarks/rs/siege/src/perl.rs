use ::libc;
extern "C" {
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn xfree(ptr: *mut libc::c_void);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub unsafe extern "C" fn chomp(mut str: *mut libc::c_char) -> *mut libc::c_char {
    if *str as libc::c_int != 0
        && *str
            .offset(
                (strlen(str)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '\n' as i32
    {
        *str
            .offset(
                (strlen(str)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    return str;
}
pub unsafe extern "C" fn rtrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    ptr = str;
    while *ptr as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    len = strlen(str) as libc::c_int;
    ptr = str.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while ptr >= str
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(-1);
        ptr;
    }
    *ptr.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn ltrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    ptr = str;
    while *ptr as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    len = strlen(ptr) as libc::c_int;
    memmove(
        str as *mut libc::c_void,
        ptr as *const libc::c_void,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    return str;
}
pub unsafe extern "C" fn trim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    ptr = rtrim(str);
    str = ltrim(ptr);
    return str;
}
pub unsafe extern "C" fn valid(mut s: *const libc::c_char) -> libc::c_int {
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        flag = (flag != 0 || *s.offset(i as isize) as libc::c_int == '\0' as i32)
            as libc::c_int;
        i += 1;
        i;
    }
    if flag != 0 { return 1 as libc::c_int } else { return 0 as libc::c_int };
}
pub unsafe extern "C" fn empty(mut s: *const libc::c_char) -> BOOLEAN {
    if s.is_null() {
        return boolean_true;
    }
    if strlen(s) < 1 as libc::c_int as libc::c_ulong {
        return boolean_true;
    }
    while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    return (*s as libc::c_int == '\0' as i32) as libc::c_int as BOOLEAN;
}
pub unsafe extern "C" fn word_count(
    mut pattern: libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut in_word_flag: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = s;
    while *ptr != 0 {
        if *ptr as libc::c_int != pattern as libc::c_int {
            if in_word_flag == 0 as libc::c_int {
                count += 1;
                count;
            }
            in_word_flag = 1 as libc::c_int;
        } else {
            in_word_flag = 0 as libc::c_int;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    return count;
}
pub unsafe extern "C" fn split(
    mut pattern: libc::c_char,
    mut s: *mut libc::c_char,
    mut n_words: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut words: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut str0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    *n_words = word_count(pattern, s);
    if *n_words == 0 as libc::c_int {
        return 0 as *mut *mut libc::c_char;
    }
    words = xmalloc(
        (*n_words as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if words.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    str0 = s;
    i = 0 as libc::c_int;
    while *str0 != 0 {
        let mut len: size_t = 0;
        str1 = strchr(str0, pattern as libc::c_int);
        if !str1.is_null() {
            len = str1.offset_from(str0) as libc::c_long as size_t;
        } else {
            len = strlen(str0);
        }
        if len == 0 as libc::c_int as libc::c_ulong {
            i -= 1;
            i;
        } else {
            let ref mut fresh0 = *words.offset(i as isize);
            *fresh0 = xmalloc(4096 as libc::c_int as size_t) as *mut libc::c_char;
            memset(
                *words.offset(i as isize) as *mut libc::c_void,
                '\0' as i32,
                4096 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                *words.offset(i as isize) as *mut libc::c_void,
                str0 as *const libc::c_void,
                4096 as libc::c_int as libc::c_ulong,
            );
            *(*words.offset(i as isize))
                .offset(len as isize) = '\0' as i32 as libc::c_char;
        }
        if str1.is_null() {
            break;
        }
        str1 = str1.offset(1);
        str0 = str1;
        i += 1;
        i;
    }
    return words;
}
pub unsafe extern "C" fn split_free(
    mut split_0: *mut *mut libc::c_char,
    mut length: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < length {
        if !(*split_0.offset(x as isize)).is_null() {
            let mut tmp: *mut libc::c_char = *split_0.offset(x as isize);
            xfree(tmp as *mut libc::c_void);
        }
        x += 1;
        x;
    }
    free(split_0 as *mut libc::c_void);
}
