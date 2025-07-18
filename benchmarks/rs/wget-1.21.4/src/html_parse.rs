use ::libc;
extern "C" {
    pub type hash_table;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_pair {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub value_raw_beginning: *const libc::c_char,
    pub value_raw_size: libc::c_int,
    pub name_pool_index: libc::c_int,
    pub value_pool_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taginfo {
    pub name: *mut libc::c_char,
    pub end_tag_p: libc::c_int,
    pub nattrs: libc::c_int,
    pub attrs: *mut attr_pair,
    pub start_position: *const libc::c_char,
    pub end_position: *const libc::c_char,
    pub contents_begin: *const libc::c_char,
    pub contents_end: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagstack_item {
    pub tagname_begin: *const libc::c_char,
    pub tagname_end: *const libc::c_char,
    pub contents_begin: *const libc::c_char,
    pub prev: *mut tagstack_item,
    pub next: *mut tagstack_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub contents: *mut libc::c_char,
    pub size: libc::c_int,
    pub tail: libc::c_int,
    pub resized: bool,
    pub orig_contents: *mut libc::c_char,
    pub orig_size: libc::c_int,
}
pub const AP_DOWNCASE: C2RustUnnamed_0 = 1;
pub const AP_TRIM_BLANKS: C2RustUnnamed_0 = 4;
pub const AP_DECODE_ENTITIES: C2RustUnnamed_0 = 2;
pub const AC_S_BACKOUT: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const AC_S_QUOTE2: C2RustUnnamed = 12;
pub const AC_S_IN_QUOTE: C2RustUnnamed = 11;
pub const AC_S_QUOTE1: C2RustUnnamed = 10;
pub const AC_S_DASH4: C2RustUnnamed = 9;
pub const AC_S_DASH3: C2RustUnnamed = 8;
pub const AC_S_COMMENT: C2RustUnnamed = 7;
pub const AC_S_DASH2: C2RustUnnamed = 6;
pub const AC_S_DASH1: C2RustUnnamed = 5;
pub const AC_S_DCLNAME: C2RustUnnamed = 4;
pub const AC_S_DEFAULT: C2RustUnnamed = 3;
pub const AC_S_BANG: C2RustUnnamed = 2;
pub const AC_S_DONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn _unhex(mut c: libc::c_uchar) -> libc::c_uchar {
    return (if c as libc::c_int <= '9' as i32 {
        c as libc::c_int - '0' as i32
    } else if c as libc::c_int <= 'F' as i32 {
        c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    } else {
        c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }) as libc::c_uchar;
}
unsafe extern "C" fn tagstack_push(
    mut head: *mut *mut tagstack_item,
    mut tail: *mut *mut tagstack_item,
) -> *mut tagstack_item {
    let mut ts: *mut tagstack_item = xmalloc(
        ::std::mem::size_of::<tagstack_item>() as libc::c_ulong,
    ) as *mut tagstack_item;
    if (*head).is_null() {
        *tail = ts;
        *head = *tail;
        (*ts).next = 0 as *mut tagstack_item;
        (*ts).prev = (*ts).next;
    } else {
        (**tail).next = ts;
        (*ts).prev = *tail;
        *tail = ts;
        (*ts).next = 0 as *mut tagstack_item;
    }
    return ts;
}
unsafe extern "C" fn tagstack_pop(
    mut head: *mut *mut tagstack_item,
    mut tail: *mut *mut tagstack_item,
    mut ts: *mut tagstack_item,
) {
    if (*head).is_null() {
        return;
    }
    if ts == *tail {
        if ts == *head {
            rpl_free(ts as *mut libc::c_void);
            ts = 0 as *mut tagstack_item;
            *tail = 0 as *mut tagstack_item;
            *head = *tail;
        } else {
            (*(*ts).prev).next = 0 as *mut tagstack_item;
            *tail = (*ts).prev;
            rpl_free(ts as *mut libc::c_void);
            ts = 0 as *mut tagstack_item;
        }
    } else {
        if ts == *head {
            *head = 0 as *mut tagstack_item;
        }
        *tail = (*ts).prev;
        if !((*ts).prev).is_null() {
            (*(*ts).prev).next = 0 as *mut tagstack_item;
        }
        while !ts.is_null() {
            let mut p: *mut tagstack_item = (*ts).next;
            rpl_free(ts as *mut libc::c_void);
            ts = 0 as *mut tagstack_item;
            ts = p;
        }
    };
}
unsafe extern "C" fn tagstack_find(
    mut tail: *mut tagstack_item,
    mut tagname_begin: *const libc::c_char,
    mut tagname_end: *const libc::c_char,
) -> *mut tagstack_item {
    let mut len: libc::c_int = tagname_end.offset_from(tagname_begin) as libc::c_long
        as libc::c_int;
    while !tail.is_null() {
        if len as libc::c_long
            == ((*tail).tagname_end).offset_from((*tail).tagname_begin) as libc::c_long
        {
            if 0 as libc::c_int
                == strncasecmp(
                    (*tail).tagname_begin,
                    tagname_begin,
                    len as libc::c_ulong,
                )
            {
                return tail;
            }
        }
        tail = (*tail).prev;
    }
    return 0 as *mut tagstack_item;
}
unsafe extern "C" fn decode_entity(
    mut ptr: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = *ptr;
    let mut value: libc::c_int = -(1 as libc::c_int);
    p = p.offset(1);
    if p == end {
        return -(1 as libc::c_int);
    }
    let fresh0 = p;
    p = p.offset(1);
    match *fresh0 as libc::c_int {
        35 => {
            let mut digits: libc::c_int = 0 as libc::c_int;
            value = 0 as libc::c_int;
            if *p as libc::c_int == 'x' as i32 {
                p = p.offset(1);
                p;
                while value < 256 as libc::c_int && p < end
                    && c_isxdigit(*p as libc::c_int) as libc::c_int != 0
                {
                    value = (value << 4 as libc::c_int)
                        + _unhex(*p as libc::c_uchar) as libc::c_int;
                    p = p.offset(1);
                    p;
                    digits += 1;
                    digits;
                }
            } else {
                while value < 256 as libc::c_int && p < end
                    && c_isdigit(*p as libc::c_int) as libc::c_int != 0
                {
                    value = value * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
                    p = p.offset(1);
                    p;
                    digits += 1;
                    digits;
                }
            }
            if digits == 0 {
                return -(1 as libc::c_int);
            }
            if value == 0 || value & !(0x7f as libc::c_int) != 0 {
                return -(1 as libc::c_int);
            }
            p = p.offset(0 as libc::c_int as isize);
            *ptr = if p < end && *p as libc::c_int == ';' as i32 {
                p = p.offset(1);
                p
            } else {
                p
            };
            return value;
        }
        103 => {
            if (p.offset(1 as libc::c_int as isize) == end
                || p.offset(1 as libc::c_int as isize) < end
                    && !c_isalnum(*p.offset(1 as libc::c_int as isize) as libc::c_int))
                && *p.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
            {
                value = '>' as i32;
                p = p.offset(1 as libc::c_int as isize);
                *ptr = if p < end && *p as libc::c_int == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                };
            }
        }
        108 => {
            if (p.offset(1 as libc::c_int as isize) == end
                || p.offset(1 as libc::c_int as isize) < end
                    && !c_isalnum(*p.offset(1 as libc::c_int as isize) as libc::c_int))
                && *p.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
            {
                value = '<' as i32;
                p = p.offset(1 as libc::c_int as isize);
                *ptr = if p < end && *p as libc::c_int == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                };
            }
        }
        97 => {
            if (p.offset(2 as libc::c_int as isize) == end
                || p.offset(2 as libc::c_int as isize) < end
                    && !c_isalnum(*p.offset(2 as libc::c_int as isize) as libc::c_int))
                && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'm' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'p' as i32
            {
                value = '&' as i32;
                p = p.offset(2 as libc::c_int as isize);
                *ptr = if p < end && *p as libc::c_int == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                };
            } else if (p.offset(3 as libc::c_int as isize) == end
                || p.offset(3 as libc::c_int as isize) < end
                    && !c_isalnum(*p.offset(3 as libc::c_int as isize) as libc::c_int))
                && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'p' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == 's' as i32
            {
                value = '\'' as i32;
                p = p.offset(3 as libc::c_int as isize);
                *ptr = if p < end && *p as libc::c_int == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                };
            }
        }
        113 => {
            if (p.offset(3 as libc::c_int as isize) == end
                || p.offset(3 as libc::c_int as isize) < end
                    && !c_isalnum(*p.offset(3 as libc::c_int as isize) as libc::c_int))
                && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'u' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32
            {
                value = '"' as i32;
                p = p.offset(3 as libc::c_int as isize);
                *ptr = if p < end && *p as libc::c_int == ';' as i32 {
                    p = p.offset(1);
                    p
                } else {
                    p
                };
            }
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn convert_and_copy(
    mut pool: *mut pool,
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
    mut flags: libc::c_int,
) {
    let mut old_tail: libc::c_int = (*pool).tail;
    if flags & AP_TRIM_BLANKS as libc::c_int != 0 {
        while beg < end && c_isspace(*beg as libc::c_int) as libc::c_int != 0 {
            beg = beg.offset(1);
            beg;
        }
        while end > beg
            && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_int != 0
        {
            end = end.offset(-1);
            end;
        }
    }
    if flags & AP_DECODE_ENTITIES as libc::c_int != 0 {
        let mut from: *const libc::c_char = beg;
        let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut squash_newlines: bool = flags & AP_TRIM_BLANKS as libc::c_int != 0;
        let mut ga_needed_size: libc::c_long = (*pool).tail as libc::c_long
            + end.offset_from(beg) as libc::c_long;
        let mut ga_newsize: libc::c_long = (*pool).size as libc::c_long;
        while ga_newsize < ga_needed_size {
            ga_newsize <<= 1 as libc::c_int;
        }
        if ga_newsize != (*pool).size as libc::c_long {
            if (*pool).resized {
                (*pool)
                    .contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
            } else {
                let mut ga_new: *mut libc::c_void = xmalloc(
                    (ga_newsize as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                memcpy(
                    ga_new,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                (*pool).contents = ga_new as *mut libc::c_char;
                (*pool).resized = 1 as libc::c_int != 0;
            }
            (*pool).size = ga_newsize as libc::c_int;
        }
        to = ((*pool).contents).offset((*pool).tail as isize);
        while from < end {
            if *from as libc::c_int == '&' as i32 {
                let mut entity: libc::c_int = decode_entity(&mut from, end);
                if entity != -(1 as libc::c_int) {
                    let fresh1 = to;
                    to = to.offset(1);
                    *fresh1 = entity as libc::c_char;
                } else {
                    let fresh2 = from;
                    from = from.offset(1);
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = *fresh2;
                }
            } else if (*from as libc::c_int == '\n' as i32
                || *from as libc::c_int == '\r' as i32)
                && squash_newlines as libc::c_int != 0
            {
                from = from.offset(1);
                from;
            } else {
                let fresh4 = from;
                from = from.offset(1);
                let fresh5 = to;
                to = to.offset(1);
                *fresh5 = *fresh4;
            }
        }
        (*pool).tail = to.offset_from((*pool).contents) as libc::c_long as libc::c_int;
        let mut PAC_char: libc::c_char = '\0' as i32 as libc::c_char;
        let mut ga_needed_size_0: libc::c_long = ((*pool).tail + 1 as libc::c_int)
            as libc::c_long;
        let mut ga_newsize_0: libc::c_long = (*pool).size as libc::c_long;
        while ga_newsize_0 < ga_needed_size_0 {
            ga_newsize_0 <<= 1 as libc::c_int;
        }
        if ga_newsize_0 != (*pool).size as libc::c_long {
            if (*pool).resized {
                (*pool)
                    .contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize_0 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
            } else {
                let mut ga_new_0: *mut libc::c_void = xmalloc(
                    (ga_newsize_0 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                memcpy(
                    ga_new_0,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                (*pool).contents = ga_new_0 as *mut libc::c_char;
                (*pool).resized = 1 as libc::c_int != 0;
            }
            (*pool).size = ga_newsize_0 as libc::c_int;
        }
        let fresh6 = (*pool).tail;
        (*pool).tail = (*pool).tail + 1;
        *((*pool).contents).offset(fresh6 as isize) = PAC_char;
    } else {
        let mut PA_beg: *const libc::c_char = beg;
        let mut PA_size: libc::c_int = end.offset_from(PA_beg) as libc::c_long
            as libc::c_int;
        let mut ga_needed_size_1: libc::c_long = ((*pool).tail + PA_size)
            as libc::c_long;
        let mut ga_newsize_1: libc::c_long = (*pool).size as libc::c_long;
        while ga_newsize_1 < ga_needed_size_1 {
            ga_newsize_1 <<= 1 as libc::c_int;
        }
        if ga_newsize_1 != (*pool).size as libc::c_long {
            if (*pool).resized {
                (*pool)
                    .contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize_1 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
            } else {
                let mut ga_new_1: *mut libc::c_void = xmalloc(
                    (ga_newsize_1 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                memcpy(
                    ga_new_1,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                (*pool).contents = ga_new_1 as *mut libc::c_char;
                (*pool).resized = 1 as libc::c_int != 0;
            }
            (*pool).size = ga_newsize_1 as libc::c_int;
        }
        memcpy(
            ((*pool).contents).offset((*pool).tail as isize) as *mut libc::c_void,
            PA_beg as *const libc::c_void,
            PA_size as libc::c_ulong,
        );
        (*pool).tail += PA_size;
        let mut PAC_char_0: libc::c_char = '\0' as i32 as libc::c_char;
        let mut ga_needed_size_2: libc::c_long = ((*pool).tail + 1 as libc::c_int)
            as libc::c_long;
        let mut ga_newsize_2: libc::c_long = (*pool).size as libc::c_long;
        while ga_newsize_2 < ga_needed_size_2 {
            ga_newsize_2 <<= 1 as libc::c_int;
        }
        if ga_newsize_2 != (*pool).size as libc::c_long {
            if (*pool).resized {
                (*pool)
                    .contents = xrealloc(
                    (*pool).contents as *mut libc::c_void,
                    (ga_newsize_2 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
            } else {
                let mut ga_new_2: *mut libc::c_void = xmalloc(
                    (ga_newsize_2 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                memcpy(
                    ga_new_2,
                    (*pool).contents as *const libc::c_void,
                    ((*pool).size as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                (*pool).contents = ga_new_2 as *mut libc::c_char;
                (*pool).resized = 1 as libc::c_int != 0;
            }
            (*pool).size = ga_newsize_2 as libc::c_int;
        }
        let fresh7 = (*pool).tail;
        (*pool).tail = (*pool).tail + 1;
        *((*pool).contents).offset(fresh7 as isize) = PAC_char_0;
    }
    if flags & AP_DOWNCASE as libc::c_int != 0 {
        let mut p: *mut libc::c_char = ((*pool).contents).offset(old_tail as isize);
        while *p != 0 {
            *p = c_tolower(*p as libc::c_int) as libc::c_char;
            p = p.offset(1);
            p;
        }
    }
}
unsafe extern "C" fn advance_declaration(
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = beg;
    let mut quote_char: libc::c_char = '\0' as i32 as libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut state: C2RustUnnamed = AC_S_BANG;
    if beg == end {
        return beg;
    }
    let fresh8 = p;
    p = p.offset(1);
    ch = *fresh8;
    while state as libc::c_uint != AC_S_DONE as libc::c_int as libc::c_uint
        && state as libc::c_uint != AC_S_BACKOUT as libc::c_int as libc::c_uint
    {
        if p == end {
            state = AC_S_BACKOUT;
        }
        match state as libc::c_uint {
            2 => {
                if ch as libc::c_int == '!' as i32 {
                    let fresh9 = p;
                    p = p.offset(1);
                    ch = *fresh9;
                    state = AC_S_DEFAULT;
                } else {
                    state = AC_S_BACKOUT;
                }
            }
            3 => {
                match ch as libc::c_int {
                    45 => {
                        state = AC_S_DASH1;
                    }
                    32 | 9 | 13 | 10 => {
                        let fresh10 = p;
                        p = p.offset(1);
                        ch = *fresh10;
                    }
                    60 | 62 => {
                        state = AC_S_DONE;
                    }
                    39 | 34 => {
                        state = AC_S_QUOTE1;
                    }
                    _ => {
                        if ch as libc::c_int > 32 as libc::c_int
                            && (ch as libc::c_int) < 127 as libc::c_int
                            && ch as libc::c_int != '=' as i32
                            && ch as libc::c_int != '<' as i32
                            && ch as libc::c_int != '>' as i32
                            && ch as libc::c_int != '/' as i32
                        {
                            state = AC_S_DCLNAME;
                        } else {
                            state = AC_S_BACKOUT;
                        }
                    }
                }
            }
            4 => {
                if ch as libc::c_int == '-' as i32 {
                    state = AC_S_DASH1;
                } else if ch as libc::c_int > 32 as libc::c_int
                    && (ch as libc::c_int) < 127 as libc::c_int
                    && ch as libc::c_int != '=' as i32 && ch as libc::c_int != '<' as i32
                    && ch as libc::c_int != '>' as i32 && ch as libc::c_int != '/' as i32
                {
                    let fresh11 = p;
                    p = p.offset(1);
                    ch = *fresh11;
                } else {
                    state = AC_S_DEFAULT;
                }
            }
            10 => {
                quote_char = ch;
                let fresh12 = p;
                p = p.offset(1);
                ch = *fresh12;
                state = AC_S_IN_QUOTE;
            }
            11 => {
                if ch as libc::c_int == quote_char as libc::c_int {
                    state = AC_S_QUOTE2;
                } else {
                    let fresh13 = p;
                    p = p.offset(1);
                    ch = *fresh13;
                }
            }
            12 => {
                let fresh14 = p;
                p = p.offset(1);
                ch = *fresh14;
                state = AC_S_DEFAULT;
            }
            5 => {
                let fresh15 = p;
                p = p.offset(1);
                ch = *fresh15;
                state = AC_S_DASH2;
            }
            6 => {
                match ch as libc::c_int {
                    45 => {
                        let fresh16 = p;
                        p = p.offset(1);
                        ch = *fresh16;
                        state = AC_S_COMMENT;
                    }
                    _ => {
                        state = AC_S_BACKOUT;
                    }
                }
            }
            7 => {
                match ch as libc::c_int {
                    45 => {
                        state = AC_S_DASH3;
                    }
                    _ => {
                        let fresh17 = p;
                        p = p.offset(1);
                        ch = *fresh17;
                    }
                }
            }
            8 => {
                let fresh18 = p;
                p = p.offset(1);
                ch = *fresh18;
                state = AC_S_DASH4;
            }
            9 => {
                match ch as libc::c_int {
                    45 => {
                        let fresh19 = p;
                        p = p.offset(1);
                        ch = *fresh19;
                        state = AC_S_DEFAULT;
                    }
                    _ => {
                        state = AC_S_COMMENT;
                    }
                }
            }
            0 | 1 | _ => {}
        }
    }
    if state as libc::c_uint == AC_S_BACKOUT as libc::c_int as libc::c_uint {
        return beg.offset(1 as libc::c_int as isize);
    }
    return p;
}
unsafe extern "C" fn find_comment_end(
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = beg.offset(-(1 as libc::c_int as isize));
    loop {
        p = p.offset(3 as libc::c_int as isize);
        if !(p < end) {
            break;
        }
        match *p.offset(0 as libc::c_int as isize) as libc::c_int {
            62 => {
                if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '-' as i32
                    && *p.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == '-' as i32
                {
                    return p.offset(1 as libc::c_int as isize);
                }
            }
            45 => {
                loop {
                    if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '-' as i32
                    {
                        's_46: {
                            loop {
                                p = p.offset(1);
                                if p == end {
                                    return 0 as *const libc::c_char;
                                }
                                match *p.offset(0 as libc::c_int as isize) as libc::c_int {
                                    62 => return p.offset(1 as libc::c_int as isize),
                                    45 => {}
                                    _ => {
                                        break 's_46;
                                    }
                                }
                            }
                        }
                        break;
                    } else {
                        p = p.offset(2 as libc::c_int as isize);
                        if p >= end {
                            return 0 as *const libc::c_char;
                        }
                        match *p.offset(0 as libc::c_int as isize) as libc::c_int {
                            62 => {}
                            45 => {
                                continue;
                            }
                            _ => {
                                break;
                            }
                        }
                        if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '-' as i32
                        {
                            return p.offset(1 as libc::c_int as isize);
                        }
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn name_allowed(
    mut ht: *const hash_table,
    mut b: *const libc::c_char,
    mut e: *const libc::c_char,
) -> bool {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = e.offset_from(b) as libc::c_long as size_t;
    let mut ret: bool = false;
    if ht.is_null() {
        return 1 as libc::c_int != 0;
    }
    if len < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
        copy = buf.as_mut_ptr();
    } else {
        copy = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    memcpy(copy as *mut libc::c_void, b as *const libc::c_void, len);
    *copy.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    ret = !(hash_table_get(ht, copy as *const libc::c_void)).is_null();
    if copy != buf.as_mut_ptr() {
        rpl_free(copy as *mut libc::c_void);
        copy = 0 as *mut libc::c_char;
    }
    return ret;
}
pub unsafe extern "C" fn map_html_tags(
    mut text: *const libc::c_char,
    mut size: libc::c_int,
    mut mapfun: Option::<unsafe extern "C" fn(*mut taginfo, *mut libc::c_void) -> ()>,
    mut maparg: *mut libc::c_void,
    mut flags: libc::c_int,
    mut allowed_tags: *const hash_table,
    mut allowed_attributes: *const hash_table,
) {
    let mut current_block: u64;
    let mut pool_initial_storage: [libc::c_char; 256] = [0; 256];
    let mut pool: pool = pool {
        contents: 0 as *mut libc::c_char,
        size: 0,
        tail: 0,
        resized: false,
        orig_contents: 0 as *mut libc::c_char,
        orig_size: 0,
    };
    let mut p: *const libc::c_char = text;
    let mut end: *const libc::c_char = text.offset(size as isize);
    let mut attr_pair_initial_storage: [attr_pair; 8] = [attr_pair {
        name: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
        value_raw_beginning: 0 as *const libc::c_char,
        value_raw_size: 0,
        name_pool_index: 0,
        value_pool_index: 0,
    }; 8];
    let mut attr_pair_size: libc::c_int = (::std::mem::size_of::<[attr_pair; 8]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<attr_pair>() as libc::c_ulong)
        as libc::c_int;
    let mut attr_pair_resized: bool = 0 as libc::c_int != 0;
    let mut pairs: *mut attr_pair = attr_pair_initial_storage.as_mut_ptr();
    let mut head: *mut tagstack_item = 0 as *mut tagstack_item;
    let mut tail: *mut tagstack_item = 0 as *mut tagstack_item;
    if size == 0 {
        return;
    }
    let mut P: *mut pool = &mut pool;
    (*P).contents = pool_initial_storage.as_mut_ptr();
    (*P)
        .size = (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    (*P).tail = 0 as libc::c_int;
    (*P).resized = 0 as libc::c_int != 0;
    (*P).orig_contents = (*P).contents;
    (*P).orig_size = (*P).size;
    let mut nattrs: libc::c_int = 0;
    let mut end_tag: libc::c_int = 0;
    let mut tag_name_begin: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag_name_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag_start_position: *const libc::c_char = 0 as *const libc::c_char;
    let mut uninteresting_tag: bool = false;
    '_look_for_tag: loop {
        pool.tail = 0 as libc::c_int;
        nattrs = 0 as libc::c_int;
        end_tag = 0 as libc::c_int;
        p = memchr(
            p as *const libc::c_void,
            '<' as i32,
            end.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if p.is_null() {
            break;
        }
        tag_start_position = p;
        p = p.offset(1);
        p;
        if p >= end {
            break;
        }
        if *p as libc::c_int == '!' as i32 {
            if flags & 1 as libc::c_int == 0 && p.offset(3 as libc::c_int as isize) < end
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                let mut comment_end: *const libc::c_char = find_comment_end(
                    p.offset(3 as libc::c_int as isize),
                    end,
                );
                if !comment_end.is_null() {
                    p = comment_end;
                }
            } else {
                p = advance_declaration(p, end);
            }
            if p == end {
                break;
            }
        } else {
            if *p as libc::c_int == '/' as i32 {
                end_tag = 1 as libc::c_int;
                p = p.offset(1);
                p;
                if p >= end {
                    break;
                }
            }
            tag_name_begin = p;
            while *p as libc::c_int > 32 as libc::c_int
                && (*p as libc::c_int) < 127 as libc::c_int
                && *p as libc::c_int != '=' as i32 && *p as libc::c_int != '<' as i32
                && *p as libc::c_int != '>' as i32 && *p as libc::c_int != '/' as i32
            {
                p = p.offset(1);
                p;
                if p >= end {
                    break '_look_for_tag;
                }
            }
            if p == tag_name_begin {
                continue;
            }
            tag_name_end = p;
            while c_isspace(*p as libc::c_int) {
                p = p.offset(1);
                p;
                if p >= end {
                    break '_look_for_tag;
                }
            }
            if end_tag == 0 {
                let mut ts: *mut tagstack_item = tagstack_push(&mut head, &mut tail);
                if !ts.is_null() {
                    (*ts).tagname_begin = tag_name_begin;
                    (*ts).tagname_end = tag_name_end;
                    (*ts).contents_begin = 0 as *const libc::c_char;
                }
            }
            if !(end_tag != 0 && *p as libc::c_int != '>' as i32
                && *p as libc::c_int != '<' as i32)
            {
                if !name_allowed(allowed_tags, tag_name_begin, tag_name_end) {
                    uninteresting_tag = 1 as libc::c_int != 0;
                } else {
                    uninteresting_tag = 0 as libc::c_int != 0;
                    convert_and_copy(
                        &mut pool,
                        tag_name_begin,
                        tag_name_end,
                        AP_DOWNCASE as libc::c_int,
                    );
                }
                loop {
                    let mut attr_name_begin: *const libc::c_char = 0
                        as *const libc::c_char;
                    let mut attr_name_end: *const libc::c_char = 0
                        as *const libc::c_char;
                    let mut attr_value_begin: *const libc::c_char = 0
                        as *const libc::c_char;
                    let mut attr_value_end: *const libc::c_char = 0
                        as *const libc::c_char;
                    let mut attr_raw_value_begin: *const libc::c_char = 0
                        as *const libc::c_char;
                    let mut attr_raw_value_end: *const libc::c_char = 0
                        as *const libc::c_char;
                    let mut operation: libc::c_int = AP_DOWNCASE as libc::c_int;
                    while c_isspace(*p as libc::c_int) {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                    }
                    if *p as libc::c_int == '/' as i32 {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                        while c_isspace(*p as libc::c_int) {
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                        }
                        if *p as libc::c_int != '<' as i32
                            && *p as libc::c_int != '>' as i32
                        {
                            current_block = 5848396766481592867;
                            break;
                        }
                    }
                    if *p as libc::c_int == '<' as i32 || *p as libc::c_int == '>' as i32
                    {
                        current_block = 9270770154621591809;
                        break;
                    }
                    attr_name_begin = p;
                    while *p as libc::c_int > 32 as libc::c_int
                        && (*p as libc::c_int) < 127 as libc::c_int
                        && *p as libc::c_int != '=' as i32
                        && *p as libc::c_int != '<' as i32
                        && *p as libc::c_int != '>' as i32
                        && *p as libc::c_int != '/' as i32
                    {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                    }
                    attr_name_end = p;
                    if attr_name_begin == attr_name_end {
                        current_block = 5848396766481592867;
                        break;
                    }
                    while c_isspace(*p as libc::c_int) {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                    }
                    if *p as libc::c_int > 32 as libc::c_int
                        && (*p as libc::c_int) < 127 as libc::c_int
                        && *p as libc::c_int != '=' as i32
                        && *p as libc::c_int != '<' as i32
                        && *p as libc::c_int != '>' as i32
                        && *p as libc::c_int != '/' as i32
                        || *p as libc::c_int == '/' as i32
                        || *p as libc::c_int == '<' as i32
                        || *p as libc::c_int == '>' as i32
                    {
                        attr_value_begin = attr_name_begin;
                        attr_raw_value_begin = attr_value_begin;
                        attr_value_end = attr_name_end;
                        attr_raw_value_end = attr_value_end;
                    } else if *p as libc::c_int == '=' as i32 {
                        p = p.offset(1);
                        p;
                        if p >= end {
                            break '_look_for_tag;
                        }
                        while c_isspace(*p as libc::c_int) {
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                        }
                        if *p as libc::c_int == '"' as i32
                            || *p as libc::c_int == '\'' as i32
                        {
                            let mut newline_seen: bool = 0 as libc::c_int != 0;
                            let mut quote_char: libc::c_char = *p;
                            attr_raw_value_begin = p;
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                            attr_value_begin = p;
                            while *p as libc::c_int != quote_char as libc::c_int {
                                if !newline_seen && *p as libc::c_int == '\n' as i32 {
                                    p = attr_value_begin;
                                    newline_seen = 1 as libc::c_int != 0;
                                } else {
                                    if newline_seen as libc::c_int != 0
                                        && (*p as libc::c_int == '<' as i32
                                            || *p as libc::c_int == '>' as i32)
                                    {
                                        break;
                                    }
                                    p = p.offset(1);
                                    p;
                                    if p >= end {
                                        break '_look_for_tag;
                                    }
                                }
                            }
                            attr_value_end = p;
                            if !(*p as libc::c_int == quote_char as libc::c_int) {
                                continue '_look_for_tag;
                            }
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break '_look_for_tag;
                            }
                            attr_raw_value_end = p;
                            operation = AP_DECODE_ENTITIES as libc::c_int;
                            if flags & 2 as libc::c_int != 0 {
                                operation |= AP_TRIM_BLANKS as libc::c_int;
                            }
                        } else {
                            attr_value_begin = p;
                            while !c_isspace(*p as libc::c_int)
                                && *p as libc::c_int != '<' as i32
                                && *p as libc::c_int != '>' as i32
                            {
                                p = p.offset(1);
                                p;
                                if p >= end {
                                    break '_look_for_tag;
                                }
                            }
                            attr_value_end = p;
                            if attr_value_begin == attr_value_end {
                                current_block = 5848396766481592867;
                                break;
                            } else {
                                attr_raw_value_begin = attr_value_begin;
                                attr_raw_value_end = attr_value_end;
                                operation = AP_DECODE_ENTITIES as libc::c_int;
                            }
                        }
                    } else {
                        current_block = 5848396766481592867;
                        break;
                    }
                    if uninteresting_tag {
                        continue;
                    }
                    if !name_allowed(
                        allowed_attributes,
                        attr_name_begin,
                        attr_name_end,
                    ) {
                        continue;
                    }
                    let mut ga_needed_size: libc::c_long = (nattrs + 1 as libc::c_int)
                        as libc::c_long;
                    let mut ga_newsize: libc::c_long = attr_pair_size as libc::c_long;
                    while ga_newsize < ga_needed_size {
                        ga_newsize <<= 1 as libc::c_int;
                    }
                    if ga_newsize != attr_pair_size as libc::c_long {
                        if attr_pair_resized {
                            pairs = xrealloc(
                                pairs as *mut libc::c_void,
                                (ga_newsize as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<attr_pair>() as libc::c_ulong,
                                    ),
                            ) as *mut attr_pair;
                        } else {
                            let mut ga_new: *mut libc::c_void = xmalloc(
                                (ga_newsize as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<attr_pair>() as libc::c_ulong,
                                    ),
                            );
                            memcpy(
                                ga_new,
                                pairs as *const libc::c_void,
                                (attr_pair_size as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<attr_pair>() as libc::c_ulong,
                                    ),
                            );
                            pairs = ga_new as *mut attr_pair;
                            attr_pair_resized = 1 as libc::c_int != 0;
                        }
                        attr_pair_size = ga_newsize as libc::c_int;
                    }
                    (*pairs.offset(nattrs as isize)).name_pool_index = pool.tail;
                    convert_and_copy(
                        &mut pool,
                        attr_name_begin,
                        attr_name_end,
                        AP_DOWNCASE as libc::c_int,
                    );
                    (*pairs.offset(nattrs as isize)).value_pool_index = pool.tail;
                    convert_and_copy(
                        &mut pool,
                        attr_value_begin,
                        attr_value_end,
                        operation,
                    );
                    let ref mut fresh20 = (*pairs.offset(nattrs as isize))
                        .value_raw_beginning;
                    *fresh20 = attr_raw_value_begin;
                    (*pairs.offset(nattrs as isize))
                        .value_raw_size = attr_raw_value_end
                        .offset_from(attr_raw_value_begin) as libc::c_long
                        as libc::c_int;
                    nattrs += 1;
                    nattrs;
                }
                match current_block {
                    5848396766481592867 => {}
                    _ => {
                        if end_tag == 0 && !tail.is_null()
                            && (*tail).tagname_begin == tag_name_begin
                        {
                            (*tail).contents_begin = p.offset(1 as libc::c_int as isize);
                        }
                        if uninteresting_tag {
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break;
                            } else {
                                continue;
                            }
                        } else {
                            let mut i: libc::c_int = 0;
                            let mut taginfo: taginfo = taginfo {
                                name: 0 as *mut libc::c_char,
                                end_tag_p: 0,
                                nattrs: 0,
                                attrs: 0 as *mut attr_pair,
                                start_position: 0 as *const libc::c_char,
                                end_position: 0 as *const libc::c_char,
                                contents_begin: 0 as *const libc::c_char,
                                contents_end: 0 as *const libc::c_char,
                            };
                            let mut ts_0: *mut tagstack_item = 0 as *mut tagstack_item;
                            taginfo.name = pool.contents;
                            taginfo.end_tag_p = end_tag;
                            taginfo.nattrs = nattrs;
                            i = 0 as libc::c_int;
                            while i < nattrs {
                                let ref mut fresh21 = (*pairs.offset(i as isize)).name;
                                *fresh21 = (pool.contents)
                                    .offset(
                                        (*pairs.offset(i as isize)).name_pool_index as isize,
                                    );
                                let ref mut fresh22 = (*pairs.offset(i as isize)).value;
                                *fresh22 = (pool.contents)
                                    .offset(
                                        (*pairs.offset(i as isize)).value_pool_index as isize,
                                    );
                                i += 1;
                                i;
                            }
                            taginfo.attrs = pairs;
                            taginfo.start_position = tag_start_position;
                            taginfo.end_position = p.offset(1 as libc::c_int as isize);
                            taginfo.contents_begin = 0 as *const libc::c_char;
                            taginfo.contents_end = 0 as *const libc::c_char;
                            if end_tag != 0 {
                                ts_0 = tagstack_find(tail, tag_name_begin, tag_name_end);
                                if !ts_0.is_null() {
                                    if !((*ts_0).contents_begin).is_null() {
                                        taginfo.contents_begin = (*ts_0).contents_begin;
                                        taginfo.contents_end = tag_start_position;
                                    }
                                    tagstack_pop(&mut head, &mut tail, ts_0);
                                }
                            }
                            mapfun.unwrap()(&mut taginfo, maparg);
                            if !(*p as libc::c_int != '<' as i32) {
                                continue;
                            }
                            p = p.offset(1);
                            p;
                            if p >= end {
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }
            }
            p = tag_start_position.offset(1 as libc::c_int as isize);
        }
    }
    let mut P_0: *mut pool = &mut pool;
    if (*P_0).resized {
        rpl_free((*P_0).contents as *mut libc::c_void);
        (*P_0).contents = 0 as *mut libc::c_char;
    }
    (*P_0).contents = (*P_0).orig_contents;
    (*P_0).size = (*P_0).orig_size;
    (*P_0).tail = 0 as libc::c_int;
    (*P_0).resized = 0 as libc::c_int != 0;
    if attr_pair_resized {
        rpl_free(pairs as *mut libc::c_void);
        pairs = 0 as *mut attr_pair;
    }
    tagstack_pop(&mut head, &mut tail, head);
}
