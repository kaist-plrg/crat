use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type strm_queue;
    pub type node_error;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_nil_value() -> strm_value;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_number_p(_: strm_value) -> libc::c_int;
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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn strm_rand_init(state: *mut strm_state);
    fn strm_sort_init(state: *mut strm_state);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sum_data {
    pub sum: libc::c_double,
    pub c: libc::c_double,
    pub num: strm_int,
    pub func: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvavg_data {
    pub num: strm_int,
    pub i: strm_int,
    pub filled: strm_int,
    pub func: strm_value,
    pub func_p: strm_int,
    pub data: [libc::c_double; 0],
}
pub type stdev_mode = libc::c_uint;
pub const mode_mean_variance: stdev_mode = 3;
pub const mode_mean_stdev: stdev_mode = 2;
pub const mode_variance: stdev_mode = 1;
pub const mode_stdev: stdev_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stdev_data {
    pub num: strm_int,
    pub s1: libc::c_double,
    pub s2: libc::c_double,
    pub func: strm_value,
    pub mode: stdev_mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct correl_data {
    pub n: strm_int,
    pub sx: libc::c_double,
    pub sy: libc::c_double,
    pub sxx: libc::c_double,
    pub syy: libc::c_double,
    pub sxy: libc::c_double,
}
unsafe extern "C" fn iter_sum(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sum_data = (*strm).data as *mut sum_data;
    let mut x: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    if strm_number_p(data) == 0 {
        return 1 as libc::c_int;
    }
    x = strm_value_float(data);
    t = (*d).sum + x;
    if fabs((*d).sum) >= fabs(x) {
        (*d).c += (*d).sum - t + x;
    } else {
        (*d).c += x - t + (*d).sum;
    }
    (*d).sum = t;
    (*d).num += 1;
    (*d).num;
    return 0 as libc::c_int;
}
unsafe extern "C" fn convert_number(
    mut strm: *mut strm_stream,
    mut data: strm_value,
    mut func: strm_value,
) -> strm_value {
    let mut val: strm_value = 0;
    if strm_funcall(strm, func, 1 as libc::c_int, &mut data, &mut val)
        == 1 as libc::c_int
    {
        return strm_nil_value();
    }
    if strm_number_p(val) == 0 {
        strm_raise(strm, b"number required\0" as *const u8 as *const libc::c_char);
        return strm_nil_value();
    }
    return val;
}
unsafe extern "C" fn iter_sumf(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sum_data = (*strm).data as *mut sum_data;
    let mut x: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    data = convert_number(strm, data, (*d).func);
    if strm_number_p(data) == 0 {
        return 1 as libc::c_int;
    }
    x = strm_value_float(data);
    t = (*d).sum + x;
    if fabs((*d).sum) >= fabs(x) {
        (*d).c += (*d).sum - t + x;
    } else {
        (*d).c += x - t + (*d).sum;
    }
    (*d).sum = t;
    (*d).num += 1;
    (*d).num;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sum_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sum_data = (*strm).data as *mut sum_data;
    strm_emit(strm, strm_float_value((*d).sum + (*d).c), None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn avg_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut sum_data = (*strm).data as *mut sum_data;
    strm_emit(
        strm,
        strm_float_value(((*d).sum + (*d).c) / (*d).num as libc::c_double),
        None,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_sum_avg(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut avg: libc::c_int,
) -> libc::c_int {
    let mut d: *mut sum_data = 0 as *mut sum_data;
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
    d = malloc(::std::mem::size_of::<sum_data>() as libc::c_ulong) as *mut sum_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).sum = 0 as libc::c_int as libc::c_double;
    (*d).c = 0 as libc::c_int as libc::c_double;
    (*d).num = 0 as libc::c_int;
    if argc == 0 as libc::c_int {
        (*d).func = strm_nil_value();
        *ret = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    iter_sum
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                if avg != 0 {
                    Some(
                        avg_finish
                            as unsafe extern "C" fn(
                                *mut strm_stream,
                                strm_value,
                            ) -> libc::c_int,
                    )
                } else {
                    Some(
                        sum_finish
                            as unsafe extern "C" fn(
                                *mut strm_stream,
                                strm_value,
                            ) -> libc::c_int,
                    )
                },
                d as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    } else {
        (*d).func = func;
        *ret = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    iter_sumf
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                if avg != 0 {
                    Some(
                        avg_finish
                            as unsafe extern "C" fn(
                                *mut strm_stream,
                                strm_value,
                            ) -> libc::c_int,
                    )
                } else {
                    Some(
                        sum_finish
                            as unsafe extern "C" fn(
                                *mut strm_stream,
                                strm_value,
                            ) -> libc::c_int,
                    )
                },
                d as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_sum(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_sum_avg(strm, argc, args, ret, 0 as libc::c_int);
}
unsafe extern "C" fn exec_avg(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_sum_avg(strm, argc, args, ret, 1 as libc::c_int);
}
unsafe extern "C" fn ary_sum_avg(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut avg: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut c: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut func: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a|v\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut libc::c_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < len {
            let mut x: libc::c_double = 0.;
            let mut t: libc::c_double = 0.;
            if strm_number_p(*v.offset(i as isize)) == 0 {
                return 1 as libc::c_int;
            }
            x = strm_value_float(*v.offset(i as isize));
            t = sum + x;
            if fabs(sum) >= fabs(x) {
                c += sum - t + x;
            } else {
                c += x - t + sum;
            }
            sum = t;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < len {
            let mut val: strm_value = 0;
            let mut x_0: libc::c_double = 0.;
            let mut t_0: libc::c_double = 0.;
            val = convert_number(strm, *v.offset(i as isize), func);
            if strm_number_p(val) == 0 {
                return 1 as libc::c_int;
            }
            x_0 = strm_value_float(val);
            t_0 = sum + x_0;
            if fabs(sum) >= fabs(x_0) {
                c += sum - t_0 + x_0;
            } else {
                c += x_0 - t_0 + sum;
            }
            sum = t_0;
            i += 1;
            i;
        }
    }
    sum += c;
    if avg != 0 {
        *ret = strm_float_value(sum / len as libc::c_double);
    } else {
        *ret = strm_float_value(sum);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_sum(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return ary_sum_avg(strm, argc, args, ret, 0 as libc::c_int);
}
unsafe extern "C" fn ary_avg(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return ary_sum_avg(strm, argc, args, ret, 1 as libc::c_int);
}
unsafe extern "C" fn iter_mvavg(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut mvavg_data = (*strm).data as *mut mvavg_data;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut c: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: strm_int = 0;
    let mut len: strm_int = (*d).num;
    if (*d).func_p != 0 {
        data = convert_number(strm, data, (*d).func);
    }
    if strm_number_p(data) == 0 {
        strm_raise(strm, b"number required\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let fresh0 = (*d).i;
    (*d).i = (*d).i + 1;
    *((*d).data).as_mut_ptr().offset(fresh0 as isize) = strm_value_float(data);
    if (*d).i == (*d).num {
        (*d).filled = 1 as libc::c_int;
        (*d).i = 0 as libc::c_int;
    }
    if (*d).filled == 0 {
        strm_emit(strm, strm_nil_value(), None);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < len {
        let mut x: libc::c_double = *((*d).data).as_mut_ptr().offset(i as isize);
        let mut t: libc::c_double = sum + x;
        if fabs(sum) >= fabs(x) {
            c += sum - t + x;
        } else {
            c += x - t + sum;
        }
        sum = t;
        i += 1;
        i;
    }
    strm_emit(strm, strm_float_value((sum + c) / (*d).num as libc::c_double), None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_mvavg(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut mvavg_data = 0 as *mut mvavg_data;
    let mut n: strm_int = 0;
    let mut func: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"i|v\0" as *const u8 as *const libc::c_char,
        &mut n as *mut strm_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(
        (::std::mem::size_of::<mvavg_data>() as libc::c_ulong)
            .wrapping_add(
                (n as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ),
    ) as *mut mvavg_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).num = n;
    (*d).i = 0 as libc::c_int;
    (*d).filled = 0 as libc::c_int;
    if argc == 1 as libc::c_int {
        (*d).func = strm_nil_value();
        (*d).func_p = 0 as libc::c_int;
    } else {
        (*d).func = func;
        (*d).func_p = 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_mvavg
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_stdev(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut stdev_data = (*strm).data as *mut stdev_data;
    let mut x: libc::c_double = strm_value_float(data);
    (*d).num += 1;
    (*d).num;
    x -= (*d).s1;
    (*d).s1 += x / (*d).num as libc::c_double;
    (*d).s2
        += ((*d).num - 1 as libc::c_int) as libc::c_double * x * x
            / (*d).num as libc::c_double;
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_stdevf(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut stdev_data = (*strm).data as *mut stdev_data;
    let mut x: libc::c_double = 0.;
    data = convert_number(strm, data, (*d).func);
    if strm_number_p(data) == 0 {
        return 1 as libc::c_int;
    }
    x = strm_value_float(data);
    (*d).num += 1;
    (*d).num;
    x -= (*d).s1;
    (*d).s1 += x / (*d).num as libc::c_double;
    (*d).s2
        += ((*d).num - 1 as libc::c_int) as libc::c_double * x * x
            / (*d).num as libc::c_double;
    return 0 as libc::c_int;
}
unsafe extern "C" fn float2(mut m: libc::c_double, mut s: libc::c_double) -> strm_value {
    let mut buf: [strm_value; 2] = [0; 2];
    fprintf(stderr, b"f2[%f, %f]\n\0" as *const u8 as *const libc::c_char, m, s);
    buf[0 as libc::c_int as usize] = strm_float_value(m);
    buf[1 as libc::c_int as usize] = strm_float_value(s);
    return strm_ary_new(buf.as_mut_ptr(), 2 as libc::c_int);
}
unsafe extern "C" fn stdev_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut stdev_data = (*strm).data as *mut stdev_data;
    let mut s: libc::c_double = 0.;
    match (*d).mode as libc::c_uint {
        0 => {
            s = sqrt((*d).s2 / ((*d).num - 1 as libc::c_int) as libc::c_double);
            strm_emit(strm, strm_float_value(s), None);
        }
        1 => {
            s = (*d).s2 / ((*d).num - 1 as libc::c_int) as libc::c_double;
            strm_emit(strm, strm_float_value(s), None);
        }
        2 => {
            s = sqrt((*d).s2 / ((*d).num - 1 as libc::c_int) as libc::c_double);
            strm_emit(strm, float2((*d).s1, s), None);
        }
        3 => {
            s = (*d).s2 / ((*d).num - 1 as libc::c_int) as libc::c_double;
            strm_emit(strm, float2((*d).s1, s), None);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_var_stdev(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut mode: stdev_mode,
) -> libc::c_int {
    let mut d: *mut stdev_data = 0 as *mut stdev_data;
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
    d = malloc(::std::mem::size_of::<stdev_data>() as libc::c_ulong) as *mut stdev_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).num = 0 as libc::c_int;
    (*d).s2 = 0.0f64;
    (*d).s1 = (*d).s2;
    (*d).mode = mode;
    if argc == 0 as libc::c_int {
        *ret = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    iter_stdev
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                Some(
                    stdev_finish
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                d as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    } else {
        (*d).func = func;
        *ret = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    iter_stdevf
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                Some(
                    stdev_finish
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                d as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_stdev(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_var_stdev(strm, argc, args, ret, mode_stdev);
}
unsafe extern "C" fn exec_variance(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_var_stdev(strm, argc, args, ret, mode_variance);
}
unsafe extern "C" fn exec_mean_stdev(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_var_stdev(strm, argc, args, ret, mode_mean_stdev);
}
unsafe extern "C" fn exec_mean_variance(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_var_stdev(strm, argc, args, ret, mode_mean_variance);
}
unsafe extern "C" fn ary_var_stdev(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut stdev: libc::c_int,
) -> libc::c_int {
    let mut func: strm_value = 0;
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a|v\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut libc::c_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    s2 = 0.0f64;
    s1 = s2;
    if argc == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < len {
            let mut x: libc::c_double = strm_value_float(*v.offset(i as isize));
            x -= s1;
            s1 += x / (i + 1 as libc::c_int) as libc::c_double;
            s2 += i as libc::c_double * x * x / (i + 1 as libc::c_int) as libc::c_double;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < len {
            let mut val: strm_value = 0;
            let mut x_0: libc::c_double = 0.;
            val = convert_number(strm, *v.offset(i as isize), func);
            if strm_number_p(val) == 0 {
                return 1 as libc::c_int;
            }
            x_0 = strm_value_float(val);
            x_0 -= s1;
            s1 += x_0 / (i + 1 as libc::c_int) as libc::c_double;
            s2
                += i as libc::c_double * x_0 * x_0
                    / (i + 1 as libc::c_int) as libc::c_double;
            i += 1;
            i;
        }
    }
    s2 = s2 / (i - 1 as libc::c_int) as libc::c_double;
    if stdev != 0 {
        s2 = sqrt(s2);
    }
    *ret = strm_float_value(s2);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_stdev(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return ary_var_stdev(strm, argc, args, ret, 1 as libc::c_int);
}
unsafe extern "C" fn ary_var(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return ary_var_stdev(strm, argc, args, ret, 0 as libc::c_int);
}
unsafe extern "C" fn iter_correl(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut correl_data = (*strm).data as *mut correl_data;
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    if strm_array_p(data) == 0 || (*strm_ary_struct(data)).len != 2 as libc::c_int {
        strm_raise(strm, b"invalid data\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    v = (*strm_ary_struct(data)).ptr;
    if strm_number_p(*v.offset(0 as libc::c_int as isize)) == 0
        || strm_number_p(*v.offset(1 as libc::c_int as isize)) == 0
    {
        strm_raise(
            strm,
            b"correl() requires [num, num]\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*d).n += 1;
    (*d).n;
    dx = strm_value_float(*v.offset(0 as libc::c_int as isize)) - (*d).sx;
    (*d).sx += dx / (*d).n as libc::c_double;
    dy = strm_value_float(*v.offset(1 as libc::c_int as isize)) - (*d).sy;
    (*d).sy += dy / (*d).n as libc::c_double;
    (*d).sxx
        += ((*d).n - 1 as libc::c_int) as libc::c_double * dx * dx
            / (*d).n as libc::c_double;
    (*d).syy
        += ((*d).n - 1 as libc::c_int) as libc::c_double * dy * dy
            / (*d).n as libc::c_double;
    (*d).sxy
        += ((*d).n - 1 as libc::c_int) as libc::c_double * dx * dy
            / (*d).n as libc::c_double;
    return 0 as libc::c_int;
}
unsafe extern "C" fn correl_finish(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut correl_data = (*strm).data as *mut correl_data;
    (*d).n -= 1;
    (*d).n;
    let mut sxx: libc::c_double = sqrt((*d).sxx / (*d).n as libc::c_double);
    let mut syy: libc::c_double = sqrt((*d).syy / (*d).n as libc::c_double);
    let mut sxy: libc::c_double = (*d).sxy / ((*d).n as libc::c_double * sxx * syy);
    strm_emit(strm, strm_float_value(sxy), None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_correl(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut correl_data = 0 as *mut correl_data;
    if strm_parse_args(strm, argc, args, b"\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<correl_data>() as libc::c_ulong)
        as *mut correl_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).n = 0 as libc::c_int;
    (*d).sxy = 0 as libc::c_int as libc::c_double;
    (*d).syy = (*d).sxy;
    (*d).sxx = (*d).syy;
    (*d).sy = (*d).sxx;
    (*d).sx = (*d).sy;
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_filter,
            Some(
                iter_correl
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                correl_finish
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_correl(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut sx: libc::c_double = 0.;
    let mut sy: libc::c_double = 0.;
    let mut sxx: libc::c_double = 0.;
    let mut syy: libc::c_double = 0.;
    let mut sxy: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    sxy = 0 as libc::c_int as libc::c_double;
    syy = sxy;
    sxx = syy;
    sy = sxx;
    sx = sy;
    i = 0 as libc::c_int;
    while i < len {
        let mut data: strm_value = *v.offset(i as isize);
        let mut dv: *mut strm_value = 0 as *mut strm_value;
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        if !(strm_array_p(data) == 0 || (*strm_ary_struct(data)).len != 2 as libc::c_int)
        {
            dv = (*strm_ary_struct(data)).ptr;
            dx = strm_value_float(*dv.offset(0 as libc::c_int as isize)) - sx;
            sx += dx / (i + 1 as libc::c_int) as libc::c_double;
            dy = strm_value_float(*dv.offset(1 as libc::c_int as isize)) - sy;
            sy += dy / (i + 1 as libc::c_int) as libc::c_double;
            sxx
                += i as libc::c_double * dx * dx
                    / (i + 1 as libc::c_int) as libc::c_double;
            syy
                += i as libc::c_double * dy * dy
                    / (i + 1 as libc::c_int) as libc::c_double;
            sxy
                += i as libc::c_double * dx * dy
                    / (i + 1 as libc::c_int) as libc::c_double;
        }
        i += 1;
        i;
    }
    sxx = sqrt(sxx / (len - 1 as libc::c_int) as libc::c_double);
    syy = sqrt(syy / (len - 1 as libc::c_int) as libc::c_double);
    sxy /= (len - 1 as libc::c_int) as libc::c_double * sxx * syy;
    *ret = strm_float_value(sxy);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_stat_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"sum\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_sum
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
        b"average\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_avg
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
        b"mean\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_avg
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
        b"moving_average\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_mvavg
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
        b"rolling_mean\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_mvavg
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
        b"stdev\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_stdev
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
        b"variance\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_variance
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
        b"mean_stdev\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_mean_stdev
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
        b"mean_variance\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_mean_variance
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
        b"correl\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_correl
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
        b"sum\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_sum
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
        b"average\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_avg
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
        b"stdev\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_stdev
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
        b"variance\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_var
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
        b"correl\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_correl
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_rand_init(state);
    strm_sort_init(state);
}
