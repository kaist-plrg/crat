use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
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
    fn strm_connect(
        strm: *mut strm_stream,
        src: strm_value,
        dst: strm_value,
        ret: *mut strm_value,
    ) -> libc::c_int;
    fn strm_stream_close(strm: *mut strm_stream);
    fn strm_queue_new() -> *mut strm_queue;
    fn strm_queue_add(queue: *mut strm_queue, val: *mut libc::c_void) -> libc::c_int;
    fn strm_queue_get(queue: *mut strm_queue) -> *mut libc::c_void;
    fn strm_queue_empty_p(queue: *mut strm_queue) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recv_data {
    pub strm: *mut strm_stream,
    pub func: strm_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latch_data {
    pub dq: *mut strm_queue,
    pub rq: *mut strm_queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_data {
    pub a: strm_array,
    pub i: strm_int,
    pub len: strm_int,
    pub latch: [*mut strm_stream; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct concat_data {
    pub i: strm_int,
    pub len: strm_int,
    pub latch: [*mut strm_stream; 0],
}
pub unsafe extern "C" fn strm_latch_finish_p(
    mut latch: *mut strm_stream,
) -> libc::c_int {
    let mut c: *mut latch_data = 0 as *mut latch_data;
    if (*latch).mode as libc::c_uint == strm_consumer as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    c = (*latch).data as *mut latch_data;
    return strm_queue_empty_p((*c).dq);
}
unsafe extern "C" fn latch_push(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut latch_data = (*strm).data as *mut latch_data;
    let mut r: *mut recv_data = strm_queue_get((*d).rq) as *mut recv_data;
    if (*strm).mode as libc::c_uint != strm_consumer as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    if !r.is_null() {
        (Some(((*r).func).unwrap())).unwrap()((*r).strm, data);
        free(r as *mut libc::c_void);
    } else {
        let mut v: *mut strm_value = malloc(
            ::std::mem::size_of::<strm_value>() as libc::c_ulong,
        ) as *mut strm_value;
        *v = data;
        strm_queue_add((*d).dq, v as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_latch_receive(
    mut latch: *mut strm_stream,
    mut strm: *mut strm_stream,
    mut func: strm_callback,
) {
    let mut d: *mut latch_data = 0 as *mut latch_data;
    let mut v: *mut strm_value = 0 as *mut strm_value;
    if (*latch).start_func
        == Some(
            latch_push
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        )
    {} else {
        __assert_fail(
            b"latch->start_func == latch_push\0" as *const u8 as *const libc::c_char,
            b"latch.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void strm_latch_receive(strm_stream *, strm_stream *, strm_callback)\0"))
                .as_ptr(),
        );
    }
    'c_3036: {
        if (*latch).start_func
            == Some(
                latch_push
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            )
        {} else {
            __assert_fail(
                b"latch->start_func == latch_push\0" as *const u8 as *const libc::c_char,
                b"latch.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void strm_latch_receive(strm_stream *, strm_stream *, strm_callback)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    d = (*latch).data as *mut latch_data;
    v = strm_queue_get((*d).dq) as *mut strm_value;
    if !v.is_null() {
        (Some(func.unwrap())).unwrap()(strm, *v);
        free(v as *mut libc::c_void);
    } else {
        let mut r: *mut recv_data = malloc(
            ::std::mem::size_of::<recv_data>() as libc::c_ulong,
        ) as *mut recv_data;
        (*r).strm = strm;
        (*r).func = func;
        strm_queue_add((*d).rq, r as *mut libc::c_void);
    };
}
unsafe extern "C" fn latch_close(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut latch_data = (*strm).data as *mut latch_data;
    loop {
        let mut r: *mut recv_data = strm_queue_get((*d).rq) as *mut recv_data;
        if r.is_null() {
            break;
        }
        (Some(((*r).func).unwrap())).unwrap()((*r).strm, data);
        free(r as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_latch_new() -> *mut strm_stream {
    let mut d: *mut latch_data = malloc(
        ::std::mem::size_of::<latch_data>() as libc::c_ulong,
    ) as *mut latch_data;
    if !d.is_null() {} else {
        __assert_fail(
            b"d != NULL\0" as *const u8 as *const libc::c_char,
            b"latch.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"strm_stream *strm_latch_new()\0"))
                .as_ptr(),
        );
    }
    'c_3178: {
        if !d.is_null() {} else {
            __assert_fail(
                b"d != NULL\0" as *const u8 as *const libc::c_char,
                b"latch.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"strm_stream *strm_latch_new()\0"))
                    .as_ptr(),
            );
        }
    };
    (*d).dq = strm_queue_new();
    (*d).rq = strm_queue_new();
    return strm_stream_new(
        strm_consumer,
        Some(
            latch_push
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        Some(
            latch_close
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        d as *mut libc::c_void,
    );
}
unsafe extern "C" fn zip_iter(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut z: *mut zip_data = (*strm).data as *mut zip_data;
    let fresh0 = (*z).i;
    (*z).i = (*z).i + 1;
    *((*strm_ary_struct((*z).a)).ptr).offset(fresh0 as isize) = data;
    if (*z).i < (*z).len {
        strm_latch_receive(
            *((*z).latch).as_mut_ptr().offset((*z).i as isize),
            strm,
            Some(
                zip_iter
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
        );
    } else {
        let mut i: strm_int = 0;
        let mut done: strm_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*z).len {
            if strm_latch_finish_p(*((*z).latch).as_mut_ptr().offset(i as isize)) != 0 {
                done = 1 as libc::c_int;
                break;
            } else {
                i += 1;
                i;
            }
        }
        if done != 0 {
            strm_emit(strm, (*z).a, None);
            i = 0 as libc::c_int;
            while i < (*z).len {
                strm_stream_close(*((*z).latch).as_mut_ptr().offset(i as isize));
                i += 1;
                i;
            }
            strm_stream_close(strm);
        } else {
            strm_emit(
                strm,
                (*z).a,
                Some(
                    zip_start
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zip_start(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut z: *mut zip_data = (*strm).data as *mut zip_data;
    if !z.is_null() {
        (*z).i = 0 as libc::c_int;
        (*z).a = strm_ary_new(0 as *const strm_value, (*z).len);
        strm_latch_receive(
            *((*z).latch).as_mut_ptr().offset(0 as libc::c_int as isize),
            strm,
            Some(
                zip_iter
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zip_close(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    (*strm).data = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_zip(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut z: *mut zip_data = malloc(
        (::std::mem::size_of::<zip_data>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<*mut strm_stream>() as libc::c_ulong)
                    .wrapping_mul(argc as libc::c_ulong),
            ),
    ) as *mut zip_data;
    let mut i: strm_int = 0;
    let mut s: *mut strm_stream = 0 as *mut strm_stream;
    (*z).i = 0 as libc::c_int;
    (*z).len = argc;
    i = 0 as libc::c_int;
    while i < argc {
        let mut r: strm_value = 0;
        s = strm_latch_new();
        strm_connect(
            strm,
            *args.offset(i as isize),
            strm_ptr_value(s as *mut libc::c_void),
            &mut r,
        );
        let ref mut fresh1 = *((*z).latch).as_mut_ptr().offset(i as isize);
        *fresh1 = s;
        i += 1;
        i;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                zip_start
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                zip_close
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            z as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn concat_iter(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut concat_data = (*strm).data as *mut concat_data;
    strm_emit(strm, data, None);
    if strm_latch_finish_p(*((*d).latch).as_mut_ptr().offset((*d).i as isize)) != 0 {
        strm_stream_close(*((*d).latch).as_mut_ptr().offset((*d).i as isize));
        (*d).i += 1;
        (*d).i;
    }
    if (*d).i < (*d).len {
        strm_latch_receive(
            *((*d).latch).as_mut_ptr().offset((*d).i as isize),
            strm,
            Some(
                concat_iter
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
        );
    } else {
        strm_stream_close(strm);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn concat_start(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut concat_data = (*strm).data as *mut concat_data;
    if !d.is_null() {
        strm_latch_receive(
            *((*d).latch).as_mut_ptr().offset((*d).i as isize),
            strm,
            Some(
                concat_iter
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_concat(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut concat_data = malloc(
        (::std::mem::size_of::<concat_data>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<*mut strm_stream>() as libc::c_ulong)
                    .wrapping_mul(argc as libc::c_ulong),
            ),
    ) as *mut concat_data;
    let mut i: strm_int = 0;
    let mut s: *mut strm_stream = 0 as *mut strm_stream;
    (*d).i = 0 as libc::c_int;
    (*d).len = argc;
    i = 0 as libc::c_int;
    while i < argc {
        let mut r: strm_value = 0;
        s = strm_latch_new();
        strm_connect(
            strm,
            *args.offset(i as isize),
            strm_ptr_value(s as *mut libc::c_void),
            &mut r,
        );
        let ref mut fresh2 = *((*d).latch).as_mut_ptr().offset(i as isize);
        *fresh2 = s;
        i += 1;
        i;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_producer,
            Some(
                concat_start
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            None,
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_latch_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"&\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_zip
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
        b"zip\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_zip
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
        b"concat\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_concat
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
