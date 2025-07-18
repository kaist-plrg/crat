use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn json_object_size(object: *const json_t) -> size_t;
    fn json_object_get(object: *const json_t, key: *const libc::c_char) -> *mut json_t;
    fn json_object_iter(object: *mut json_t) -> *mut libc::c_void;
    fn json_object_iter_next(
        object: *mut json_t,
        iter: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn json_object_iter_key(iter: *mut libc::c_void) -> *const libc::c_char;
    fn json_object_iter_value(iter: *mut libc::c_void) -> *mut json_t;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_integer_value(integer: *const json_t) -> json_int_t;
    fn json_real_value(real: *const json_t) -> libc::c_double;
    fn jsonp_object_iter_fullkey(iter: *mut libc::c_void) -> *const object_key_t;
    fn strbuffer_init(strbuff: *mut strbuffer_t) -> libc::c_int;
    fn strbuffer_close(strbuff: *mut strbuffer_t);
    fn strbuffer_value(strbuff: *const strbuffer_t) -> *const libc::c_char;
    fn strbuffer_append_bytes(
        strbuff: *mut strbuffer_t,
        data: *const libc::c_char,
        size: libc::c_int,
    ) -> libc::c_int;
    fn utf8_iterate(
        buffer: *const libc::c_char,
        codepoint: *mut int32_t,
    ) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct strbuffer_t {
    pub value: *mut libc::c_char,
    pub length: libc::c_int,
    pub size: libc::c_int,
}
pub type dump_func = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_t {
    pub json: json_t,
    pub hashtable: hashtable_t,
    pub serial: size_t,
    pub visited: libc::c_int,
}
pub type hashtable_t = hashtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable {
    pub size: size_t,
    pub buckets: *mut hashtable_bucket,
    pub num_buckets: size_t,
    pub list: hashtable_list,
    pub hash_key: key_hash_fn,
    pub cmp_keys: key_cmp_fn,
    pub free_key: free_fn,
    pub free_value: free_fn,
}
pub type free_fn = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type key_cmp_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type key_hash_fn = Option::<unsafe extern "C" fn(*const libc::c_void) -> size_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_list {
    pub prev: *mut hashtable_list,
    pub next: *mut hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable_bucket {
    pub first: *mut hashtable_list,
    pub last: *mut hashtable_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_key_t {
    pub serial: size_t,
    pub key: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_t {
    pub json: json_t,
    pub size: size_t,
    pub entries: size_t,
    pub table: *mut *mut json_t,
    pub visited: libc::c_int,
}
unsafe extern "C" fn dump_to_strbuffer(
    mut buffer: *const libc::c_char,
    mut size: libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    return strbuffer_append_bytes(data as *mut strbuffer_t, buffer, size);
}
unsafe extern "C" fn dump_to_file(
    mut buffer: *const libc::c_char,
    mut size: libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut dest: *mut FILE = data as *mut FILE;
    if fwrite(
        buffer as *const libc::c_void,
        size as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        dest,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
static mut whitespace: [libc::c_char; 33] = unsafe {
    *::std::mem::transmute::<
        &[u8; 33],
        &mut [libc::c_char; 33],
    >(b"                                \0")
};
unsafe extern "C" fn dump_indent(
    mut flags: size_t,
    mut depth: libc::c_int,
    mut space: libc::c_int,
    mut dump: dump_func,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if flags & 0x1f as libc::c_int as libc::c_ulong > 0 as libc::c_int as libc::c_ulong {
        let mut i: libc::c_int = 0;
        let mut ws_count: libc::c_int = (flags & 0x1f as libc::c_int as libc::c_ulong)
            as libc::c_int;
        if dump
            .unwrap()(
            b"\n\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            data,
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < depth {
            if dump.unwrap()(whitespace.as_mut_ptr(), ws_count, data) != 0 {
                return -(1 as libc::c_int);
            }
            i += 1;
            i;
        }
    } else if space != 0 && flags & 0x20 as libc::c_int as libc::c_ulong == 0 {
        return dump
            .unwrap()(b" \0" as *const u8 as *const libc::c_char, 1 as libc::c_int, data)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dump_string(
    mut str: *const libc::c_char,
    mut ascii: libc::c_int,
    mut dump: dump_func,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut codepoint: int32_t = 0 as libc::c_int;
    if dump.unwrap()(b"\"\0" as *const u8 as *const libc::c_char, 1 as libc::c_int, data)
        != 0
    {
        return -(1 as libc::c_int);
    }
    pos = str;
    end = pos;
    loop {
        let mut text: *const libc::c_char = 0 as *const libc::c_char;
        let mut seq: [libc::c_char; 22] = [0; 22];
        let mut length: libc::c_int = 0;
        while *end != 0 {
            end = utf8_iterate(pos, &mut codepoint);
            if end.is_null() {
                return -(1 as libc::c_int);
            }
            if codepoint == '\\' as i32 || codepoint == '"' as i32
                || codepoint < 0x20 as libc::c_int
            {
                break;
            }
            if ascii != 0 && codepoint > 0x7f as libc::c_int {
                break;
            }
            pos = end;
        }
        if pos != str {
            if dump
                .unwrap()(str, pos.offset_from(str) as libc::c_long as libc::c_int, data)
                != 0
            {
                return -(1 as libc::c_int);
            }
        }
        if end == pos {
            break;
        }
        length = 2 as libc::c_int;
        match codepoint {
            92 => {
                text = b"\\\\\0" as *const u8 as *const libc::c_char;
            }
            34 => {
                text = b"\\\"\0" as *const u8 as *const libc::c_char;
            }
            8 => {
                text = b"\\b\0" as *const u8 as *const libc::c_char;
            }
            12 => {
                text = b"\\f\0" as *const u8 as *const libc::c_char;
            }
            10 => {
                text = b"\\n\0" as *const u8 as *const libc::c_char;
            }
            13 => {
                text = b"\\r\0" as *const u8 as *const libc::c_char;
            }
            9 => {
                text = b"\\t\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                if codepoint < 0x10000 as libc::c_int {
                    sprintf(
                        seq.as_mut_ptr(),
                        b"\\u%04x\0" as *const u8 as *const libc::c_char,
                        codepoint,
                    );
                    length = 6 as libc::c_int;
                } else {
                    let mut first: int32_t = 0;
                    let mut last: int32_t = 0;
                    codepoint -= 0x10000 as libc::c_int;
                    first = 0xd800 as libc::c_int
                        | (codepoint & 0xffc00 as libc::c_int) >> 10 as libc::c_int;
                    last = 0xdc00 as libc::c_int | codepoint & 0x3ff as libc::c_int;
                    sprintf(
                        seq.as_mut_ptr(),
                        b"\\u%04x\\u%04x\0" as *const u8 as *const libc::c_char,
                        first,
                        last,
                    );
                    length = 12 as libc::c_int;
                }
                text = seq.as_mut_ptr();
            }
        }
        if dump.unwrap()(text, length, data) != 0 {
            return -(1 as libc::c_int);
        }
        pos = end;
        str = pos;
    }
    return dump
        .unwrap()(b"\"\0" as *const u8 as *const libc::c_char, 1 as libc::c_int, data);
}
unsafe extern "C" fn object_key_compare_keys(
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return strcmp(
        ((**(key1 as *mut *const object_key_t)).key).as_ptr(),
        ((**(key2 as *mut *const object_key_t)).key).as_ptr(),
    );
}
unsafe extern "C" fn object_key_compare_serials(
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return ((**(key1 as *mut *const object_key_t)).serial)
        .wrapping_sub((**(key2 as *mut *const object_key_t)).serial) as libc::c_int;
}
unsafe extern "C" fn do_dump(
    mut json: *const json_t,
    mut flags: size_t,
    mut depth: libc::c_int,
    mut dump: dump_func,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ascii: libc::c_int = if flags & 0x40 as libc::c_int as libc::c_ulong != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    match (*json).type_0 as libc::c_uint {
        7 => {
            return dump
                .unwrap()(
                b"null\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                data,
            );
        }
        5 => {
            return dump
                .unwrap()(
                b"true\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                data,
            );
        }
        6 => {
            return dump
                .unwrap()(
                b"false\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
                data,
            );
        }
        3 => {
            let mut buffer: [libc::c_char; 100] = [0; 100];
            let mut size: libc::c_int = 0;
            size = snprintf(
                buffer.as_mut_ptr(),
                100 as libc::c_int as libc::c_ulong,
                b"%lld\0" as *const u8 as *const libc::c_char,
                json_integer_value(json),
            );
            if size >= 100 as libc::c_int {
                return -(1 as libc::c_int);
            }
            return dump.unwrap()(buffer.as_mut_ptr(), size, data);
        }
        4 => {
            let mut buffer_0: [libc::c_char; 100] = [0; 100];
            let mut size_0: libc::c_int = 0;
            size_0 = snprintf(
                buffer_0.as_mut_ptr(),
                100 as libc::c_int as libc::c_ulong,
                b"%.17g\0" as *const u8 as *const libc::c_char,
                json_real_value(json),
            );
            if size_0 >= 100 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if (strchr(buffer_0.as_mut_ptr(), '.' as i32)).is_null()
                && (strchr(buffer_0.as_mut_ptr(), 'e' as i32)).is_null()
            {
                if size_0 + 2 as libc::c_int >= 100 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                buffer_0[size_0 as usize] = '.' as i32 as libc::c_char;
                buffer_0[(size_0 + 1 as libc::c_int)
                    as usize] = '0' as i32 as libc::c_char;
                size_0 += 2 as libc::c_int;
            }
            return dump.unwrap()(buffer_0.as_mut_ptr(), size_0, data);
        }
        2 => return dump_string(json_string_value(json), ascii, dump, data),
        1 => {
            let mut i: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            let mut array: *mut json_array_t = 0 as *mut json_array_t;
            array = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_array_t;
            if !((*array).visited != 0) {
                (*array).visited = 1 as libc::c_int;
                n = json_array_size(json) as libc::c_int;
                if !(dump
                    .unwrap()(
                    b"[\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    data,
                ) != 0)
                {
                    if n == 0 as libc::c_int {
                        (*array).visited = 0 as libc::c_int;
                        return dump
                            .unwrap()(
                            b"]\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                            data,
                        );
                    }
                    if !(dump_indent(
                        flags,
                        depth + 1 as libc::c_int,
                        0 as libc::c_int,
                        dump,
                        data,
                    ) != 0)
                    {
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < n) {
                                current_block = 11385396242402735691;
                                break;
                            }
                            if do_dump(
                                json_array_get(json, i as size_t),
                                flags,
                                depth + 1 as libc::c_int,
                                dump,
                                data,
                            ) != 0
                            {
                                current_block = 14668945286757690703;
                                break;
                            }
                            if i < n - 1 as libc::c_int {
                                if dump
                                    .unwrap()(
                                    b",\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int,
                                    data,
                                ) != 0
                                    || dump_indent(
                                        flags,
                                        depth + 1 as libc::c_int,
                                        1 as libc::c_int,
                                        dump,
                                        data,
                                    ) != 0
                                {
                                    current_block = 14668945286757690703;
                                    break;
                                }
                            } else if dump_indent(
                                flags,
                                depth,
                                0 as libc::c_int,
                                dump,
                                data,
                            ) != 0
                            {
                                current_block = 14668945286757690703;
                                break;
                            }
                            i += 1;
                            i;
                        }
                        match current_block {
                            14668945286757690703 => {}
                            _ => {
                                (*array).visited = 0 as libc::c_int;
                                return dump
                                    .unwrap()(
                                    b"]\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int,
                                    data,
                                );
                            }
                        }
                    }
                }
            }
            (*array).visited = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
        0 => {
            let mut object: *mut json_object_t = 0 as *mut json_object_t;
            let mut iter: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut separator: *const libc::c_char = 0 as *const libc::c_char;
            let mut separator_length: libc::c_int = 0;
            if flags & 0x20 as libc::c_int as libc::c_ulong != 0 {
                separator = b":\0" as *const u8 as *const libc::c_char;
                separator_length = 1 as libc::c_int;
            } else {
                separator = b": \0" as *const u8 as *const libc::c_char;
                separator_length = 2 as libc::c_int;
            }
            object = (json as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut json_object_t;
            if !((*object).visited != 0) {
                (*object).visited = 1 as libc::c_int;
                iter = json_object_iter(json as *mut json_t);
                if !(dump
                    .unwrap()(
                    b"{\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    data,
                ) != 0)
                {
                    if iter.is_null() {
                        (*object).visited = 0 as libc::c_int;
                        return dump
                            .unwrap()(
                            b"}\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                            data,
                        );
                    }
                    if !(dump_indent(
                        flags,
                        depth + 1 as libc::c_int,
                        0 as libc::c_int,
                        dump,
                        data,
                    ) != 0)
                    {
                        if flags & 0x80 as libc::c_int as libc::c_ulong != 0
                            || flags & 0x100 as libc::c_int as libc::c_ulong != 0
                        {
                            let mut keys: *mut *const object_key_t = 0
                                as *mut *const object_key_t;
                            let mut size_1: size_t = 0;
                            let mut i_0: size_t = 0;
                            let mut cmp_func: Option::<
                                unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                            > = None;
                            size_1 = json_object_size(json);
                            keys = malloc(
                                size_1
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut object_key_t>() as libc::c_ulong,
                                    ),
                            ) as *mut *const object_key_t;
                            if keys.is_null() {
                                current_block = 9198307788677743524;
                            } else {
                                i_0 = 0 as libc::c_int as size_t;
                                while !iter.is_null() {
                                    let ref mut fresh0 = *keys.offset(i_0 as isize);
                                    *fresh0 = jsonp_object_iter_fullkey(iter);
                                    iter = json_object_iter_next(json as *mut json_t, iter);
                                    i_0 = i_0.wrapping_add(1);
                                    i_0;
                                }
                                if i_0 == size_1 {} else {
                                    __assert_fail(
                                        b"i == size\0" as *const u8 as *const libc::c_char,
                                        b"src/jansson/src/dump.c\0" as *const u8
                                            as *const libc::c_char,
                                        327 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 60],
                                            &[libc::c_char; 60],
                                        >(
                                            b"int do_dump(const json_t *, size_t, int, dump_func, void *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                };
                                if flags & 0x80 as libc::c_int as libc::c_ulong != 0 {
                                    cmp_func = Some(
                                        object_key_compare_keys
                                            as unsafe extern "C" fn(
                                                *const libc::c_void,
                                                *const libc::c_void,
                                            ) -> libc::c_int,
                                    );
                                } else {
                                    cmp_func = Some(
                                        object_key_compare_serials
                                            as unsafe extern "C" fn(
                                                *const libc::c_void,
                                                *const libc::c_void,
                                            ) -> libc::c_int,
                                    );
                                }
                                qsort(
                                    keys as *mut libc::c_void,
                                    size_1,
                                    ::std::mem::size_of::<*mut object_key_t>() as libc::c_ulong,
                                    cmp_func,
                                );
                                i_0 = 0 as libc::c_int as size_t;
                                loop {
                                    if !(i_0 < size_1) {
                                        current_block = 1352918242886884122;
                                        break;
                                    }
                                    let mut key: *const libc::c_char = 0 as *const libc::c_char;
                                    let mut value: *mut json_t = 0 as *mut json_t;
                                    key = ((**keys.offset(i_0 as isize)).key).as_ptr();
                                    value = json_object_get(json, key);
                                    if !value.is_null() {} else {
                                        __assert_fail(
                                            b"value\0" as *const u8 as *const libc::c_char,
                                            b"src/jansson/src/dump.c\0" as *const u8
                                                as *const libc::c_char,
                                            343 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 60],
                                                &[libc::c_char; 60],
                                            >(
                                                b"int do_dump(const json_t *, size_t, int, dump_func, void *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                    dump_string(key, ascii, dump, data);
                                    if dump.unwrap()(separator, separator_length, data) != 0
                                        || do_dump(
                                            value,
                                            flags,
                                            depth + 1 as libc::c_int,
                                            dump,
                                            data,
                                        ) != 0
                                    {
                                        free(keys as *mut libc::c_void);
                                        current_block = 9198307788677743524;
                                        break;
                                    } else {
                                        if i_0
                                            < size_1.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        {
                                            if dump
                                                .unwrap()(
                                                b",\0" as *const u8 as *const libc::c_char,
                                                1 as libc::c_int,
                                                data,
                                            ) != 0
                                                || dump_indent(
                                                    flags,
                                                    depth + 1 as libc::c_int,
                                                    1 as libc::c_int,
                                                    dump,
                                                    data,
                                                ) != 0
                                            {
                                                free(keys as *mut libc::c_void);
                                                current_block = 9198307788677743524;
                                                break;
                                            }
                                        } else if dump_indent(
                                            flags,
                                            depth,
                                            0 as libc::c_int,
                                            dump,
                                            data,
                                        ) != 0
                                        {
                                            free(keys as *mut libc::c_void);
                                            current_block = 9198307788677743524;
                                            break;
                                        }
                                        i_0 = i_0.wrapping_add(1);
                                        i_0;
                                    }
                                }
                                match current_block {
                                    9198307788677743524 => {}
                                    _ => {
                                        free(keys as *mut libc::c_void);
                                        current_block = 1852451392920375136;
                                    }
                                }
                            }
                        } else {
                            loop {
                                if iter.is_null() {
                                    current_block = 1852451392920375136;
                                    break;
                                }
                                let mut next: *mut libc::c_void = json_object_iter_next(
                                    json as *mut json_t,
                                    iter,
                                );
                                dump_string(json_object_iter_key(iter), ascii, dump, data);
                                if dump.unwrap()(separator, separator_length, data) != 0
                                    || do_dump(
                                        json_object_iter_value(iter),
                                        flags,
                                        depth + 1 as libc::c_int,
                                        dump,
                                        data,
                                    ) != 0
                                {
                                    current_block = 9198307788677743524;
                                    break;
                                }
                                if !next.is_null() {
                                    if dump
                                        .unwrap()(
                                        b",\0" as *const u8 as *const libc::c_char,
                                        1 as libc::c_int,
                                        data,
                                    ) != 0
                                        || dump_indent(
                                            flags,
                                            depth + 1 as libc::c_int,
                                            1 as libc::c_int,
                                            dump,
                                            data,
                                        ) != 0
                                    {
                                        current_block = 9198307788677743524;
                                        break;
                                    }
                                } else if dump_indent(
                                    flags,
                                    depth,
                                    0 as libc::c_int,
                                    dump,
                                    data,
                                ) != 0
                                {
                                    current_block = 9198307788677743524;
                                    break;
                                }
                                iter = next;
                            }
                        }
                        match current_block {
                            9198307788677743524 => {}
                            _ => {
                                (*object).visited = 0 as libc::c_int;
                                return dump
                                    .unwrap()(
                                    b"}\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int,
                                    data,
                                );
                            }
                        }
                    }
                }
            }
            (*object).visited = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
        _ => return -(1 as libc::c_int),
    };
}
pub unsafe extern "C" fn json_dumps(
    mut json: *const json_t,
    mut flags: size_t,
) -> *mut libc::c_char {
    let mut strbuff: strbuffer_t = strbuffer_t {
        value: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
    };
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
        && !(!json.is_null()
            && (*json).type_0 as libc::c_uint
                == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return 0 as *mut libc::c_char;
    }
    if strbuffer_init(&mut strbuff) != 0 {
        return 0 as *mut libc::c_char;
    }
    if do_dump(
        json,
        flags,
        0 as libc::c_int,
        Some(
            dump_to_strbuffer
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut strbuff as *mut strbuffer_t as *mut libc::c_void,
    ) != 0
    {
        strbuffer_close(&mut strbuff);
        return 0 as *mut libc::c_char;
    }
    result = strdup(strbuffer_value(&mut strbuff));
    strbuffer_close(&mut strbuff);
    return result;
}
pub unsafe extern "C" fn json_dumpf(
    mut json: *const json_t,
    mut output: *mut FILE,
    mut flags: size_t,
) -> libc::c_int {
    if !(!json.is_null()
        && (*json).type_0 as libc::c_uint == JSON_ARRAY as libc::c_int as libc::c_uint)
        && !(!json.is_null()
            && (*json).type_0 as libc::c_uint
                == JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    return do_dump(
        json,
        flags,
        0 as libc::c_int,
        Some(
            dump_to_file
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        output as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn json_dump_file(
    mut json: *const json_t,
    mut path: *const libc::c_char,
    mut flags: size_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut output: *mut FILE = fopen(path, b"w\0" as *const u8 as *const libc::c_char);
    if output.is_null() {
        return -(1 as libc::c_int);
    }
    result = json_dumpf(json, output, flags);
    fclose(output);
    return result;
}
