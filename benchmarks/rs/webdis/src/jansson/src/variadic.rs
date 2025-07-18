use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn json_number_value(json: *const json_t) -> libc::c_double;
    fn json_integer_value(integer: *const json_t) -> json_int_t;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_object() -> *mut json_t;
    fn json_array() -> *mut json_t;
    fn json_string(value: *const libc::c_char) -> *mut json_t;
    fn json_integer(value: json_int_t) -> *mut json_t;
    fn json_real(value: libc::c_double) -> *mut json_t;
    fn json_true() -> *mut json_t;
    fn json_false() -> *mut json_t;
    fn json_null() -> *mut json_t;
    fn json_delete(json: *mut json_t);
    fn json_object_size(object: *const json_t) -> size_t;
    fn json_object_get(object: *const json_t, key: *const libc::c_char) -> *mut json_t;
    fn json_object_set_new(
        object: *mut json_t,
        key: *const libc::c_char,
        value: *mut json_t,
    ) -> libc::c_int;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_array_append_new(array: *mut json_t, value: *mut json_t) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn jsonp_error_init(error: *mut json_error_t, source: *const libc::c_char);
    fn jsonp_error_set(
        error: *mut json_error_t,
        line: libc::c_int,
        column: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type json_type = libc::c_uint;
pub const JSON_NULL: json_type = 7;
pub const JSON_FALSE: json_type = 6;
pub const JSON_TRUE: json_type = 5;
pub const JSON_REAL: json_type = 4;
pub const JSON_INTEGER: json_type = 3;
pub const JSON_STRING: json_type = 2;
pub const JSON_ARRAY: json_type = 1;
pub const JSON_OBJECT: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_int_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub text: [libc::c_char; 160],
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub source: [libc::c_char; 80],
}
#[inline]
unsafe extern "C" fn json_incref(mut json: *mut json_t) -> *mut json_t {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t {
        (*json).refcount = ((*json).refcount).wrapping_add(1);
        (*json).refcount;
    }
    return json;
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t
        && {
            (*json).refcount = ((*json).refcount).wrapping_sub(1);
            (*json).refcount == 0 as libc::c_int as libc::c_ulong
        }
    {
        json_delete(json);
    }
}
pub unsafe extern "C" fn json_pack(
    mut error: *mut json_error_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut json_t {
    let mut current_block: u64;
    let mut fmt_length: libc::c_int = strlen(fmt) as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    let mut depth: libc::c_int = 0 as libc::c_int;
    let mut stack: *mut *mut json_t = 0 as *mut *mut json_t;
    let mut free_count: libc::c_int = 0 as libc::c_int;
    let mut free_list: *mut *mut json_t = 0 as *mut *mut json_t;
    let mut cur: *mut json_t = 0 as *mut json_t;
    let mut root: *mut json_t = 0 as *mut json_t;
    let mut obj: *mut json_t = 0 as *mut json_t;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 1 as libc::c_int;
    stack = calloc(
        fmt_length as libc::c_ulong,
        ::std::mem::size_of::<*mut json_t>() as libc::c_ulong,
    ) as *mut *mut json_t;
    free_list = calloc(
        fmt_length as libc::c_ulong,
        ::std::mem::size_of::<*mut json_t>() as libc::c_ulong,
    ) as *mut *mut json_t;
    jsonp_error_init(error, b"\0" as *const u8 as *const libc::c_char);
    if !(stack.is_null() || free_list.is_null()) {
        ap = args.clone();
        loop {
            if !(*fmt != 0) {
                current_block = 11796148217846552555;
                break;
            }
            match *fmt as libc::c_int {
                10 => {
                    line += 1;
                    line;
                    current_block = 3229571381435211107;
                }
                44 => {
                    if root.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unexpected COMMA precedes root element!\0" as *const u8
                                as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if cur.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unexpected COMMA outside a list or object!\0" as *const u8
                                as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if !key.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Expected KEY, got COMMA!\0" as *const u8
                                as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    }
                    current_block = 3229571381435211107;
                }
                58 => {
                    if key.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Got key/value separator without a key preceding it!\0"
                                as *const u8 as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if !(!cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint)
                    {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Got a key/value separator (':') outside an object!\0"
                                as *const u8 as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    }
                    current_block = 3229571381435211107;
                }
                93 | 125 => {
                    if !key.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"OBJECT or ARRAY ended with an incomplete key/value pair!\0"
                                as *const u8 as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if depth <= 0 as libc::c_int {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Too many close-brackets '%c'\0" as *const u8
                                as *const libc::c_char,
                            *fmt as libc::c_int,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if *fmt as libc::c_int == ']' as i32
                        && !(!cur.is_null()
                            && (*cur).type_0 as libc::c_uint
                                == JSON_ARRAY as libc::c_int as libc::c_uint)
                    {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Stray close-array ']' character\0" as *const u8
                                as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if *fmt as libc::c_int == '}' as i32
                        && !(!cur.is_null()
                            && (*cur).type_0 as libc::c_uint
                                == JSON_OBJECT as libc::c_int as libc::c_uint)
                    {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Stray close-object '}' character\0" as *const u8
                                as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else {
                        depth -= 1;
                        cur = *stack.offset(depth as isize);
                    }
                    current_block = 3229571381435211107;
                }
                91 => {
                    obj = json_array();
                    current_block = 12505187733534816596;
                }
                123 => {
                    obj = json_object();
                    current_block = 12505187733534816596;
                }
                115 => {
                    s = ap.arg::<*mut libc::c_char>();
                    if s.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Refusing to handle a NULL string\0" as *const u8
                                as *const libc::c_char,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint
                        && key.is_null()
                    {
                        key = s;
                        current_block = 3229571381435211107;
                    } else {
                        obj = json_string(s);
                        current_block = 12505187733534816596;
                    }
                }
                110 => {
                    obj = json_null();
                    current_block = 12505187733534816596;
                }
                98 => {
                    obj = if ap.arg::<libc::c_int>() != 0 {
                        json_true()
                    } else {
                        json_false()
                    };
                    current_block = 12505187733534816596;
                }
                105 => {
                    obj = json_integer(ap.arg::<libc::c_int>() as json_int_t);
                    current_block = 12505187733534816596;
                }
                102 => {
                    obj = json_real(ap.arg::<libc::c_double>());
                    current_block = 12505187733534816596;
                }
                79 => {
                    obj = ap.arg::<*mut json_t>();
                    json_incref(obj);
                    current_block = 12505187733534816596;
                }
                111 => {
                    obj = ap.arg::<*mut json_t>();
                    current_block = 12505187733534816596;
                }
                32 | _ => {
                    current_block = 3229571381435211107;
                }
            }
            match current_block {
                12505187733534816596 => {
                    let fresh0 = free_count;
                    free_count = free_count + 1;
                    let ref mut fresh1 = *free_list.offset(fresh0 as isize);
                    *fresh1 = obj;
                    if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint
                    {
                        if key.is_null() {
                            jsonp_error_set(
                                error,
                                line,
                                -(1 as libc::c_int),
                                b"Expected key, got identifier '%c'!\0" as *const u8
                                    as *const libc::c_char,
                                *fmt as libc::c_int,
                            );
                            root = 0 as *mut json_t;
                            current_block = 6785566911446336637;
                            break;
                        } else {
                            json_object_set_new(cur, key, obj);
                            key = 0 as *mut libc::c_char;
                        }
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_ARRAY as libc::c_int as libc::c_uint
                    {
                        json_array_append_new(cur, obj);
                    } else if root.is_null() {
                        printf(b"Rooting\n\0" as *const u8 as *const libc::c_char);
                        root = obj;
                    } else {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Can't figure out where to attach '%c' object!\0"
                                as *const u8 as *const libc::c_char,
                            *fmt as libc::c_int,
                        );
                        root = 0 as *mut json_t;
                        current_block = 6785566911446336637;
                        break;
                    }
                    if !obj.is_null()
                        && (*obj).type_0 as libc::c_uint
                            == JSON_ARRAY as libc::c_int as libc::c_uint
                        || !obj.is_null()
                            && (*obj).type_0 as libc::c_uint
                                == JSON_OBJECT as libc::c_int as libc::c_uint
                    {
                        let fresh2 = depth;
                        depth = depth + 1;
                        let ref mut fresh3 = *stack.offset(fresh2 as isize);
                        *fresh3 = cur;
                        cur = obj;
                    }
                }
                _ => {}
            }
            fmt = fmt.offset(1);
            fmt;
        }
        match current_block {
            6785566911446336637 => {}
            _ => {
                if depth != 0 as libc::c_int {
                    jsonp_error_set(
                        error,
                        line,
                        -(1 as libc::c_int),
                        b"Missing object or array close-brackets in format string\0"
                            as *const u8 as *const libc::c_char,
                    );
                    root = 0 as *mut json_t;
                } else {
                    free_count = 0 as libc::c_int;
                }
            }
        }
    }
    while free_count != 0 {
        free_count -= 1;
        json_decref(*free_list.offset(free_count as isize));
    }
    if !free_list.is_null() {
        free(free_list as *mut libc::c_void);
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return root;
}
pub unsafe extern "C" fn json_unpack(
    mut root: *mut json_t,
    mut error: *mut json_error_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut unvisited: libc::c_int = 0;
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 1 as libc::c_int;
    let mut depth: libc::c_int = 0 as libc::c_int;
    let mut stack: *mut *mut json_t = 0 as *mut *mut json_t;
    let mut array_index: libc::c_int = 0 as libc::c_int;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: *mut json_t = 0 as *mut json_t;
    let mut obj: *mut json_t = 0 as *mut json_t;
    let mut fmt_length: libc::c_int = strlen(fmt) as libc::c_int;
    jsonp_error_init(error, b"\0" as *const u8 as *const libc::c_char);
    stack = calloc(
        fmt_length as libc::c_ulong,
        ::std::mem::size_of::<*mut json_t>() as libc::c_ulong,
    ) as *mut *mut json_t;
    if stack.is_null() {
        jsonp_error_set(
            error,
            line,
            -(1 as libc::c_int),
            b"Out of memory!\0" as *const u8 as *const libc::c_char,
        );
        rv = -(1 as libc::c_int);
    } else {
        unvisited = 0 as libc::c_int;
        ap = args.clone();
        loop {
            if !(*fmt != 0) {
                current_block = 8937240710477387595;
                break;
            }
            match *fmt as libc::c_int {
                32 => {
                    current_block = 7416055328783156979;
                }
                10 => {
                    line += 1;
                    line;
                    current_block = 7416055328783156979;
                }
                44 => {
                    if cur.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unexpected COMMA outside a list or object!\0" as *const u8
                                as *const libc::c_char,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else if !key.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Expected KEY, got COMMA!\0" as *const u8
                                as *const libc::c_char,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    }
                    current_block = 7416055328783156979;
                }
                58 => {
                    if !(!cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint)
                        || key.is_null()
                    {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unexpected ':'\0" as *const u8 as *const libc::c_char,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else {
                        current_block = 7416055328783156979;
                    }
                }
                91 | 123 => {
                    if cur.is_null() {
                        obj = root;
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint
                    {
                        if key.is_null() {
                            jsonp_error_set(
                                error,
                                line,
                                -(1 as libc::c_int),
                                b"Objects can't be keys\0" as *const u8
                                    as *const libc::c_char,
                            );
                            rv = -(1 as libc::c_int);
                            current_block = 1428307939028130064;
                            break;
                        } else {
                            obj = json_object_get(cur, key);
                            unvisited -= 1;
                            unvisited;
                            key = 0 as *mut libc::c_char;
                        }
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_ARRAY as libc::c_int as libc::c_uint
                    {
                        obj = json_array_get(cur, array_index as size_t);
                        unvisited -= 1;
                        unvisited;
                        array_index += 1;
                        array_index;
                    } else {
                        if 0 as libc::c_int != 0 {} else {
                            __assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"src/jansson/src/variadic.c\0" as *const u8
                                    as *const libc::c_char,
                                370 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 61],
                                    &[libc::c_char; 61],
                                >(
                                    b"int json_unpack(json_t *, json_error_t *, const char *, ...)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                    }
                    if *fmt as libc::c_int == '{' as i32
                        && !(!obj.is_null()
                            && (*obj).type_0 as libc::c_uint
                                == JSON_OBJECT as libc::c_int as libc::c_uint)
                    {
                        rv = -(2 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else if *fmt as libc::c_int == '[' as i32
                        && !(!obj.is_null()
                            && (*obj).type_0 as libc::c_uint
                                == JSON_ARRAY as libc::c_int as libc::c_uint)
                    {
                        rv = -(2 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else {
                        unvisited = (unvisited as libc::c_ulong)
                            .wrapping_add(
                                if !obj.is_null()
                                    && (*obj).type_0 as libc::c_uint
                                        == JSON_OBJECT as libc::c_int as libc::c_uint
                                {
                                    json_object_size(obj)
                                } else {
                                    json_array_size(obj)
                                },
                            ) as libc::c_int as libc::c_int;
                        let fresh4 = depth;
                        depth = depth + 1;
                        let ref mut fresh5 = *stack.offset(fresh4 as isize);
                        *fresh5 = cur;
                        cur = obj;
                        key = 0 as *mut libc::c_char;
                    }
                    current_block = 7416055328783156979;
                }
                93 | 125 => {
                    if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_ARRAY as libc::c_int as libc::c_uint
                        && *fmt as libc::c_int != ']' as i32
                    {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Missing ']'\0" as *const u8 as *const libc::c_char,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint
                        && *fmt as libc::c_int != '}' as i32
                    {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Missing '}'\0" as *const u8 as *const libc::c_char,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else if !key.is_null() {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unexpected '%c'\0" as *const u8 as *const libc::c_char,
                            *fmt as libc::c_int,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else if depth <= 0 as libc::c_int {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unexpected '%c'\0" as *const u8 as *const libc::c_char,
                            *fmt as libc::c_int,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    } else {
                        depth -= 1;
                        cur = *stack.offset(depth as isize);
                    }
                    current_block = 7416055328783156979;
                }
                115 => {
                    if key.is_null()
                        && (!cur.is_null()
                            && (*cur).type_0 as libc::c_uint
                                == JSON_OBJECT as libc::c_int as libc::c_uint)
                    {
                        key = ap.arg::<*mut libc::c_char>();
                        current_block = 7416055328783156979;
                    } else {
                        current_block = 10238508159951227465;
                    }
                }
                105 => {
                    current_block = 10238508159951227465;
                }
                102 => {
                    current_block = 6163499581338803661;
                }
                79 => {
                    current_block = 17583837684161480671;
                }
                111 => {
                    current_block = 9293908815909468982;
                }
                98 => {
                    current_block = 56673797834558889;
                }
                110 => {
                    current_block = 16545672169641455831;
                }
                _ => {
                    current_block = 7416055328783156979;
                }
            }
            match current_block {
                10238508159951227465 => {
                    current_block = 6163499581338803661;
                }
                _ => {}
            }
            match current_block {
                6163499581338803661 => {
                    current_block = 17583837684161480671;
                }
                _ => {}
            }
            match current_block {
                17583837684161480671 => {
                    current_block = 9293908815909468982;
                }
                _ => {}
            }
            match current_block {
                9293908815909468982 => {
                    current_block = 56673797834558889;
                }
                _ => {}
            }
            match current_block {
                56673797834558889 => {
                    current_block = 16545672169641455831;
                }
                _ => {}
            }
            match current_block {
                16545672169641455831 => {
                    if cur.is_null() {
                        obj = root;
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_OBJECT as libc::c_int as libc::c_uint
                    {
                        if key.is_null() {
                            jsonp_error_set(
                                error,
                                line,
                                -(1 as libc::c_int),
                                b"Only strings may be used as keys!\0" as *const u8
                                    as *const libc::c_char,
                            );
                            rv = -(1 as libc::c_int);
                            current_block = 1428307939028130064;
                            break;
                        } else {
                            obj = json_object_get(cur, key);
                            unvisited -= 1;
                            unvisited;
                            key = 0 as *mut libc::c_char;
                        }
                    } else if !cur.is_null()
                        && (*cur).type_0 as libc::c_uint
                            == JSON_ARRAY as libc::c_int as libc::c_uint
                    {
                        obj = json_array_get(cur, array_index as size_t);
                        unvisited -= 1;
                        unvisited;
                        array_index += 1;
                        array_index;
                    } else {
                        jsonp_error_set(
                            error,
                            line,
                            -(1 as libc::c_int),
                            b"Unsure how to retrieve JSON object '%c'\0" as *const u8
                                as *const libc::c_char,
                            *fmt as libc::c_int,
                        );
                        rv = -(1 as libc::c_int);
                        current_block = 1428307939028130064;
                        break;
                    }
                    match *fmt as libc::c_int {
                        115 => {
                            current_block = 4589371822259783267;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                        105 => {
                            current_block = 10876639498027152528;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                        98 => {
                            current_block = 8230173412473719446;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                        102 => {
                            current_block = 8560119967913591254;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                        79 => {
                            current_block = 14152827358935435035;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                        111 => {
                            current_block = 12205017671613270583;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                        110 => {}
                        _ => {
                            current_block = 8462258133321559388;
                            match current_block {
                                8230173412473719446 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_TRUE as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_FALSE as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a boolean.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = (!obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_TRUE as libc::c_int as libc::c_uint) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                10876639498027152528 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't an integer.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap
                                            .arg::<
                                                *mut libc::c_int,
                                            >() = json_integer_value(obj) as libc::c_int;
                                    }
                                    current_block = 7416055328783156979;
                                }
                                4589371822259783267 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_STRING as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a string.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        let ref mut fresh6 = *ap.arg::<*mut *const libc::c_char>();
                                        *fresh6 = json_string_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                8462258133321559388 => {
                                    jsonp_error_set(
                                        error,
                                        line,
                                        -(1 as libc::c_int),
                                        b"Unknown format character '%c'\0" as *const u8
                                            as *const libc::c_char,
                                        *fmt as libc::c_int,
                                    );
                                    rv = -(1 as libc::c_int);
                                    current_block = 1428307939028130064;
                                    break;
                                }
                                8560119967913591254 => {
                                    if !(!obj.is_null()
                                        && (*obj).type_0 as libc::c_uint
                                            == JSON_INTEGER as libc::c_int as libc::c_uint
                                        || !obj.is_null()
                                            && (*obj).type_0 as libc::c_uint
                                                == JSON_REAL as libc::c_int as libc::c_uint)
                                    {
                                        jsonp_error_set(
                                            error,
                                            line,
                                            -(1 as libc::c_int),
                                            b"Type mismatch! Object wasn't a real.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rv = -(2 as libc::c_int);
                                        current_block = 1428307939028130064;
                                        break;
                                    } else {
                                        *ap.arg::<*mut libc::c_double>() = json_number_value(obj);
                                    }
                                    current_block = 7416055328783156979;
                                }
                                14152827358935435035 => {
                                    json_incref(obj);
                                    current_block = 12205017671613270583;
                                }
                                _ => {}
                            }
                            match current_block {
                                7416055328783156979 => {}
                                _ => {
                                    let ref mut fresh7 = *ap.arg::<*mut *mut json_t>();
                                    *fresh7 = obj;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            fmt = fmt.offset(1);
            fmt;
        }
        match current_block {
            1428307939028130064 => {}
            _ => {
                rv = unvisited;
            }
        }
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return rv;
}
