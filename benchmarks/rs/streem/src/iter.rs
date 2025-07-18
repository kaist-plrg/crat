use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_nil_value() -> strm_value;
    fn strm_value_bool(_: strm_value) -> libc::c_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_value_eq(_: strm_value, _: strm_value) -> libc::c_int;
    fn strm_nil_p(_: strm_value) -> libc::c_int;
    fn strm_array_p(_: strm_value) -> libc::c_int;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_ary_new(_: *const strm_value, _: strm_int) -> strm_array;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
    fn strm_stream_close(strm: *mut strm_stream);
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_funcall(
        _: *mut strm_stream,
        _: strm_value,
        _: libc::c_int,
        _: *mut strm_value,
        _: *mut strm_value,
    ) -> libc::c_int;
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    static mut strm_ns_array: *mut strm_state;
    fn strm_stat_init(state: *mut strm_state);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
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
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_array_0 {
    pub len: strm_int,
    pub ptr: *mut strm_value,
    pub headers: strm_array,
    pub ns: *mut strm_state,
}
pub type khint32_t = libc::c_uint;
pub type khint64_t = libc::c_ulong;
pub type khint_t = khint32_t;
pub type khiter_t = khint_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_data {
    pub n: libc::c_double,
    pub end: libc::c_double,
    pub inc: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repeat_data {
    pub v: strm_value,
    pub count: strm_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cycle_data {
    pub ary: strm_array,
    pub count: strm_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map_data {
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct count_data {
    pub count: strm_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minmax_data {
    pub start: libc::c_int,
    pub min: libc::c_int,
    pub data: strm_value,
    pub num: libc::c_double,
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reduce_data {
    pub init: strm_int,
    pub acc: strm_value,
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_rbk_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut khint64_t,
    pub vals: *mut strm_value,
}
pub type kh_rbk_t = kh_rbk_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbk_data {
    pub tbl: *mut kh_rbk_t,
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slice_data {
    pub n: strm_int,
    pub i: strm_int,
    pub buf: *mut strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct take_data {
    pub n: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uniq_data {
    pub last: strm_value,
    pub v: strm_value,
    pub func: strm_value,
    pub init: libc::c_int,
}
static mut khash_ac_HASH_UPPER: libc::c_double = 0.77f64;
unsafe extern "C" fn gen_seq(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut seq_data = (*strm).data as *mut seq_data;
    if (*d).end > 0 as libc::c_int as libc::c_double && (*d).n > (*d).end {
        strm_stream_close(strm);
        return 0 as libc::c_int;
    }
    strm_emit(
        strm,
        strm_float_value((*d).n),
        Some(
            gen_seq as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    (*d).n += (*d).inc;
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_seq(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut start: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut end: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut inc: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut tmp: libc::c_double = 0.;
    let mut d: *mut seq_data = 0 as *mut seq_data;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|fff\0" as *const u8 as *const libc::c_char,
        &mut start as *mut libc::c_double,
        &mut end as *mut libc::c_double,
        &mut tmp as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    match argc {
        1 => {
            end = start;
            start = 1 as libc::c_int as libc::c_double;
        }
        3 => {
            inc = end;
            end = tmp;
        }
        _ => {}
    }
    d = malloc(::std::mem::size_of::<seq_data>() as libc::c_ulong) as *mut seq_data;
    (*d).n = start;
    (*d).inc = inc;
    (*d).end = end;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                gen_seq
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn gen_repeat(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut repeat_data = (*strm).data as *mut repeat_data;
    (*d).count -= 1;
    (*d).count;
    if (*d).count == 0 as libc::c_int {
        strm_emit(strm, (*d).v, None);
        strm_stream_close(strm);
    } else {
        strm_emit(
            strm,
            (*d).v,
            Some(
                gen_repeat
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fin_repeat(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    free((*strm).data);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_repeat(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut v: strm_value = 0;
    let mut n: strm_int = -(1 as libc::c_int);
    let mut d: *mut repeat_data = 0 as *mut repeat_data;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v|i\0" as *const u8 as *const libc::c_char,
        &mut v as *mut strm_value,
        &mut n as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 2 as libc::c_int && n <= 0 as libc::c_int {
        strm_raise(strm, b"invalid count number\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<repeat_data>() as libc::c_ulong)
        as *mut repeat_data;
    (*d).v = v;
    (*d).count = n;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                gen_repeat
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                fin_repeat
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn gen_cycle(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut cycle_data = (*strm).data as *mut cycle_data;
    let mut p: *mut strm_value = 0 as *mut strm_value;
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    (*d).count -= 1;
    (*d).count;
    p = (*strm_ary_struct((*d).ary)).ptr;
    len = (*strm_ary_struct((*d).ary)).len;
    if (*d).count != 0 as libc::c_int {
        len -= 1;
        len;
    }
    i = 0 as libc::c_int;
    while i < len {
        strm_emit(strm, *p.offset(i as isize), None);
        i += 1;
        i;
    }
    if (*d).count == 0 as libc::c_int {
        strm_stream_close(strm);
    } else {
        strm_emit(
            strm,
            *p.offset(i as isize),
            Some(
                gen_cycle
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fin_cycle(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    free((*strm).data);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_cycle(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut a: strm_array = 0;
    let mut n: strm_int = -(1 as libc::c_int);
    let mut d: *mut cycle_data = 0 as *mut cycle_data;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"A|i\0" as *const u8 as *const libc::c_char,
        &mut a as *mut strm_array,
        &mut n as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 2 as libc::c_int && n <= 0 as libc::c_int {
        strm_raise(strm, b"invalid count number\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<cycle_data>() as libc::c_ulong) as *mut cycle_data;
    (*d).ary = a;
    (*d).count = n;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                gen_cycle
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                fin_cycle
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_each(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = (*strm).data as *mut map_data;
    let mut val: strm_value = 0;
    if strm_funcall(strm, (*d).func, 1 as libc::c_int, &mut data, &mut val)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_each(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = 0 as *mut map_data;
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
    d = malloc(::std::mem::size_of::<map_data>() as libc::c_ulong) as *mut map_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).func = func;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_each
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_each(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    let mut func: strm_value = 0;
    let mut i: strm_int = 0;
    let mut r: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"av\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut strm_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < len {
        if strm_funcall(strm, func, 1 as libc::c_int, &mut *v.offset(i as isize), &mut r)
            == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    *ret = *args.offset(0 as libc::c_int as isize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_map(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = (*strm).data as *mut map_data;
    let mut val: strm_value = 0;
    if strm_funcall(strm, (*d).func, 1 as libc::c_int, &mut data, &mut val)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    strm_emit(strm, val, None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_map(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = 0 as *mut map_data;
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
    d = malloc(::std::mem::size_of::<map_data>() as libc::c_ulong) as *mut map_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).func = func;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_map
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_map(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    let mut func: strm_value = 0;
    let mut i: strm_int = 0;
    let mut a2: strm_array = 0;
    let mut v2: *mut strm_value = 0 as *mut strm_value;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"av\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut strm_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    a2 = strm_ary_new(0 as *const strm_value, len);
    v2 = (*strm_ary_struct(a2)).ptr;
    i = 0 as libc::c_int;
    while i < len {
        if strm_funcall(
            strm,
            func,
            1 as libc::c_int,
            &mut *v.offset(i as isize),
            &mut *v2.offset(i as isize),
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    *ret = a2;
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_flatmap(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = (*strm).data as *mut map_data;
    let mut val: strm_value = 0;
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    let mut e: *mut strm_value = 0 as *mut strm_value;
    if strm_funcall(strm, (*d).func, 1 as libc::c_int, &mut data, &mut val)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_array_p(val) == 0 {
        strm_raise(
            strm,
            b"no array given for flatmap\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    len = (*strm_ary_struct(val)).len;
    e = (*strm_ary_struct(val)).ptr;
    i = 0 as libc::c_int;
    while i < len {
        strm_emit(strm, *e.offset(i as isize), None);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn flatmap_len(mut ary: strm_array) -> libc::c_int {
    let mut v: *mut strm_value = (*strm_ary_struct(ary)).ptr;
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    let mut n: strm_int = 0 as libc::c_int;
    len = (*strm_ary_struct(ary)).len;
    i = 0 as libc::c_int;
    while i < len {
        if strm_array_p(*v.offset(i as isize)) != 0 {
            n += flatmap_len(*v.offset(i as isize));
        } else {
            n += 1;
            n;
        }
        i += 1;
        i;
    }
    return n;
}
unsafe extern "C" fn flatmap_push(
    mut strm: *mut strm_stream,
    mut ary: strm_array,
    mut func: strm_value,
    mut p: *mut *mut strm_value,
) -> libc::c_int {
    let mut v: *mut strm_value = (*strm_ary_struct(ary)).ptr;
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    len = (*strm_ary_struct(ary)).len;
    i = 0 as libc::c_int;
    while i < len {
        if strm_array_p(*v.offset(i as isize)) != 0 {
            if flatmap_push(strm, *v.offset(i as isize), func, p) == 1 as libc::c_int {
                return 1 as libc::c_int;
            }
        } else {
            if strm_funcall(strm, func, 1 as libc::c_int, &mut *v.offset(i as isize), *p)
                == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
            *p = (*p).offset(1 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_flatmap(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = 0 as *mut map_data;
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
    d = malloc(::std::mem::size_of::<map_data>() as libc::c_ulong) as *mut map_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).func = func;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_flatmap
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_flatmap(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut ary: strm_array = 0;
    let mut func: strm_value = 0;
    let mut a2: strm_array = 0;
    let mut v2: *mut strm_value = 0 as *mut strm_value;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"Av\0" as *const u8 as *const libc::c_char,
        &mut ary as *mut strm_array,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    a2 = strm_ary_new(0 as *const strm_value, flatmap_len(ary));
    v2 = (*strm_ary_struct(a2)).ptr;
    if flatmap_push(strm, ary, func, &mut v2) == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *ret = a2;
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_filter(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = (*strm).data as *mut map_data;
    let mut val: strm_value = 0;
    if strm_funcall(strm, (*d).func, 1 as libc::c_int, &mut data, &mut val)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_value_bool(val) != 0 {
        strm_emit(strm, data, None);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_filter(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut map_data = malloc(::std::mem::size_of::<map_data>() as libc::c_ulong)
        as *mut map_data;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut (*d).func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_filter
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_count(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut count_data = (*strm).data as *mut count_data;
    (*d).count += 1;
    (*d).count;
    return 0 as libc::c_int;
}
unsafe extern "C" fn count_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut count_data = (*strm).data as *mut count_data;
    strm_emit(strm, strm_int_value((*d).count), None);
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_count(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut count_data = 0 as *mut count_data;
    if strm_parse_args(strm, argc, args, b"\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<count_data>() as libc::c_ulong) as *mut count_data;
    (*d).count = 0 as libc::c_int;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_count
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                count_finish
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_minmax(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut minmax_data = (*strm).data as *mut minmax_data;
    let mut e: strm_value = 0;
    let mut num: libc::c_double = 0.;
    if strm_nil_p((*d).func) == 0 {
        if strm_funcall(strm, (*d).func, 1 as libc::c_int, &mut data, &mut e)
            == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    } else {
        e = data;
    }
    num = strm_value_float(e);
    if (*d).start != 0 {
        (*d).start = 0 as libc::c_int;
        (*d).num = num;
        (*d).data = data;
    } else if (*d).min != 0 {
        if (*d).num > num {
            (*d).num = num;
            (*d).data = data;
        }
    } else if (*d).num < num {
        (*d).num = num;
        (*d).data = data;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn minmax_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut minmax_data = (*strm).data as *mut minmax_data;
    strm_emit(strm, (*d).data, None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_minmax(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut min: libc::c_int,
) -> libc::c_int {
    let mut d: *mut minmax_data = 0 as *mut minmax_data;
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
    d = malloc(::std::mem::size_of::<minmax_data>() as libc::c_ulong)
        as *mut minmax_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).start = 1 as libc::c_int;
    (*d).min = min;
    (*d).num = 0 as libc::c_int as libc::c_double;
    (*d).data = strm_nil_value();
    (*d).func = func;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_minmax
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                minmax_finish
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_min(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_minmax(strm, argc, args, ret, 1 as libc::c_int);
}
unsafe extern "C" fn exec_max(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_minmax(strm, argc, args, ret, 0 as libc::c_int);
}
unsafe extern "C" fn iter_reduce(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut reduce_data = (*strm).data as *mut reduce_data;
    let mut args: [strm_value; 2] = [0; 2];
    if (*d).init == 0 {
        (*d).init = 1 as libc::c_int;
        (*d).acc = data;
        return 0 as libc::c_int;
    }
    args[0 as libc::c_int as usize] = (*d).acc;
    args[1 as libc::c_int as usize] = data;
    if strm_funcall(strm, (*d).func, 2 as libc::c_int, args.as_mut_ptr(), &mut data)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    (*d).acc = data;
    return 0 as libc::c_int;
}
unsafe extern "C" fn reduce_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut reduce_data = (*strm).data as *mut reduce_data;
    if (*d).init == 0 {
        return 1 as libc::c_int;
    }
    strm_emit(strm, (*d).acc, None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_reduce(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut reduce_data = 0 as *mut reduce_data;
    let mut v1: strm_value = 0;
    let mut v2: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v|v\0" as *const u8 as *const libc::c_char,
        &mut v1 as *mut strm_value,
        &mut v2 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<reduce_data>() as libc::c_ulong)
        as *mut reduce_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    if argc == 2 as libc::c_int {
        (*d).init = 1 as libc::c_int;
        (*d).acc = v1;
        (*d).func = v2;
    } else {
        (*d).init = 0 as libc::c_int;
        (*d).acc = strm_nil_value();
        (*d).func = v1;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_reduce
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                reduce_finish
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_resize_rbk(
    mut h: *mut kh_rbk_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * khash_ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = malloc(
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut khint64_t = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<khint64_t>() as libc::c_ulong),
            ) as *mut khint64_t;
            if new_keys.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut strm_value = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
            ) as *mut strm_value;
            if new_vals.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: khint64_t = *((*h).keys).offset(j as isize);
                let mut val: strm_value = 0;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh0 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh0 = (*fresh0 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int)
                        as khint32_t;
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh1 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh1 = (*fresh1 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: khint64_t = *((*h).keys).offset(i as isize);
                        *((*h).keys).offset(i as isize) = key;
                        key = tmp;
                        let mut tmp_0: strm_value = *((*h).vals).offset(i as isize);
                        *((*h).vals).offset(i as isize) = val;
                        val = tmp_0;
                        let ref mut fresh2 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh2 = (*fresh2 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<khint64_t>() as libc::c_ulong),
            ) as *mut khint64_t;
            (*h)
                .vals = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
            ) as *mut strm_value;
        }
        free((*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * khash_ac_HASH_UPPER
            + 0.5f64) as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_put_rbk(
    mut h: *mut kh_rbk_t,
    mut key: khint64_t,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_rbk(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_rbk(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh3 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh3 = (*fresh3 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh4 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh4 = (*fresh4 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_init_rbk() -> *mut kh_rbk_t {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kh_rbk_t>() as libc::c_ulong,
    ) as *mut kh_rbk_t;
}
unsafe extern "C" fn iter_rbk(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut rbk_data = (*strm).data as *mut rbk_data;
    let mut k: strm_value = 0;
    let mut v: strm_value = 0;
    let mut i: khiter_t = 0;
    let mut r: libc::c_int = 0;
    if strm_array_p(data) == 0 || (*strm_ary_struct(data)).len != 2 as libc::c_int {
        strm_raise(
            strm,
            b"reduce_by_key element must be a key-value pair\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    k = *((*strm_ary_struct(data)).ptr).offset(0 as libc::c_int as isize);
    v = *((*strm_ary_struct(data)).ptr).offset(1 as libc::c_int as isize);
    i = kh_put_rbk((*d).tbl, k, &mut r);
    if r < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if r != 0 as libc::c_int {
        *((*(*d).tbl).vals).offset(i as isize) = v;
    } else {
        let mut args: [strm_value; 3] = [0; 3];
        args[0 as libc::c_int as usize] = k;
        args[1 as libc::c_int as usize] = *((*(*d).tbl).vals).offset(i as isize);
        args[2 as libc::c_int as usize] = v;
        if strm_funcall(strm, (*d).func, 3 as libc::c_int, args.as_mut_ptr(), &mut v)
            == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        *((*(*d).tbl).vals).offset(i as isize) = v;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn rbk_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut rbk_data = (*strm).data as *mut rbk_data;
    let mut i: khiter_t = 0;
    i = 0 as libc::c_int as khint_t;
    while i != (*(*d).tbl).n_buckets {
        if *((*(*d).tbl).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint == 0
        {
            let mut values: [strm_value; 2] = [0; 2];
            values[0 as libc::c_int as usize] = *((*(*d).tbl).keys).offset(i as isize);
            values[1 as libc::c_int as usize] = *((*(*d).tbl).vals).offset(i as isize);
            strm_emit(strm, strm_ary_new(values.as_mut_ptr(), 2 as libc::c_int), None);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_rbk(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut rbk_data = 0 as *mut rbk_data;
    let mut t: *mut kh_rbk_t = 0 as *mut kh_rbk_t;
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
    t = kh_init_rbk();
    if t.is_null() {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<rbk_data>() as libc::c_ulong) as *mut rbk_data;
    (*d).tbl = t;
    (*d).func = func;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_rbk
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                rbk_finish
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_slice(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut slice_data = (*strm).data as *mut slice_data;
    let mut n: strm_int = (*d).n;
    let fresh5 = (*d).i;
    (*d).i = (*d).i + 1;
    *((*d).buf).offset(fresh5 as isize) = data;
    if (*d).i == n {
        let mut ary: strm_array = strm_ary_new((*d).buf, n);
        (*d).i = 0 as libc::c_int;
        strm_emit(strm, ary, None);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn finish_slice(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut slice_data = (*strm).data as *mut slice_data;
    if (*d).i > 0 as libc::c_int {
        let mut ary: strm_array = strm_ary_new((*d).buf, (*d).i);
        strm_emit(strm, ary, None);
    }
    free((*d).buf as *mut libc::c_void);
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_slice(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut slice_data = 0 as *mut slice_data;
    let mut n: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"i\0" as *const u8 as *const libc::c_char,
        &mut n as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<slice_data>() as libc::c_ulong) as *mut slice_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).n = n;
    (*d).i = 0 as libc::c_int;
    (*d)
        .buf = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
    ) as *mut strm_value;
    if ((*d).buf).is_null() {
        free(d as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_slice
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                finish_slice
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_consec(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut slice_data = (*strm).data as *mut slice_data;
    let mut n: strm_int = (*d).n;
    if (*d).i < n {
        let fresh6 = (*d).i;
        (*d).i = (*d).i + 1;
        *((*d).buf).offset(fresh6 as isize) = data;
        if (*d).i == n {
            let mut ary: strm_array = strm_ary_new((*d).buf, n);
            strm_emit(strm, ary, None);
        }
        return 0 as libc::c_int;
    } else {
        let mut ary_0: strm_array = 0;
        let mut i: strm_int = 0;
        let mut len: strm_int = n - 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < len {
            *((*d).buf)
                .offset(
                    i as isize,
                ) = *((*d).buf).offset((i + 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
        *((*d).buf).offset(len as isize) = data;
        ary_0 = strm_ary_new((*d).buf, n);
        strm_emit(strm, ary_0, None);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn finish_consec(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut slice_data = (*strm).data as *mut slice_data;
    free((*d).buf as *mut libc::c_void);
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_consec(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut slice_data = 0 as *mut slice_data;
    let mut n: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"i\0" as *const u8 as *const libc::c_char,
        &mut n as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<slice_data>() as libc::c_ulong) as *mut slice_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).n = n;
    (*d).i = 0 as libc::c_int;
    (*d)
        .buf = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<strm_value>() as libc::c_ulong),
    ) as *mut strm_value;
    if ((*d).buf).is_null() {
        free(d as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_consec
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                finish_consec
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_take(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut take_data = (*strm).data as *mut take_data;
    strm_emit(strm, data, None);
    (*d).n -= 1;
    (*d).n;
    if (*d).n == 0 as libc::c_int {
        strm_stream_close(strm);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_take(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut take_data = 0 as *mut take_data;
    let mut n: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"i\0" as *const u8 as *const libc::c_char,
        &mut n as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if n < 0 as libc::c_int {
        strm_raise(strm, b"negative iteration\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<take_data>() as libc::c_ulong) as *mut take_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).n = n;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_take
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_drop(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut take_data = (*strm).data as *mut take_data;
    if (*d).n > 0 as libc::c_int {
        (*d).n -= 1;
        (*d).n;
        return 0 as libc::c_int;
    }
    strm_emit(strm, data, None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_drop(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut take_data = 0 as *mut take_data;
    let mut n: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"i\0" as *const u8 as *const libc::c_char,
        &mut n as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if n < 0 as libc::c_int {
        strm_raise(strm, b"negative iteration\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<take_data>() as libc::c_ulong) as *mut take_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).n = n;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_drop
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_uniq(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut uniq_data = (*strm).data as *mut uniq_data;
    if (*d).init == 0 {
        (*d).init = 1 as libc::c_int;
        (*d).last = data;
        strm_emit(strm, data, None);
        return 0 as libc::c_int;
    }
    if strm_value_eq(data, (*d).last) == 0 {
        (*d).last = data;
        strm_emit(strm, data, None);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_uniqf(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut uniq_data = (*strm).data as *mut uniq_data;
    let mut val: strm_value = 0;
    if strm_funcall(strm, (*d).func, 1 as libc::c_int, &mut data, &mut val)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*d).init == 0 {
        (*d).init = 1 as libc::c_int;
        (*d).last = data;
        (*d).v = val;
        strm_emit(strm, data, None);
        return 0 as libc::c_int;
    }
    if strm_value_eq(val, (*d).v) == 0 {
        (*d).last = data;
        (*d).v = val;
        strm_emit(strm, data, None);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_uniq(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut uniq_data = 0 as *mut uniq_data;
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
    d = malloc(::std::mem::size_of::<uniq_data>() as libc::c_ulong) as *mut uniq_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).last = strm_nil_value();
    (*d).func = func;
    (*d).init = 0 as libc::c_int;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            if strm_nil_p(func) != 0 {
                Some(
                    iter_uniq
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                )
            } else {
                Some(
                    iter_uniqf
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                )
            },
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_iter_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"seq\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_seq
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
        b"repeat\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_repeat
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
        b"cycle\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_cycle
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
        b"each\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_each
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
        b"map\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_map
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
        b"flatmap\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_flatmap
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
        b"filter\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_filter
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
        b"count\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_count
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
        b"min\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_min
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
        b"max\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_max
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
        b"reduce\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_reduce
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
        b"reduce_by_key\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_rbk
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
        b"slice\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_slice
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
        b"consec\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_consec
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
        b"take\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_take
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
        b"drop\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_drop
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
        b"uniq\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_uniq
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
        b"each\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_each
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
        b"map\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_map
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
        b"flatmap\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_flatmap
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_stat_init(state);
}
