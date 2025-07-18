use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_nil_value() -> strm_value;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_string_p(_: strm_value) -> libc::c_int;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_str_intern(p: *const libc::c_char, len: strm_int) -> strm_string;
    fn strm_str_intern_str(s: strm_string) -> strm_string;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_ary_new(_: *const strm_value, _: strm_int) -> strm_array;
    static mut strm_ns_string: *mut strm_state;
    fn strm_time_p(_: strm_value) -> libc::c_int;
    fn strm_time_new(
        sec: libc::c_long,
        usec: libc::c_long,
        offset: libc::c_int,
    ) -> strm_value;
    fn strm_time_parse_time(
        s: *const libc::c_char,
        len: strm_int,
        sec: *mut libc::c_long,
        usec: *mut libc::c_long,
        offset: *mut libc::c_int,
    ) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_string = uint64_t;
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_array_0 {
    pub len: strm_int,
    pub ptr: *mut strm_value,
    pub headers: strm_array,
    pub ns: *mut strm_state,
}
pub type csv_type = libc::c_uint;
pub const TYPE_FLOAT: csv_type = 6;
pub const TYPE_INT: csv_type = 5;
pub const TYPE_ESC: csv_type = 4;
pub const TYPE_TIME: csv_type = 3;
pub const TYPE_NUM: csv_type = 2;
pub const TYPE_STR: csv_type = 1;
pub const TYPE_UNSPC: csv_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_data {
    pub headers: strm_array,
    pub types: *mut csv_type,
    pub prev: strm_string,
    pub sep: libc::c_char,
    pub n: libc::c_int,
}
unsafe extern "C" fn count_fields(
    mut line: strm_string,
    mut sep: libc::c_char,
) -> libc::c_int {
    let mut ptr: *const libc::c_char = strm_strp_ptr(&mut line);
    let mut pend: *const libc::c_char = ptr.offset(strm_str_len(line) as isize);
    let mut cnt: libc::c_int = 0;
    let mut quoted: libc::c_int = 0 as libc::c_int;
    let mut current_block_5: u64;
    cnt = 1 as libc::c_int;
    while ptr < pend {
        if quoted != 0 {
            if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                if *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                    ptr = ptr.offset(1);
                    ptr;
                } else {
                    quoted = 0 as libc::c_int;
                }
            }
        } else {
            match *ptr as libc::c_int {
                34 => {
                    current_block_5 = 11206632339930292516;
                    match current_block_5 {
                        15270969162102947270 => {
                            if *ptr as libc::c_int == sep as libc::c_int {
                                cnt += 1;
                                cnt;
                            }
                        }
                        _ => {
                            quoted = 1 as libc::c_int;
                        }
                    }
                }
                44 | 9 => {
                    current_block_5 = 15270969162102947270;
                    match current_block_5 {
                        15270969162102947270 => {
                            if *ptr as libc::c_int == sep as libc::c_int {
                                cnt += 1;
                                cnt;
                            }
                        }
                        _ => {
                            quoted = 1 as libc::c_int;
                        }
                    }
                }
                _ => {}
            }
        }
        ptr = ptr.offset(1);
        ptr;
    }
    if quoted != 0 {
        return -(1 as libc::c_int);
    }
    return cnt;
}
unsafe extern "C" fn csv_string(
    mut p: *const libc::c_char,
    mut len: strm_int,
    mut ftype: csv_type,
) -> strm_value {
    let mut str: strm_string = 0;
    match ftype as libc::c_uint {
        4 => {
            let mut pend: *const libc::c_char = p.offset(len as isize);
            let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut in_quote: libc::c_int = 0 as libc::c_int;
            s = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
            t = s;
            while p < pend {
                if in_quote != 0 {
                    if *p as libc::c_int == '"' as i32 {
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '"' as i32
                        {
                            p = p.offset(1);
                            p;
                            let fresh0 = t;
                            t = t.offset(1);
                            *fresh0 = '"' as i32 as libc::c_char;
                            continue;
                        } else {
                            in_quote = 0 as libc::c_int;
                        }
                    } else {
                        let fresh1 = t;
                        t = t.offset(1);
                        *fresh1 = *p;
                    }
                } else if *p as libc::c_int == '"' as i32 {
                    in_quote = 1 as libc::c_int;
                } else {
                    let fresh2 = t;
                    t = t.offset(1);
                    *fresh2 = *p;
                }
                p = p.offset(1);
                p;
            }
            str = strm_str_new(s, t.offset_from(s) as libc::c_long as strm_int);
            free(s as *mut libc::c_void);
        }
        _ => {
            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut sec: libc::c_long = 0;
                let mut usec: libc::c_long = 0;
                let mut offset: libc::c_int = 0;
                if strm_time_parse_time(p, len, &mut sec, &mut usec, &mut offset)
                    == 0 as libc::c_int
                {
                    return strm_time_new(sec, usec, offset);
                }
            }
            str = strm_str_new(p, len);
        }
    }
    return str;
}
unsafe extern "C" fn csv_value(
    mut p: *const libc::c_char,
    mut len: strm_int,
    mut ftype: csv_type,
) -> strm_value {
    let mut s: *const libc::c_char = p;
    let mut send: *const libc::c_char = s.offset(len as isize);
    let mut i: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut f: libc::c_double = 0.;
    let mut pow: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut type_0: csv_type = TYPE_STR;
    match ftype as libc::c_uint {
        0 | 2 => {
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                s = s.offset(1);
                s;
            }
            while s < send {
                match *s as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        if type_0 as libc::c_uint
                            == TYPE_STR as libc::c_int as libc::c_uint
                        {
                            type_0 = TYPE_INT;
                        }
                        i = i * 10 as libc::c_int as libc::c_long
                            + (*s as libc::c_int - '0' as i32) as libc::c_long;
                        pow *= 10 as libc::c_int as libc::c_double;
                    }
                    46 => {
                        if type_0 as libc::c_uint
                            == TYPE_FLOAT as libc::c_int as libc::c_uint
                        {
                            type_0 = TYPE_TIME;
                        } else {
                            type_0 = TYPE_FLOAT;
                            f = i as libc::c_double;
                            i = 0 as libc::c_int as libc::c_long;
                            pow = 1 as libc::c_int as libc::c_double;
                        }
                    }
                    _ => {
                        type_0 = TYPE_UNSPC;
                    }
                }
                s = s.offset(1);
                s;
            }
        }
        _ => {}
    }
    match type_0 as libc::c_uint {
        5 => return strm_int_value(i as strm_int),
        6 => {
            f += i as libc::c_double / pow;
            return strm_float_value(f);
        }
        _ => return csv_string(p, len, ftype),
    };
}
pub unsafe extern "C" fn csv_type(mut v: strm_value) -> csv_type {
    if strm_number_p(v) != 0 {
        return TYPE_NUM;
    }
    if strm_time_p(v) != 0 { return TYPE_TIME } else { return TYPE_STR };
}
unsafe extern "C" fn sv_accept(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut ary: strm_array = 0;
    let mut line: strm_string = data;
    let mut bp: *mut strm_value = 0 as *mut strm_value;
    let mut fbeg: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pend: *const libc::c_char = 0 as *const libc::c_char;
    let mut fieldcnt: libc::c_int = 0;
    let mut in_quote: libc::c_int = 0 as libc::c_int;
    let mut all_str: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ftype: csv_type = TYPE_UNSPC;
    let mut types: *mut csv_type = 0 as *mut csv_type;
    let mut cd: *mut csv_data = (*strm).data as *mut csv_data;
    let mut sep: libc::c_char = (*cd).sep;
    if (*cd).prev != 0 {
        let mut len: strm_int = strm_str_len((*cd).prev) + strm_str_len(line)
            + 1 as libc::c_int;
        let mut tmp: *mut libc::c_char = malloc(len as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(
            tmp as *mut libc::c_void,
            strm_strp_ptr(&mut (*cd).prev) as *const libc::c_void,
            strm_str_len((*cd).prev) as libc::c_ulong,
        );
        *tmp.offset(strm_str_len((*cd).prev) as isize) = '\n' as i32 as libc::c_char;
        memcpy(
            tmp
                .offset(strm_str_len((*cd).prev) as isize)
                .offset(1 as libc::c_int as isize) as *mut libc::c_void,
            strm_strp_ptr(&mut line) as *const libc::c_void,
            strm_str_len(line) as libc::c_ulong,
        );
        line = strm_str_new(tmp, len);
        free(tmp as *mut libc::c_void);
        (*cd).prev = 0 as libc::c_int as strm_string;
    }
    fieldcnt = count_fields(line, (*cd).sep);
    if fieldcnt == -(1 as libc::c_int) {
        (*cd).prev = line;
        return 1 as libc::c_int;
    }
    if (*cd).n > 0 as libc::c_int && fieldcnt != (*cd).n {
        return 1 as libc::c_int;
    }
    ptr = strm_strp_ptr(&mut line);
    pend = ptr.offset(strm_str_len(line) as isize);
    ary = strm_ary_new(0 as *const strm_value, fieldcnt);
    if ary == 0 {
        return 1 as libc::c_int;
    }
    bp = (*strm_ary_struct(ary)).ptr;
    types = (*cd).types;
    ftype = (if !types.is_null() {
        *types.offset(0 as libc::c_int as isize) as libc::c_uint
    } else {
        TYPE_UNSPC as libc::c_int as libc::c_uint
    }) as csv_type;
    let mut current_block_40: u64;
    fbeg = ptr;
    while ptr < pend {
        if in_quote != 0 {
            if *ptr as libc::c_int == '"' as i32 {
                if *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                    ptr = ptr.offset(1);
                    ptr;
                    ftype = TYPE_ESC;
                } else {
                    in_quote = 0 as libc::c_int;
                }
            }
        } else {
            match *ptr as libc::c_int {
                34 => {
                    current_block_40 = 4161986340281116891;
                    match current_block_40 {
                        4161986340281116891 => {
                            in_quote = 1 as libc::c_int;
                            if ptr == fbeg {
                                ftype = TYPE_STR;
                                fbeg = ptr.offset(1 as libc::c_int as isize);
                            } else {
                                ftype = TYPE_ESC;
                            }
                        }
                        _ => {
                            if !(*ptr as libc::c_int != sep as libc::c_int) {
                                *bp = csv_value(
                                    fbeg,
                                    ptr.offset_from(fbeg) as libc::c_long as strm_int,
                                    ftype,
                                );
                                if strm_string_p(*bp) == 0 {
                                    all_str = 0 as libc::c_int;
                                }
                                bp = bp.offset(1);
                                bp;
                                fbeg = ptr.offset(1 as libc::c_int as isize);
                                i += 1;
                                i;
                                ftype = (if !types.is_null() {
                                    *types.offset(i as isize) as libc::c_uint
                                } else {
                                    TYPE_UNSPC as libc::c_int as libc::c_uint
                                }) as csv_type;
                            }
                        }
                    }
                }
                44 | 9 => {
                    current_block_40 = 8750500724598335420;
                    match current_block_40 {
                        4161986340281116891 => {
                            in_quote = 1 as libc::c_int;
                            if ptr == fbeg {
                                ftype = TYPE_STR;
                                fbeg = ptr.offset(1 as libc::c_int as isize);
                            } else {
                                ftype = TYPE_ESC;
                            }
                        }
                        _ => {
                            if !(*ptr as libc::c_int != sep as libc::c_int) {
                                *bp = csv_value(
                                    fbeg,
                                    ptr.offset_from(fbeg) as libc::c_long as strm_int,
                                    ftype,
                                );
                                if strm_string_p(*bp) == 0 {
                                    all_str = 0 as libc::c_int;
                                }
                                bp = bp.offset(1);
                                bp;
                                fbeg = ptr.offset(1 as libc::c_int as isize);
                                i += 1;
                                i;
                                ftype = (if !types.is_null() {
                                    *types.offset(i as isize) as libc::c_uint
                                } else {
                                    TYPE_UNSPC as libc::c_int as libc::c_uint
                                }) as csv_type;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32 {
        ptr = ptr.offset(-1);
        ptr;
    }
    if *ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32 {
        ptr = ptr.offset(-1);
        ptr;
    }
    *bp = csv_value(fbeg, ptr.offset_from(fbeg) as libc::c_long as strm_int, ftype);
    if strm_string_p(*bp) == 0 {
        all_str = 0 as libc::c_int;
    }
    if (*cd).headers == 0 && ((*cd).types).is_null() {
        if all_str != 0 {
            (*cd).headers = ary;
            ary = 0 as libc::c_int as strm_array;
        }
        (*cd).n = fieldcnt;
    }
    if ary != 0 {
        if (*cd).headers != 0 {
            (*strm_ary_struct(ary)).headers = (*cd).headers;
        }
        if ((*cd).types).is_null() {
            if (*cd).headers != 0 {
                if all_str != 0 {
                    strm_emit(strm, (*cd).headers, None);
                    (*cd).headers = 0 as libc::c_int as strm_array;
                } else {
                    let mut h: strm_array = (*cd).headers;
                    let mut p: *mut strm_value = (*strm_ary_struct(h)).ptr;
                    let mut i_0: libc::c_int = 0;
                    i_0 = 0 as libc::c_int;
                    while i_0 < (*strm_ary_struct(h)).len {
                        let mut str: strm_string = *p.offset(i_0 as isize);
                        *p.offset(i_0 as isize) = strm_str_intern_str(str);
                        i_0 += 1;
                        i_0;
                    }
                }
            }
            (*cd)
                .types = malloc(
                (::std::mem::size_of::<csv_type>() as libc::c_ulong)
                    .wrapping_mul(fieldcnt as libc::c_ulong),
            ) as *mut csv_type;
            if ((*cd).types).is_null() {
                return 1 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < fieldcnt {
                *((*cd).types)
                    .offset(
                        i as isize,
                    ) = csv_type(*((*strm_ary_struct(ary)).ptr).offset(i as isize));
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i < fieldcnt {
                if *((*cd).types).offset(i as isize) as libc::c_uint
                    != csv_type(*((*strm_ary_struct(ary)).ptr).offset(i as isize))
                        as libc::c_uint
                {
                    strm_raise(
                        strm,
                        b"csv type mismatch\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
        strm_emit(strm, ary, None);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sv_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut cd: *mut csv_data = (*strm).data as *mut csv_data;
    if (*cd).headers != 0 && ((*cd).types).is_null() {
        strm_emit(strm, (*cd).headers, None);
        (*cd).headers = 0 as libc::c_int as strm_array;
    }
    if !((*cd).types).is_null() {
        free((*cd).types as *mut libc::c_void);
        (*cd).types = 0 as *mut csv_type;
    }
    free(cd as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn sv(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut sep: libc::c_char,
) -> libc::c_int {
    let mut t: *mut strm_stream = 0 as *mut strm_stream;
    let mut cd: *mut csv_data = 0 as *mut csv_data;
    if strm_parse_args(strm, argc, args, b"\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    cd = malloc(::std::mem::size_of::<csv_data>() as libc::c_ulong) as *mut csv_data;
    if cd.is_null() {
        return 1 as libc::c_int;
    }
    (*cd).headers = 0 as libc::c_int as strm_array;
    (*cd).types = 0 as *mut csv_type;
    (*cd).prev = 0 as libc::c_int as strm_string;
    (*cd).sep = sep;
    (*cd).n = 0 as libc::c_int;
    t = strm_stream_new(
        strm_filter,
        Some(
            sv_accept
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        Some(
            sv_finish
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        cd as *mut libc::c_void,
    );
    *ret = strm_ptr_value(t as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn csv(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return sv(strm, argc, args, ret, ',' as i32 as libc::c_char);
}
unsafe extern "C" fn tsv(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return sv(strm, argc, args, ret, '\t' as i32 as libc::c_char);
}
unsafe extern "C" fn ltsv_accept(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    if strm_string_p(data) == 0 {
        strm_raise(strm, b"ltsv: string required\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut p: *const libc::c_char = strm_strp_ptr(&mut data);
    let mut len: strm_int = strm_str_len(data);
    let mut pend: *const libc::c_char = p.offset(len as isize);
    let mut s: *const libc::c_char = p;
    let mut nval: libc::c_int = 0 as libc::c_int;
    while p < pend {
        s = memchr(
            p as *const libc::c_void,
            '\t' as i32,
            pend.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        nval += 1;
        nval;
        if s.is_null() {
            break;
        }
        p = s.offset(1 as libc::c_int as isize);
    }
    let mut ary: strm_array = strm_ary_new(0 as *const strm_value, nval);
    let mut v: *mut strm_value = (*strm_ary_struct(ary)).ptr;
    let mut hdr: strm_array = strm_ary_new(0 as *const strm_value, nval);
    let mut h: *mut strm_value = (*strm_ary_struct(hdr)).ptr;
    p = strm_strp_ptr(&mut data);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nval {
        let mut str: strm_string = 0;
        let mut rend: *const libc::c_char = memchr(
            p as *const libc::c_void,
            '\t' as i32,
            pend.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if rend.is_null() {
            rend = pend;
        }
        s = memchr(
            p as *const libc::c_void,
            ':' as i32,
            rend.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if s.is_null() {
            *h.offset(i as isize) = strm_nil_value();
        } else {
            str = strm_str_intern(p, s.offset_from(p) as libc::c_long as strm_int);
            *h.offset(i as isize) = str;
            p = s.offset(1 as libc::c_int as isize);
        }
        if p < rend {
            *v
                .offset(
                    i as isize,
                ) = csv_value(
                p,
                rend.offset_from(p) as libc::c_long as strm_int,
                TYPE_UNSPC,
            );
        } else {
            str = strm_str_new(0 as *const libc::c_char, 0 as libc::c_int);
            *v.offset(i as isize) = str;
        }
        p = rend.offset(1 as libc::c_int as isize);
        i += 1;
        i;
    }
    (*strm_ary_struct(ary)).headers = hdr;
    strm_emit(strm, ary, None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ltsv(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t: *mut strm_stream = 0 as *mut strm_stream;
    if strm_parse_args(strm, argc, args, b"\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = strm_stream_new(
        strm_filter,
        Some(
            ltsv_accept
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        None,
        0 as *mut libc::c_void,
    );
    *ret = strm_ptr_value(t as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_number(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut s: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"S\0" as *const u8 as *const libc::c_char,
        &mut s as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    s = csv_value(strm_strp_ptr(&mut s), strm_str_len(s), TYPE_NUM);
    if strm_number_p(s) == 0 {
        strm_raise(
            strm,
            b"invalid string for number\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    *ret = s;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_csv_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"csv\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                csv
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"tsv\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                tsv
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"ltsv\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ltsv
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        strm_ns_string,
        b"number\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_number
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
