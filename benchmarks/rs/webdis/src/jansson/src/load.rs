use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn json_object() -> *mut json_t;
    fn json_array() -> *mut json_t;
    fn json_string_nocheck(value: *const libc::c_char) -> *mut json_t;
    fn json_integer(value: json_int_t) -> *mut json_t;
    fn json_real(value: libc::c_double) -> *mut json_t;
    fn json_true() -> *mut json_t;
    fn json_false() -> *mut json_t;
    fn json_null() -> *mut json_t;
    fn json_delete(json: *mut json_t);
    fn json_object_set_new_nocheck(
        object: *mut json_t,
        key: *const libc::c_char,
        value: *mut json_t,
    ) -> libc::c_int;
    fn json_array_append_new(array: *mut json_t, value: *mut json_t) -> libc::c_int;
    fn jsonp_error_init(error: *mut json_error_t, source: *const libc::c_char);
    fn jsonp_error_set(
        error: *mut json_error_t,
        line: libc::c_int,
        column: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn strbuffer_init(strbuff: *mut strbuffer_t) -> libc::c_int;
    fn strbuffer_close(strbuff: *mut strbuffer_t);
    fn strbuffer_clear(strbuff: *mut strbuffer_t);
    fn strbuffer_value(strbuff: *const strbuffer_t) -> *const libc::c_char;
    fn strbuffer_append_byte(
        strbuff: *mut strbuffer_t,
        byte: libc::c_char,
    ) -> libc::c_int;
    fn strbuffer_pop(strbuff: *mut strbuffer_t) -> libc::c_char;
    fn utf8_check_full(
        buffer: *const libc::c_char,
        size: libc::c_int,
        codepoint: *mut int32_t,
    ) -> libc::c_int;
    fn utf8_check_first(byte: libc::c_char) -> libc::c_int;
    fn utf8_encode(
        codepoint: libc::c_int,
        buffer: *mut libc::c_char,
        size: *mut libc::c_int,
    ) -> libc::c_int;
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
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lex_t {
    pub stream: stream_t,
    pub saved_text: strbuffer_t,
    pub token: libc::c_int,
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: *mut libc::c_char,
    pub integer: json_int_t,
    pub real: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strbuffer_t {
    pub value: *mut libc::c_char,
    pub length: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_t {
    pub get: get_func,
    pub eof: eof_func,
    pub data: *mut libc::c_void,
    pub stream_pos: libc::c_int,
    pub buffer: [libc::c_char; 5],
    pub buffer_pos: libc::c_int,
}
pub type eof_func = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
pub type get_func = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_data_t {
    pub data: *const libc::c_char,
    pub pos: libc::c_int,
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
#[inline]
unsafe extern "C" fn json_object_set_nocheck(
    mut object: *mut json_t,
    mut key: *const libc::c_char,
    mut value: *mut json_t,
) -> libc::c_int {
    return json_object_set_new_nocheck(object, key, json_incref(value));
}
#[inline]
unsafe extern "C" fn json_array_append(
    mut array: *mut json_t,
    mut value: *mut json_t,
) -> libc::c_int {
    return json_array_append_new(array, json_incref(value));
}
unsafe extern "C" fn error_set(
    mut error: *mut json_error_t,
    mut lex: *const lex_t,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut msg_text: [libc::c_char; 160] = [0; 160];
    let mut line: libc::c_int = -(1 as libc::c_int);
    let mut col: libc::c_int = -(1 as libc::c_int);
    let mut result: *const libc::c_char = msg_text.as_mut_ptr();
    if error.is_null() {
        return;
    }
    ap = args.clone();
    vsnprintf(
        msg_text.as_mut_ptr(),
        160 as libc::c_int as libc::c_ulong,
        msg,
        ap.as_va_list(),
    );
    if !lex.is_null() {
        let mut saved_text: *const libc::c_char = strbuffer_value(&(*lex).saved_text);
        let mut msg_with_context: [libc::c_char; 160] = [0; 160];
        line = (*lex).line;
        if !saved_text.is_null()
            && *saved_text.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            if (*lex).saved_text.length <= 20 as libc::c_int {
                snprintf(
                    msg_with_context.as_mut_ptr(),
                    160 as libc::c_int as libc::c_ulong,
                    b"%s near '%s'\0" as *const u8 as *const libc::c_char,
                    msg_text.as_mut_ptr(),
                    saved_text,
                );
                result = msg_with_context.as_mut_ptr();
            }
        } else {
            snprintf(
                msg_with_context.as_mut_ptr(),
                160 as libc::c_int as libc::c_ulong,
                b"%s near end of file\0" as *const u8 as *const libc::c_char,
                msg_text.as_mut_ptr(),
            );
            result = msg_with_context.as_mut_ptr();
        }
    }
    jsonp_error_set(
        error,
        line,
        col,
        b"%s\0" as *const u8 as *const libc::c_char,
        result,
    );
}
unsafe extern "C" fn stream_init(
    mut stream: *mut stream_t,
    mut get: get_func,
    mut eof: eof_func,
    mut data: *mut libc::c_void,
) {
    (*stream).get = get;
    (*stream).eof = eof;
    (*stream).data = data;
    (*stream).stream_pos = 0 as libc::c_int;
    (*stream).buffer[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    (*stream).buffer_pos = 0 as libc::c_int;
}
unsafe extern "C" fn stream_get(
    mut stream: *mut stream_t,
    mut error: *mut json_error_t,
) -> libc::c_char {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    if (*stream).buffer[(*stream).buffer_pos as usize] == 0 {
        (*stream)
            .buffer[0 as libc::c_int
            as usize] = ((*stream).get).unwrap()((*stream).data) as libc::c_char;
        (*stream).buffer_pos = 0 as libc::c_int;
        c = (*stream).buffer[0 as libc::c_int as usize];
        if c as libc::c_uchar as libc::c_int >= 0x80 as libc::c_int
            && c as libc::c_int != -(1 as libc::c_int) as libc::c_char as libc::c_int
        {
            let mut i: libc::c_int = 0;
            let mut count: libc::c_int = 0;
            count = utf8_check_first(c);
            if count == 0 {
                current_block = 6175971421882522271;
            } else {
                if count >= 2 as libc::c_int {} else {
                    __assert_fail(
                        b"count >= 2\0" as *const u8 as *const libc::c_char,
                        b"src/jansson/src/load.c\0" as *const u8 as *const libc::c_char,
                        149 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"char stream_get(stream_t *, json_error_t *)\0"))
                            .as_ptr(),
                    );
                };
                i = 1 as libc::c_int;
                while i < count {
                    (*stream)
                        .buffer[i
                        as usize] = ((*stream).get).unwrap()((*stream).data)
                        as libc::c_char;
                    i += 1;
                    i;
                }
                if utf8_check_full(
                    ((*stream).buffer).as_mut_ptr(),
                    count,
                    0 as *mut int32_t,
                ) == 0
                {
                    current_block = 6175971421882522271;
                } else {
                    (*stream).stream_pos += count;
                    (*stream).buffer[count as usize] = '\0' as i32 as libc::c_char;
                    current_block = 10048703153582371463;
                }
            }
            match current_block {
                10048703153582371463 => {}
                _ => {
                    error_set(
                        error,
                        0 as *const lex_t,
                        b"unable to decode byte 0x%x at position %d\0" as *const u8
                            as *const libc::c_char,
                        c as libc::c_uchar as libc::c_int,
                        (*stream).stream_pos,
                    );
                    (*stream)
                        .buffer[0 as libc::c_int
                        as usize] = -(1 as libc::c_int) as libc::c_char;
                    (*stream)
                        .buffer[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    (*stream).buffer_pos = 1 as libc::c_int;
                    return -(1 as libc::c_int) as libc::c_char;
                }
            }
        } else {
            (*stream).buffer[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            (*stream).stream_pos += 1;
            (*stream).stream_pos;
        }
    }
    let fresh0 = (*stream).buffer_pos;
    (*stream).buffer_pos = (*stream).buffer_pos + 1;
    return (*stream).buffer[fresh0 as usize];
}
unsafe extern "C" fn stream_unget(mut stream: *mut stream_t, mut c: libc::c_char) {
    if (*stream).buffer_pos > 0 as libc::c_int {} else {
        __assert_fail(
            b"stream->buffer_pos > 0\0" as *const u8 as *const libc::c_char,
            b"src/jansson/src/load.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void stream_unget(stream_t *, char)\0"))
                .as_ptr(),
        );
    };
    (*stream).buffer_pos -= 1;
    (*stream).buffer_pos;
    if (*stream).buffer[(*stream).buffer_pos as usize] as libc::c_int == c as libc::c_int
    {} else {
        __assert_fail(
            b"stream->buffer[stream->buffer_pos] == c\0" as *const u8
                as *const libc::c_char,
            b"src/jansson/src/load.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void stream_unget(stream_t *, char)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn lex_get(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> libc::c_int {
    return stream_get(&mut (*lex).stream, error) as libc::c_int;
}
unsafe extern "C" fn lex_eof(mut lex: *mut lex_t) -> libc::c_int {
    return ((*lex).stream.eof).unwrap()((*lex).stream.data);
}
unsafe extern "C" fn lex_save(mut lex: *mut lex_t, mut c: libc::c_char) {
    strbuffer_append_byte(&mut (*lex).saved_text, c);
}
unsafe extern "C" fn lex_get_save(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> libc::c_int {
    let mut c: libc::c_char = stream_get(&mut (*lex).stream, error);
    lex_save(lex, c);
    return c as libc::c_int;
}
unsafe extern "C" fn lex_unget_unsave(mut lex: *mut lex_t, mut c: libc::c_char) {
    let mut d: libc::c_char = 0;
    stream_unget(&mut (*lex).stream, c);
    d = strbuffer_pop(&mut (*lex).saved_text);
    if c as libc::c_int == d as libc::c_int {} else {
        __assert_fail(
            b"c == d\0" as *const u8 as *const libc::c_char,
            b"src/jansson/src/load.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void lex_unget_unsave(lex_t *, char)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn lex_save_cached(mut lex: *mut lex_t) {
    while (*lex).stream.buffer[(*lex).stream.buffer_pos as usize] as libc::c_int
        != '\0' as i32
    {
        lex_save(lex, (*lex).stream.buffer[(*lex).stream.buffer_pos as usize]);
        (*lex).stream.buffer_pos += 1;
        (*lex).stream.buffer_pos;
    }
}
unsafe extern "C" fn decode_unicode_escape(mut str: *const libc::c_char) -> int32_t {
    let mut i: libc::c_int = 0;
    let mut value: int32_t = 0 as libc::c_int;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == 'u' as i32 {} else {
        __assert_fail(
            b"str[0] == 'u'\0" as *const u8 as *const libc::c_char,
            b"src/jansson/src/load.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int32_t decode_unicode_escape(const char *)\0"))
                .as_ptr(),
        );
    };
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        let mut c: libc::c_char = *str.offset(i as isize);
        value <<= 4 as libc::c_int;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            value += c as libc::c_int - '0' as i32;
        } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            value += c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
        } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            value += c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
        } else {
            if 0 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"src/jansson/src/load.c\0" as *const u8 as *const libc::c_char,
                    244 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"int32_t decode_unicode_escape(const char *)\0"))
                        .as_ptr(),
                );
            };
        }
        i += 1;
        i;
    }
    return value;
}
unsafe extern "C" fn lex_scan_string(mut lex: *mut lex_t, mut error: *mut json_error_t) {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    (*lex).value.string = 0 as *mut libc::c_char;
    (*lex).token = -(1 as libc::c_int);
    c = lex_get_save(lex, error) as libc::c_char;
    's_18: loop {
        if !(c as libc::c_int != '"' as i32) {
            current_block = 14763689060501151050;
            break;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            lex_unget_unsave(lex, c);
            if lex_eof(lex) != 0 {
                error_set(
                    error,
                    lex,
                    b"premature end of input\0" as *const u8 as *const libc::c_char,
                );
            }
            current_block = 8176560729030667077;
            break;
        } else if c as libc::c_uchar as libc::c_int <= 0x1f as libc::c_int {
            lex_unget_unsave(lex, c);
            if c as libc::c_int == '\n' as i32 {
                error_set(
                    error,
                    lex,
                    b"unexpected newline\0" as *const u8 as *const libc::c_char,
                );
            } else {
                error_set(
                    error,
                    lex,
                    b"control character 0x%x\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
            }
            current_block = 8176560729030667077;
            break;
        } else if c as libc::c_int == '\\' as i32 {
            c = lex_get_save(lex, error) as libc::c_char;
            if c as libc::c_int == 'u' as i32 {
                c = lex_get_save(lex, error) as libc::c_char;
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        lex_unget_unsave(lex, c);
                        error_set(
                            error,
                            lex,
                            b"invalid escape\0" as *const u8 as *const libc::c_char,
                        );
                        current_block = 8176560729030667077;
                        break 's_18;
                    } else {
                        c = lex_get_save(lex, error) as libc::c_char;
                        i += 1;
                        i;
                    }
                }
            } else if c as libc::c_int == '"' as i32 || c as libc::c_int == '\\' as i32
                || c as libc::c_int == '/' as i32 || c as libc::c_int == 'b' as i32
                || c as libc::c_int == 'f' as i32 || c as libc::c_int == 'n' as i32
                || c as libc::c_int == 'r' as i32 || c as libc::c_int == 't' as i32
            {
                c = lex_get_save(lex, error) as libc::c_char;
            } else {
                lex_unget_unsave(lex, c);
                error_set(
                    error,
                    lex,
                    b"invalid escape\0" as *const u8 as *const libc::c_char,
                );
                current_block = 8176560729030667077;
                break;
            }
        } else {
            c = lex_get_save(lex, error) as libc::c_char;
        }
    }
    match current_block {
        14763689060501151050 => {
            (*lex)
                .value
                .string = malloc(
                ((*lex).saved_text.length + 1 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            if !((*lex).value.string).is_null() {
                t = (*lex).value.string;
                p = (strbuffer_value(&mut (*lex).saved_text))
                    .offset(1 as libc::c_int as isize);
                loop {
                    if !(*p as libc::c_int != '"' as i32) {
                        current_block = 4216521074440650966;
                        break;
                    }
                    if *p as libc::c_int == '\\' as i32 {
                        p = p.offset(1);
                        p;
                        if *p as libc::c_int == 'u' as i32 {
                            let mut buffer: [libc::c_char; 4] = [0; 4];
                            let mut length: libc::c_int = 0;
                            let mut value: int32_t = 0;
                            value = decode_unicode_escape(p);
                            p = p.offset(5 as libc::c_int as isize);
                            if 0xd800 as libc::c_int <= value
                                && value <= 0xdbff as libc::c_int
                            {
                                if *p as libc::c_int == '\\' as i32
                                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                        == 'u' as i32
                                {
                                    p = p.offset(1);
                                    let mut value2: int32_t = decode_unicode_escape(p);
                                    p = p.offset(5 as libc::c_int as isize);
                                    if 0xdc00 as libc::c_int <= value2
                                        && value2 <= 0xdfff as libc::c_int
                                    {
                                        value = ((value - 0xd800 as libc::c_int)
                                            << 10 as libc::c_int) + (value2 - 0xdc00 as libc::c_int)
                                            + 0x10000 as libc::c_int;
                                    } else {
                                        error_set(
                                            error,
                                            lex,
                                            b"invalid Unicode '\\u%04X\\u%04X'\0" as *const u8
                                                as *const libc::c_char,
                                            value,
                                            value2,
                                        );
                                        current_block = 8176560729030667077;
                                        break;
                                    }
                                } else {
                                    error_set(
                                        error,
                                        lex,
                                        b"invalid Unicode '\\u%04X'\0" as *const u8
                                            as *const libc::c_char,
                                        value,
                                    );
                                    current_block = 8176560729030667077;
                                    break;
                                }
                            } else if 0xdc00 as libc::c_int <= value
                                && value <= 0xdfff as libc::c_int
                            {
                                error_set(
                                    error,
                                    lex,
                                    b"invalid Unicode '\\u%04X'\0" as *const u8
                                        as *const libc::c_char,
                                    value,
                                );
                                current_block = 8176560729030667077;
                                break;
                            } else if value == 0 as libc::c_int {
                                error_set(
                                    error,
                                    lex,
                                    b"\\u0000 is not allowed\0" as *const u8
                                        as *const libc::c_char,
                                );
                                current_block = 8176560729030667077;
                                break;
                            }
                            if utf8_encode(value, buffer.as_mut_ptr(), &mut length) != 0
                            {
                                if 0 as libc::c_int != 0 {} else {
                                    __assert_fail(
                                        b"0\0" as *const u8 as *const libc::c_char,
                                        b"src/jansson/src/load.c\0" as *const u8
                                            as *const libc::c_char,
                                        375 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 46],
                                            &[libc::c_char; 46],
                                        >(b"void lex_scan_string(lex_t *, json_error_t *)\0"))
                                            .as_ptr(),
                                    );
                                };
                            }
                            memcpy(
                                t as *mut libc::c_void,
                                buffer.as_mut_ptr() as *const libc::c_void,
                                length as libc::c_ulong,
                            );
                            t = t.offset(length as isize);
                        } else {
                            match *p as libc::c_int {
                                34 | 92 | 47 => {
                                    *t = *p;
                                }
                                98 => {
                                    *t = '\u{8}' as i32 as libc::c_char;
                                }
                                102 => {
                                    *t = '\u{c}' as i32 as libc::c_char;
                                }
                                110 => {
                                    *t = '\n' as i32 as libc::c_char;
                                }
                                114 => {
                                    *t = '\r' as i32 as libc::c_char;
                                }
                                116 => {
                                    *t = '\t' as i32 as libc::c_char;
                                }
                                _ => {
                                    if 0 as libc::c_int != 0 {} else {
                                        __assert_fail(
                                            b"0\0" as *const u8 as *const libc::c_char,
                                            b"src/jansson/src/load.c\0" as *const u8
                                                as *const libc::c_char,
                                            389 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 46],
                                                &[libc::c_char; 46],
                                            >(b"void lex_scan_string(lex_t *, json_error_t *)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                }
                            }
                            t = t.offset(1);
                            t;
                            p = p.offset(1);
                            p;
                        }
                    } else {
                        let fresh1 = p;
                        p = p.offset(1);
                        let fresh2 = t;
                        t = t.offset(1);
                        *fresh2 = *fresh1;
                    }
                }
                match current_block {
                    8176560729030667077 => {}
                    _ => {
                        *t = '\0' as i32 as libc::c_char;
                        (*lex).token = 256 as libc::c_int;
                        return;
                    }
                }
            }
        }
        _ => {}
    }
    free((*lex).value.string as *mut libc::c_void);
}
unsafe extern "C" fn lex_scan_number(
    mut lex: *mut lex_t,
    mut c: libc::c_char,
    mut error: *mut json_error_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut saved_text: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: libc::c_double = 0.;
    (*lex).token = -(1 as libc::c_int);
    if c as libc::c_int == '-' as i32 {
        c = lex_get_save(lex, error) as libc::c_char;
    }
    if c as libc::c_int == '0' as i32 {
        c = lex_get_save(lex, error) as libc::c_char;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            lex_unget_unsave(lex, c);
            current_block = 11479900243588000194;
        } else {
            current_block = 2979737022853876585;
        }
    } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        c = lex_get_save(lex, error) as libc::c_char;
        while *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c = lex_get_save(lex, error) as libc::c_char;
        }
        current_block = 2979737022853876585;
    } else {
        lex_unget_unsave(lex, c);
        current_block = 11479900243588000194;
    }
    match current_block {
        2979737022853876585 => {
            if c as libc::c_int != '.' as i32 && c as libc::c_int != 'E' as i32
                && c as libc::c_int != 'e' as i32
            {
                let mut value_0: json_int_t = 0;
                lex_unget_unsave(lex, c);
                saved_text = strbuffer_value(&mut (*lex).saved_text);
                *__errno_location() = 0 as libc::c_int;
                value_0 = strtoll(saved_text, &mut end, 10 as libc::c_int);
                if *__errno_location() == 34 as libc::c_int {
                    if value_0 < 0 as libc::c_int as libc::c_longlong {
                        error_set(
                            error,
                            lex,
                            b"too big negative integer\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        error_set(
                            error,
                            lex,
                            b"too big integer\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    if end
                        == saved_text.offset((*lex).saved_text.length as isize)
                            as *mut libc::c_char
                    {} else {
                        __assert_fail(
                            b"end == saved_text + lex->saved_text.length\0" as *const u8
                                as *const libc::c_char,
                            b"src/jansson/src/load.c\0" as *const u8
                                as *const libc::c_char,
                            457 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 51],
                                &[libc::c_char; 51],
                            >(b"int lex_scan_number(lex_t *, char, json_error_t *)\0"))
                                .as_ptr(),
                        );
                    };
                    (*lex).token = 257 as libc::c_int;
                    (*lex).value.integer = value_0;
                    return 0 as libc::c_int;
                }
            } else {
                if c as libc::c_int == '.' as i32 {
                    c = lex_get(lex, error) as libc::c_char;
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        current_block = 11479900243588000194;
                    } else {
                        lex_save(lex, c);
                        c = lex_get_save(lex, error) as libc::c_char;
                        while *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                            as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            c = lex_get_save(lex, error) as libc::c_char;
                        }
                        current_block = 14359455889292382949;
                    }
                } else {
                    current_block = 14359455889292382949;
                }
                match current_block {
                    11479900243588000194 => {}
                    _ => {
                        if c as libc::c_int == 'E' as i32
                            || c as libc::c_int == 'e' as i32
                        {
                            c = lex_get_save(lex, error) as libc::c_char;
                            if c as libc::c_int == '+' as i32
                                || c as libc::c_int == '-' as i32
                            {
                                c = lex_get_save(lex, error) as libc::c_char;
                            }
                            if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                            {
                                lex_unget_unsave(lex, c);
                                current_block = 11479900243588000194;
                            } else {
                                c = lex_get_save(lex, error) as libc::c_char;
                                while *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    c = lex_get_save(lex, error) as libc::c_char;
                                }
                                current_block = 11048769245176032998;
                            }
                        } else {
                            current_block = 11048769245176032998;
                        }
                        match current_block {
                            11479900243588000194 => {}
                            _ => {
                                lex_unget_unsave(lex, c);
                                saved_text = strbuffer_value(&mut (*lex).saved_text);
                                value = strtod(saved_text, &mut end);
                                if end
                                    == saved_text.offset((*lex).saved_text.length as isize)
                                        as *mut libc::c_char
                                {} else {
                                    __assert_fail(
                                        b"end == saved_text + lex->saved_text.length\0" as *const u8
                                            as *const libc::c_char,
                                        b"src/jansson/src/load.c\0" as *const u8
                                            as *const libc::c_char,
                                        494 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 51],
                                            &[libc::c_char; 51],
                                        >(b"int lex_scan_number(lex_t *, char, json_error_t *)\0"))
                                            .as_ptr(),
                                    );
                                };
                                if *__errno_location() == 34 as libc::c_int
                                    && value != 0 as libc::c_int as libc::c_double
                                {
                                    error_set(
                                        error,
                                        lex,
                                        b"real number overflow\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    (*lex).token = 258 as libc::c_int;
                                    (*lex).value.real = value;
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn lex_scan(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    strbuffer_clear(&mut (*lex).saved_text);
    if (*lex).token == 256 as libc::c_int {
        free((*lex).value.string as *mut libc::c_void);
        (*lex).value.string = 0 as *mut libc::c_char;
    }
    c = lex_get(lex, error) as libc::c_char;
    while c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
        || c as libc::c_int == '\n' as i32 || c as libc::c_int == '\r' as i32
    {
        if c as libc::c_int == '\n' as i32 {
            (*lex).line += 1;
            (*lex).line;
        }
        c = lex_get(lex, error) as libc::c_char;
    }
    if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
        if lex_eof(lex) != 0 {
            (*lex).token = 0 as libc::c_int;
        } else {
            (*lex).token = -(1 as libc::c_int);
        }
    } else {
        lex_save(lex, c);
        if c as libc::c_int == '{' as i32 || c as libc::c_int == '}' as i32
            || c as libc::c_int == '[' as i32 || c as libc::c_int == ']' as i32
            || c as libc::c_int == ':' as i32 || c as libc::c_int == ',' as i32
        {
            (*lex).token = c as libc::c_int;
        } else if c as libc::c_int == '"' as i32 {
            lex_scan_string(lex, error);
        } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            || c as libc::c_int == '-' as i32
        {
            lex_scan_number(lex, c, error) != 0;
        } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut saved_text: *const libc::c_char = 0 as *const libc::c_char;
            c = lex_get_save(lex, error) as libc::c_char;
            while *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                c = lex_get_save(lex, error) as libc::c_char;
            }
            lex_unget_unsave(lex, c);
            saved_text = strbuffer_value(&mut (*lex).saved_text);
            if strcmp(saved_text, b"true\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*lex).token = 259 as libc::c_int;
            } else if strcmp(saved_text, b"false\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*lex).token = 260 as libc::c_int;
            } else if strcmp(saved_text, b"null\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*lex).token = 261 as libc::c_int;
            } else {
                (*lex).token = -(1 as libc::c_int);
            }
        } else {
            lex_save_cached(lex);
            (*lex).token = -(1 as libc::c_int);
        }
    }
    return (*lex).token;
}
unsafe extern "C" fn lex_steal_string(mut lex: *mut lex_t) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*lex).token == 256 as libc::c_int {
        result = (*lex).value.string;
        (*lex).value.string = 0 as *mut libc::c_char;
    }
    return result;
}
unsafe extern "C" fn lex_init(
    mut lex: *mut lex_t,
    mut get: get_func,
    mut eof: eof_func,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    stream_init(&mut (*lex).stream, get, eof, data);
    if strbuffer_init(&mut (*lex).saved_text) != 0 {
        return -(1 as libc::c_int);
    }
    (*lex).token = -(1 as libc::c_int);
    (*lex).line = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn lex_close(mut lex: *mut lex_t) {
    if (*lex).token == 256 as libc::c_int {
        free((*lex).value.string as *mut libc::c_void);
    }
    strbuffer_close(&mut (*lex).saved_text);
}
unsafe extern "C" fn parse_object(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    let mut current_block: u64;
    let mut object: *mut json_t = json_object();
    if object.is_null() {
        return 0 as *mut json_t;
    }
    lex_scan(lex, error);
    if (*lex).token == '}' as i32 {
        return object;
    }
    loop {
        let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut json_t = 0 as *mut json_t;
        if (*lex).token != 256 as libc::c_int {
            error_set(
                error,
                lex,
                b"string or '}' expected\0" as *const u8 as *const libc::c_char,
            );
            current_block = 220819780955478935;
            break;
        } else {
            key = lex_steal_string(lex);
            if key.is_null() {
                return 0 as *mut json_t;
            }
            lex_scan(lex, error);
            if (*lex).token != ':' as i32 {
                free(key as *mut libc::c_void);
                error_set(
                    error,
                    lex,
                    b"':' expected\0" as *const u8 as *const libc::c_char,
                );
                current_block = 220819780955478935;
                break;
            } else {
                lex_scan(lex, error);
                value = parse_value(lex, error);
                if value.is_null() {
                    free(key as *mut libc::c_void);
                    current_block = 220819780955478935;
                    break;
                } else if json_object_set_nocheck(object, key, value) != 0 {
                    free(key as *mut libc::c_void);
                    json_decref(value);
                    current_block = 220819780955478935;
                    break;
                } else {
                    json_decref(value);
                    free(key as *mut libc::c_void);
                    lex_scan(lex, error);
                    if (*lex).token != ',' as i32 {
                        current_block = 18317007320854588510;
                        break;
                    }
                    lex_scan(lex, error);
                }
            }
        }
    }
    match current_block {
        18317007320854588510 => {
            if (*lex).token != '}' as i32 {
                error_set(
                    error,
                    lex,
                    b"'}' expected\0" as *const u8 as *const libc::c_char,
                );
            } else {
                return object
            }
        }
        _ => {}
    }
    json_decref(object);
    return 0 as *mut json_t;
}
unsafe extern "C" fn parse_array(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    let mut current_block: u64;
    let mut array: *mut json_t = json_array();
    if array.is_null() {
        return 0 as *mut json_t;
    }
    lex_scan(lex, error);
    if (*lex).token == ']' as i32 {
        return array;
    }
    loop {
        if !((*lex).token != 0) {
            current_block = 7746791466490516765;
            break;
        }
        let mut elem: *mut json_t = parse_value(lex, error);
        if elem.is_null() {
            current_block = 5357539143852060714;
            break;
        }
        if json_array_append(array, elem) != 0 {
            json_decref(elem);
            current_block = 5357539143852060714;
            break;
        } else {
            json_decref(elem);
            lex_scan(lex, error);
            if (*lex).token != ',' as i32 {
                current_block = 7746791466490516765;
                break;
            }
            lex_scan(lex, error);
        }
    }
    match current_block {
        7746791466490516765 => {
            if (*lex).token != ']' as i32 {
                error_set(
                    error,
                    lex,
                    b"']' expected\0" as *const u8 as *const libc::c_char,
                );
            } else {
                return array
            }
        }
        _ => {}
    }
    json_decref(array);
    return 0 as *mut json_t;
}
unsafe extern "C" fn parse_value(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    let mut json: *mut json_t = 0 as *mut json_t;
    match (*lex).token {
        256 => {
            json = json_string_nocheck((*lex).value.string);
        }
        257 => {
            json = json_integer((*lex).value.integer);
        }
        258 => {
            json = json_real((*lex).value.real);
        }
        259 => {
            json = json_true();
        }
        260 => {
            json = json_false();
        }
        261 => {
            json = json_null();
        }
        123 => {
            json = parse_object(lex, error);
        }
        91 => {
            json = parse_array(lex, error);
        }
        -1 => {
            error_set(
                error,
                lex,
                b"invalid token\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut json_t;
        }
        _ => {
            error_set(
                error,
                lex,
                b"unexpected token\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut json_t;
        }
    }
    if json.is_null() {
        return 0 as *mut json_t;
    }
    return json;
}
unsafe extern "C" fn parse_json(
    mut lex: *mut lex_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    lex_scan(lex, error);
    if (*lex).token != '[' as i32 && (*lex).token != '{' as i32 {
        error_set(
            error,
            lex,
            b"'[' or '{' expected\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut json_t;
    }
    return parse_value(lex, error);
}
unsafe extern "C" fn string_get(mut data: *mut libc::c_void) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut stream: *mut string_data_t = data as *mut string_data_t;
    c = *((*stream).data).offset((*stream).pos as isize);
    if c as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int)
    } else {
        (*stream).pos += 1;
        (*stream).pos;
        return c as libc::c_int;
    };
}
unsafe extern "C" fn string_eof(mut data: *mut libc::c_void) -> libc::c_int {
    let mut stream: *mut string_data_t = data as *mut string_data_t;
    return (*((*stream).data).offset((*stream).pos as isize) as libc::c_int
        == '\0' as i32) as libc::c_int;
}
pub unsafe extern "C" fn json_loads(
    mut string: *const libc::c_char,
    mut flags: size_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    let mut lex: lex_t = lex_t {
        stream: stream_t {
            get: None,
            eof: None,
            data: 0 as *mut libc::c_void,
            stream_pos: 0,
            buffer: [0; 5],
            buffer_pos: 0,
        },
        saved_text: strbuffer_t {
            value: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
        },
        token: 0,
        line: 0,
        column: 0,
        value: C2RustUnnamed_0 {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut result: *mut json_t = 0 as *mut json_t;
    let mut stream_data: string_data_t = {
        let mut init = string_data_t {
            data: string,
            pos: 0 as libc::c_int,
        };
        init
    };
    if lex_init(
        &mut lex,
        Some(string_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        Some(string_eof as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        &mut stream_data as *mut string_data_t as *mut libc::c_void,
    ) != 0
    {
        return 0 as *mut json_t;
    }
    jsonp_error_init(error, b"<string>\0" as *const u8 as *const libc::c_char);
    result = parse_json(&mut lex, error);
    if !result.is_null() {
        lex_scan(&mut lex, error);
        if lex.token != 0 as libc::c_int {
            error_set(
                error,
                &mut lex as *mut lex_t,
                b"end of file expected\0" as *const u8 as *const libc::c_char,
            );
            json_decref(result);
            result = 0 as *mut json_t;
        }
    }
    lex_close(&mut lex);
    return result;
}
pub unsafe extern "C" fn json_loadf(
    mut input: *mut FILE,
    mut flags: size_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    let mut lex: lex_t = lex_t {
        stream: stream_t {
            get: None,
            eof: None,
            data: 0 as *mut libc::c_void,
            stream_pos: 0,
            buffer: [0; 5],
            buffer_pos: 0,
        },
        saved_text: strbuffer_t {
            value: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
        },
        token: 0,
        line: 0,
        column: 0,
        value: C2RustUnnamed_0 {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut source: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: *mut json_t = 0 as *mut json_t;
    if lex_init(
        &mut lex,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            get_func,
        >(Some(fgetc as unsafe extern "C" fn(*mut FILE) -> libc::c_int)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            eof_func,
        >(Some(feof as unsafe extern "C" fn(*mut FILE) -> libc::c_int)),
        input as *mut libc::c_void,
    ) != 0
    {
        return 0 as *mut json_t;
    }
    if input == stdin {
        source = b"<stdin>\0" as *const u8 as *const libc::c_char;
    } else {
        source = b"<stream>\0" as *const u8 as *const libc::c_char;
    }
    jsonp_error_init(error, source);
    result = parse_json(&mut lex, error);
    if !result.is_null() {
        lex_scan(&mut lex, error);
        if lex.token != 0 as libc::c_int {
            error_set(
                error,
                &mut lex as *mut lex_t,
                b"end of file expected\0" as *const u8 as *const libc::c_char,
            );
            json_decref(result);
            result = 0 as *mut json_t;
        }
    }
    lex_close(&mut lex);
    return result;
}
pub unsafe extern "C" fn json_load_file(
    mut path: *const libc::c_char,
    mut flags: size_t,
    mut error: *mut json_error_t,
) -> *mut json_t {
    let mut result: *mut json_t = 0 as *mut json_t;
    let mut fp: *mut FILE = 0 as *mut FILE;
    jsonp_error_init(error, path);
    fp = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        error_set(
            error,
            0 as *const lex_t,
            b"unable to open %s: %s\0" as *const u8 as *const libc::c_char,
            path,
            strerror(*__errno_location()),
        );
        return 0 as *mut json_t;
    }
    result = json_loadf(fp, flags, error);
    fclose(fp);
    return result;
}
