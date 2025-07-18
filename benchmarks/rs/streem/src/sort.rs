use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn qsort_r(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_d_fn_t,
        __arg: *mut libc::c_void,
    );
    static mut strm_ns_string: *mut strm_state;
    static mut strm_ns_array: *mut strm_state;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_funcall(
        _: *mut strm_stream,
        _: strm_value,
        _: libc::c_int,
        _: *mut strm_value,
        _: *mut strm_value,
    ) -> libc::c_int;
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_ary_new(_: *const strm_value, _: strm_int) -> strm_array;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_string_p(_: strm_value) -> libc::c_int;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_nil_p(_: strm_value) -> libc::c_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_value_int(_: strm_value) -> strm_int;
    fn strm_nil_value() -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    fn strm_bool_value(_: libc::c_int) -> strm_value;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type __compar_d_fn_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *const libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sort_arg {
    pub strm: *mut strm_stream,
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sort_data {
    pub len: strm_int,
    pub capa: strm_int,
    pub buf: *mut strm_value,
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortby_value {
    pub v: strm_value,
    pub o: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortby_data {
    pub len: strm_int,
    pub capa: strm_int,
    pub buf: *mut sortby_value,
    pub strm: *mut strm_stream,
    pub func: strm_value,
}
unsafe extern "C" fn num_cmp(mut x: strm_value, mut y: strm_value) -> libc::c_int {
    let mut a: libc::c_double = strm_value_float(x);
    let mut b: libc::c_double = strm_value_float(y);
    if a > b { return 1 as libc::c_int } else if a < b { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_cmp(mut x: strm_value, mut y: strm_value) -> libc::c_int {
    let mut a: strm_string = x;
    let mut b: strm_string = y;
    let mut alen: strm_int = strm_str_len(a);
    let mut blen: strm_int = strm_str_len(b);
    let mut len: strm_int = 0;
    let mut cmp: strm_int = 0;
    if alen > blen {
        len = blen;
    } else {
        len = alen;
    }
    cmp = memcmp(
        strm_strp_ptr(&mut a) as *const libc::c_void,
        strm_strp_ptr(&mut b) as *const libc::c_void,
        len as libc::c_ulong,
    );
    if cmp == 0 as libc::c_int {
        if alen > len {
            return 1 as libc::c_int;
        }
        if blen > len {
            return -(1 as libc::c_int);
        }
    }
    return cmp;
}
unsafe extern "C" fn strm_cmp(mut a: strm_value, mut b: strm_value) -> libc::c_int {
    if strm_number_p(a) != 0 {
        if strm_number_p(b) != 0 {
            return num_cmp(a, b);
        }
        return -(1 as libc::c_int);
    }
    if strm_string_p(a) != 0 {
        if strm_string_p(b) != 0 {
            return str_cmp(a, b);
        }
        if strm_number_p(b) != 0 {
            return 1 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sort_cmp(
    mut a_p: *const libc::c_void,
    mut b_p: *const libc::c_void,
) -> libc::c_int {
    let mut a: strm_value = *(a_p as *mut strm_value);
    let mut b: strm_value = *(b_p as *mut strm_value);
    return strm_cmp(a, b);
}
unsafe extern "C" fn sort_cmpf(
    mut a_p: *const libc::c_void,
    mut b_p: *const libc::c_void,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut args: [strm_value; 2] = [0; 2];
    let mut a: *mut sort_arg = arg as *mut sort_arg;
    let mut val: strm_value = 0;
    let mut cmp: strm_int = 0;
    args[0 as libc::c_int as usize] = *(a_p as *mut strm_value);
    args[1 as libc::c_int as usize] = *(b_p as *mut strm_value);
    if strm_funcall((*a).strm, (*a).func, 2 as libc::c_int, args.as_mut_ptr(), &mut val)
        == 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if strm_number_p(val) == 0 {
        return 0 as libc::c_int;
    }
    cmp = strm_value_int(val);
    if cmp > 0 as libc::c_int {
        return 1 as libc::c_int
    } else if cmp < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mem_sort(
    mut p: *mut strm_value,
    mut len: strm_int,
    mut arg: *mut sort_arg,
) {
    if !arg.is_null() {
        qsort_r(
            p as *mut libc::c_void,
            len as size_t,
            ::std::mem::size_of::<strm_value>() as libc::c_ulong,
            Some(
                sort_cmpf
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            arg as *mut libc::c_void,
        );
    } else {
        qsort(
            p as *mut libc::c_void,
            len as size_t,
            ::std::mem::size_of::<strm_value>() as libc::c_ulong,
            Some(
                sort_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    };
}
unsafe extern "C" fn iter_sort(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sort_data = (*strm).data as *mut sort_data;
    if (*d).len >= (*d).capa {
        (*d).capa *= 2 as libc::c_int;
        (*d)
            .buf = realloc(
            (*d).buf as *mut libc::c_void,
            (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                .wrapping_mul((*d).capa as libc::c_ulong),
        ) as *mut strm_value;
    }
    let fresh0 = (*d).len;
    (*d).len = (*d).len + 1;
    *((*d).buf).offset(fresh0 as isize) = data;
    return 0 as libc::c_int;
}
unsafe extern "C" fn finish_sort(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sort_data = (*strm).data as *mut sort_data;
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    if strm_nil_p((*d).func) != 0 {
        mem_sort((*d).buf, (*d).len, 0 as *mut sort_arg);
    } else {
        let mut arg: sort_arg = sort_arg {
            strm: 0 as *mut strm_stream,
            func: 0,
        };
        arg.strm = strm;
        arg.func = (*d).func;
        mem_sort((*d).buf, (*d).len, &mut arg);
    }
    i = 0 as libc::c_int;
    len = (*d).len;
    while i < len {
        strm_emit(strm, *((*d).buf).offset(i as isize), None);
        i += 1;
        i;
    }
    free((*d).buf as *mut libc::c_void);
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_sort(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut sort_data = 0 as *mut sort_data;
    let mut func: strm_value = strm_nil_value();
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|v\0" as *const u8 as *const libc::c_char,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<sort_data>() as libc::c_ulong) as *mut sort_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).func = func;
    (*d).len = 0 as libc::c_int;
    (*d).capa = 1024 as libc::c_int;
    (*d)
        .buf = malloc(
        (::std::mem::size_of::<strm_value>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
    ) as *mut strm_value;
    if ((*d).buf).is_null() {
        free(d as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_sort
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                finish_sort
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_sort(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut ary: strm_array = 0;
    let mut p: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    let mut func: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a|v\0" as *const u8 as *const libc::c_char,
        &mut p as *mut *mut strm_value,
        &mut len as *mut strm_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    ary = strm_ary_new(p, len);
    p = (*strm_ary_struct(ary)).ptr;
    if argc == 1 as libc::c_int {
        mem_sort(p, len, 0 as *mut sort_arg);
    } else {
        let mut arg: sort_arg = sort_arg {
            strm: 0 as *mut strm_stream,
            func: 0,
        };
        arg.strm = strm;
        arg.func = func;
        mem_sort(p, len, &mut arg);
    }
    *ret = ary;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sortby_cmp(
    mut a_p: *const libc::c_void,
    mut b_p: *const libc::c_void,
) -> libc::c_int {
    let mut av: *mut sortby_value = a_p as *mut sortby_value;
    let mut bv: *mut sortby_value = b_p as *mut sortby_value;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    if strm_number_p((*av).v) != 0 {
        a = strm_value_float((*av).v);
    } else {
        if strm_number_p((*bv).v) != 0 {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if strm_number_p((*bv).v) != 0 {
        b = strm_value_float((*bv).v);
    } else {
        return -(1 as libc::c_int)
    }
    if a > b { return 1 as libc::c_int } else if a < b { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_sortby(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sortby_data = (*strm).data as *mut sortby_data;
    if (*d).len >= (*d).capa {
        (*d).capa *= 2 as libc::c_int;
        (*d)
            .buf = realloc(
            (*d).buf as *mut libc::c_void,
            (::std::mem::size_of::<sortby_value>() as libc::c_ulong)
                .wrapping_mul((*d).capa as libc::c_ulong),
        ) as *mut sortby_value;
    }
    (*((*d).buf).offset((*d).len as isize)).o = data;
    if strm_funcall(
        (*d).strm,
        (*d).func,
        1 as libc::c_int,
        &mut data,
        &mut (*((*d).buf).offset((*d).len as isize)).v,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    (*d).len += 1;
    (*d).len;
    return 0 as libc::c_int;
}
unsafe extern "C" fn finish_sortby(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sortby_data = (*strm).data as *mut sortby_data;
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    qsort(
        (*d).buf as *mut libc::c_void,
        (*d).len as size_t,
        ::std::mem::size_of::<sortby_value>() as libc::c_ulong,
        Some(
            sortby_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    len = (*d).len;
    while i < len {
        strm_emit(strm, (*((*d).buf).offset(i as isize)).o, None);
        i += 1;
        i;
    }
    free((*d).buf as *mut libc::c_void);
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_sortby(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut sortby_data = 0 as *mut sortby_data;
    let mut func: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<sortby_data>() as libc::c_ulong)
        as *mut sortby_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).strm = strm;
    (*d).func = func;
    (*d).len = 0 as libc::c_int;
    (*d).capa = 1024 as libc::c_int;
    (*d)
        .buf = malloc(
        (::std::mem::size_of::<sortby_value>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
    ) as *mut sortby_value;
    if ((*d).buf).is_null() {
        free(d as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_sortby
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                finish_sortby
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_sortby(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut buf: *mut sortby_value = 0 as *mut sortby_value;
    let mut p: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    let mut func: strm_value = 0;
    let mut ary: strm_array = 0;
    let mut i: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"av\0" as *const u8 as *const libc::c_char,
        &mut p as *mut *mut strm_value,
        &mut len as *mut strm_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    buf = malloc(
        (::std::mem::size_of::<sortby_value>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong),
    ) as *mut sortby_value;
    if buf.is_null() {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < len {
        (*buf.offset(i as isize)).o = *p.offset(i as isize);
        if strm_funcall(
            strm,
            func,
            1 as libc::c_int,
            &mut *p.offset(i as isize),
            &mut (*buf.offset(i as isize)).v,
        ) == 1 as libc::c_int
        {
            free(buf as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    qsort(
        buf as *mut libc::c_void,
        len as size_t,
        ::std::mem::size_of::<sortby_value>() as libc::c_ulong,
        Some(
            sortby_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    ary = strm_ary_new(0 as *const strm_value, len);
    p = (*strm_ary_struct(ary)).ptr;
    i = 0 as libc::c_int;
    while i < len {
        *p.offset(i as isize) = (*buf.offset(i as isize)).o;
        i += 1;
        i;
    }
    free(buf as *mut libc::c_void);
    *ret = ary;
    return 0 as libc::c_int;
}
unsafe extern "C" fn quick_select(
    mut arr: *mut strm_value,
    mut n: libc::c_int,
) -> strm_value {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut median: libc::c_int = 0;
    let mut middle: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut hh: libc::c_int = 0;
    low = 0 as libc::c_int;
    high = n - 1 as libc::c_int;
    median = (low + high) / 2 as libc::c_int;
    loop {
        if high <= low {
            return *arr.offset(median as isize);
        }
        if high == low + 1 as libc::c_int {
            if strm_cmp(*arr.offset(low as isize), *arr.offset(high as isize))
                > 0 as libc::c_int
            {
                let mut t: strm_value = *arr.offset(low as isize);
                *arr.offset(low as isize) = *arr.offset(high as isize);
                *arr.offset(high as isize) = t;
            }
            return *arr.offset(median as isize);
        }
        middle = (low + high) / 2 as libc::c_int;
        if strm_cmp(*arr.offset(middle as isize), *arr.offset(high as isize))
            > 0 as libc::c_int
        {
            let mut t_0: strm_value = *arr.offset(middle as isize);
            *arr.offset(middle as isize) = *arr.offset(high as isize);
            *arr.offset(high as isize) = t_0;
        }
        if strm_cmp(*arr.offset(low as isize), *arr.offset(high as isize))
            > 0 as libc::c_int
        {
            let mut t_1: strm_value = *arr.offset(low as isize);
            *arr.offset(low as isize) = *arr.offset(high as isize);
            *arr.offset(high as isize) = t_1;
        }
        if strm_cmp(*arr.offset(middle as isize), *arr.offset(low as isize))
            > 0 as libc::c_int
        {
            let mut t_2: strm_value = *arr.offset(middle as isize);
            *arr.offset(middle as isize) = *arr.offset(low as isize);
            *arr.offset(low as isize) = t_2;
        }
        let mut t_3: strm_value = *arr.offset(middle as isize);
        *arr.offset(middle as isize) = *arr.offset((low + 1 as libc::c_int) as isize);
        *arr.offset((low + 1 as libc::c_int) as isize) = t_3;
        ll = low + 1 as libc::c_int;
        hh = high;
        loop {
            loop {
                ll += 1;
                ll;
                if !(strm_cmp(*arr.offset(low as isize), *arr.offset(ll as isize))
                    > 0 as libc::c_int)
                {
                    break;
                }
            }
            loop {
                hh -= 1;
                hh;
                if !(strm_cmp(*arr.offset(hh as isize), *arr.offset(low as isize))
                    > 0 as libc::c_int)
                {
                    break;
                }
            }
            if hh < ll {
                break;
            }
            let mut t_4: strm_value = *arr.offset(ll as isize);
            *arr.offset(ll as isize) = *arr.offset(hh as isize);
            *arr.offset(hh as isize) = t_4;
        }
        let mut t_5: strm_value = *arr.offset(low as isize);
        *arr.offset(low as isize) = *arr.offset(hh as isize);
        *arr.offset(hh as isize) = t_5;
        if hh <= median {
            low = ll;
        }
        if hh >= median {
            high = hh - 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn quick_median(
    mut p: *mut strm_value,
    mut len: libc::c_int,
) -> strm_value {
    let mut v: strm_value = quick_select(p, len);
    if len % 2 as libc::c_int == 0 as libc::c_int && strm_number_p(v) != 0 {
        let mut next: strm_int = len / 2 as libc::c_int;
        if strm_number_p(*p.offset(next as isize)) != 0 {
            let mut x: libc::c_double = strm_value_float(v);
            let mut y: libc::c_double = strm_value_float(*p.offset(next as isize));
            return strm_float_value((x + y) / 2 as libc::c_int as libc::c_double);
        }
    }
    return v;
}
unsafe extern "C" fn iter_median(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sort_data = (*strm).data as *mut sort_data;
    if (*d).len >= (*d).capa {
        (*d).capa *= 2 as libc::c_int;
        (*d)
            .buf = realloc(
            (*d).buf as *mut libc::c_void,
            (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                .wrapping_mul((*d).capa as libc::c_ulong),
        ) as *mut strm_value;
    }
    if strm_nil_p((*d).func) != 0 {
        let fresh1 = (*d).len;
        (*d).len = (*d).len + 1;
        *((*d).buf).offset(fresh1 as isize) = data;
    } else {
        let fresh2 = (*d).len;
        (*d).len = (*d).len + 1;
        if strm_funcall(
            strm,
            (*d).func,
            1 as libc::c_int,
            &mut data,
            &mut *((*d).buf).offset(fresh2 as isize),
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn finish_median(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sort_data = (*strm).data as *mut sort_data;
    let mut v: strm_value = 0;
    v = quick_median((*d).buf, (*d).len);
    free((*d).buf as *mut libc::c_void);
    strm_emit(strm, v, None);
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_median(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut sort_data = 0 as *mut sort_data;
    let mut func: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|v\0" as *const u8 as *const libc::c_char,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<sort_data>() as libc::c_ulong) as *mut sort_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).func = if argc == 0 as libc::c_int { strm_nil_value() } else { func };
    (*d).len = 0 as libc::c_int;
    (*d).capa = 1024 as libc::c_int;
    (*d)
        .buf = malloc(
        (::std::mem::size_of::<strm_value>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
    ) as *mut strm_value;
    if ((*d).buf).is_null() {
        free(d as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_median
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                finish_median
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_median(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut buf: *mut strm_value = 0 as *mut strm_value;
    let mut p: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    let mut func: strm_value = 0;
    let mut i: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a|v\0" as *const u8 as *const libc::c_char,
        &mut p as *mut *mut strm_value,
        &mut len as *mut strm_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if len == 0 as libc::c_int {
        strm_raise(strm, b"empty array\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    buf = malloc(
        (::std::mem::size_of::<strm_value>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong),
    ) as *mut strm_value;
    if buf.is_null() {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        memcpy(
            buf as *mut libc::c_void,
            p as *const libc::c_void,
            (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        );
    } else {
        let mut func_0: strm_value = *args.offset(1 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < len {
            if strm_funcall(
                strm,
                func_0,
                1 as libc::c_int,
                &mut *p.offset(i as isize),
                &mut *buf.offset(i as isize),
            ) == 1 as libc::c_int
            {
                free(buf as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
    }
    *ret = quick_median(buf, len);
    free(buf as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_cmp(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut cmp: strm_int = 0;
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"vv\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    cmp = strm_cmp(x, y);
    *ret = strm_int_value(cmp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_lt(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    let mut cmp: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"SS\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    cmp = str_cmp(x, y);
    *ret = strm_bool_value((cmp < 0 as libc::c_int) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_le(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    let mut cmp: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"SS\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    cmp = str_cmp(x, y);
    *ret = strm_bool_value((cmp <= 0 as libc::c_int) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_gt(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    let mut cmp: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"SS\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    cmp = str_cmp(x, y);
    *ret = strm_bool_value((cmp > 0 as libc::c_int) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_ge(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    let mut cmp: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"SS\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    cmp = str_cmp(x, y);
    *ret = strm_bool_value((cmp >= 0 as libc::c_int) as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_sort_init(mut state: *mut strm_state) {
    strm_var_def(
        strm_ns_array,
        b"sort\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_sort
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
        strm_ns_array,
        b"sort_by\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_sortby
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
        strm_ns_array,
        b"median\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_median
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
        b"cmp\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_cmp
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
        b"sort\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_sort
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
        b"sort_by\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_sortby
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
        b"median\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_median
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
        b"<\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_lt
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
        b"<=\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_le
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
        b">\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_gt
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
        b">=\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_ge
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
