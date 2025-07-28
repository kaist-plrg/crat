use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut strm_option_verbose: libc::c_int;
    fn strm_nil_value() -> strm_value;
    fn strm_nil_p(_: strm_value) -> libc::c_int;
    fn strm_eprint(_: *mut strm_stream);
    fn strm_queue_new() -> *mut strm_queue;
    fn strm_queue_add(queue_0: *mut strm_queue, val: *mut libc::c_void) -> libc::c_int;
    fn strm_queue_get(queue_0: *mut strm_queue) -> *mut libc::c_void;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn sched_yield() -> libc::c_int;
    fn cpu_count() -> libc::c_int;
    fn strm_init_io_loop();
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type uint64_t = __uint64_t;
pub type strm_value = uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_task {
    pub func: strm_callback,
    pub data: strm_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_worker {
    pub th: pthread_t,
}
pub static mut workers: *mut strm_worker = 0 as *const strm_worker as *mut strm_worker;
static mut queue: *mut strm_queue = 0 as *const strm_queue as *mut strm_queue;
static mut prod_queue: *mut strm_queue = 0 as *const strm_queue as *mut strm_queue;
static mut worker_max: libc::c_int = 0;
static mut stream_count: libc::c_int = 0 as libc::c_int;
pub static mut strm_event_loop_started: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn strm_task_new(
    mut func: strm_callback,
    mut data: strm_value,
) -> *mut strm_task {
    let mut t: *mut strm_task = 0 as *mut strm_task;
    t = malloc(::std::mem::size_of::<strm_task>() as libc::c_ulong) as *mut strm_task;
    (*t).func = func;
    (*t).data = data;
    return t;
}
pub unsafe extern "C" fn strm_task_add(
    mut strm: *mut strm_stream,
    mut task: *mut strm_task,
) {
    strm_queue_add((*strm).queue, task as *mut libc::c_void);
    if (*strm).mode as libc::c_uint == strm_producer as libc::c_int as libc::c_uint {
        strm_queue_add(prod_queue, strm as *mut libc::c_void);
    } else {
        strm_queue_add(queue, strm as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn strm_task_push(
    mut strm: *mut strm_stream,
    mut func: strm_callback,
    mut data: strm_value,
) {
    if (*strm).mode as libc::c_uint == strm_killed as libc::c_int as libc::c_uint
        || (*strm).mode as libc::c_uint == strm_dying as libc::c_int as libc::c_uint
    {
        return;
    }
    strm_task_add(strm, strm_task_new(func, data));
}
pub unsafe extern "C" fn strm_emit(
    mut strm: *mut strm_stream,
    mut data: strm_value,
    mut func: strm_callback,
) {
    if (*strm).mode as libc::c_uint == strm_dying as libc::c_int as libc::c_uint {
        return;
    }
    if strm_nil_p(data) == 0 {
        if !((*strm).dst).is_null() {
            strm_task_push((*strm).dst, (*(*strm).dst).start_func, data);
            if (*(*strm).dst).mode as libc::c_uint
                == strm_killed as libc::c_int as libc::c_uint
            {
                (*strm).dst = 0 as *mut strm_stream;
            }
        }
        if !((*strm).rest).is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) < (*strm).rsize {
                strm_task_push(
                    *((*strm).rest).offset(i as isize),
                    (**((*strm).rest).offset(i as isize)).start_func,
                    data,
                );
                i += 1;
                i;
            }
        }
        if ((*strm).dst).is_null() {
            let mut closed: libc::c_int = 1 as libc::c_int;
            if !((*strm).rest).is_null() {
                let mut i_0: libc::c_int = 0;
                i_0 = 0 as libc::c_int;
                while (i_0 as libc::c_ulong) < (*strm).rsize {
                    if (**((*strm).rest).offset(i_0 as isize)).mode as libc::c_uint
                        != strm_killed as libc::c_int as libc::c_uint
                    {
                        closed = 0 as libc::c_int;
                        break;
                    } else {
                        i_0 += 1;
                        i_0;
                    }
                }
            }
            if closed != 0 {
                (*strm).mode = strm_dying;
                return;
            }
        }
    }
    sched_yield();
    if func.is_some() {
        strm_task_push(strm, func, strm_nil_value());
    }
}
pub unsafe extern "C" fn strm_stream_connect(
    mut src: *mut strm_stream,
    mut dst: *mut strm_stream,
) -> libc::c_int {
    if (*src).mode as libc::c_uint != strm_consumer as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"src->mode != strm_consumer\0" as *const u8 as *const libc::c_char,
            b"core.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int strm_stream_connect(strm_stream *, strm_stream *)\0"))
                .as_ptr(),
        );
    }
    'c_3797: {
        if (*src).mode as libc::c_uint != strm_consumer as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"src->mode != strm_consumer\0" as *const u8 as *const libc::c_char,
                b"core.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int strm_stream_connect(strm_stream *, strm_stream *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*dst).mode as libc::c_uint != strm_producer as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"dst->mode != strm_producer\0" as *const u8 as *const libc::c_char,
            b"core.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int strm_stream_connect(strm_stream *, strm_stream *)\0"))
                .as_ptr(),
        );
    }
    'c_3748: {
        if (*dst).mode as libc::c_uint != strm_producer as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"dst->mode != strm_producer\0" as *const u8 as *const libc::c_char,
                b"core.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int strm_stream_connect(strm_stream *, strm_stream *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*src).dst).is_null() {
        (*src).dst = dst;
    } else {
        if (*src).rsize <= (*src).rcapa {
            (*src)
                .rcapa = ((*src).rcapa)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            (*src)
                .rest = realloc(
                (*src).rest as *mut libc::c_void,
                (::std::mem::size_of::<*mut strm_stream>() as libc::c_ulong)
                    .wrapping_mul((*src).rcapa),
            ) as *mut *mut strm_stream;
        }
        let fresh0 = (*src).rsize;
        (*src).rsize = ((*src).rsize).wrapping_add(1);
        let ref mut fresh1 = *((*src).rest).offset(fresh0 as isize);
        *fresh1 = dst;
    }
    ::std::intrinsics::atomic_xadd::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(&mut (*dst).refcnt, 1 as libc::c_int);
    if (*src).mode as libc::c_uint == strm_producer as libc::c_int as libc::c_uint {
        task_init();
        strm_task_push(src, (*src).start_func, strm_nil_value());
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn worker_count() -> libc::c_int {
    let mut e: *mut libc::c_char = getenv(
        b"STRM_WORKER_MAX\0" as *const u8 as *const libc::c_char,
    );
    let mut n: libc::c_int = 0;
    if !e.is_null() {
        n = atoi(e);
        if n > 0 as libc::c_int {
            return n;
        }
    }
    return cpu_count();
}
unsafe extern "C" fn task_exec(mut strm: *mut strm_stream, mut task: *mut strm_task) {
    let mut func: strm_callback = (*task).func;
    let mut data: strm_value = (*task).data;
    free(task as *mut libc::c_void);
    if (*strm).mode as libc::c_uint == strm_killed as libc::c_int as libc::c_uint {
        return;
    }
    if (Some(func.unwrap())).unwrap()(strm, data) == 1 as libc::c_int {
        if strm_option_verbose != 0 {
            strm_eprint(strm);
        }
    }
    if (*strm).mode as libc::c_uint == strm_dying as libc::c_int as libc::c_uint {
        strm_stream_close(strm);
    }
}
unsafe extern "C" fn task_loop(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut strm: *mut strm_stream = 0 as *mut strm_stream;
    loop {
        strm = strm_queue_get(queue) as *mut strm_stream;
        if strm.is_null() {
            strm = strm_queue_get(prod_queue) as *mut strm_stream;
        }
        if !strm.is_null() {
            if (::std::intrinsics::atomic_cxchg::<_, { std::intrinsics::AtomicOrdering::SeqCst }, { std::intrinsics::AtomicOrdering::SeqCst }>(
                &mut (*strm).excl as *mut strm_int,
                0 as libc::c_int,
                1 as libc::c_int,
            ))
                .1
            {
                let mut t: *mut strm_task = 0 as *mut strm_task;
                loop {
                    t = strm_queue_get((*strm).queue) as *mut strm_task;
                    if t.is_null() {
                        break;
                    }
                    task_exec(strm, t);
                }
                (::std::intrinsics::atomic_cxchg::<_, { std::intrinsics::AtomicOrdering::SeqCst }, { std::intrinsics::AtomicOrdering::SeqCst }>(
                    &mut (*strm).excl,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ))
                    .1;
            }
        }
        if stream_count == 0 as libc::c_int {
            break;
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn task_init() {
    let mut i: libc::c_int = 0;
    if !workers.is_null() {
        return;
    }
    strm_event_loop_started = 1 as libc::c_int;
    strm_init_io_loop();
    queue = strm_queue_new();
    prod_queue = strm_queue_new();
    worker_max = worker_count();
    workers = malloc(
        (::std::mem::size_of::<strm_worker>() as libc::c_ulong)
            .wrapping_mul(worker_max as libc::c_ulong),
    ) as *mut strm_worker;
    i = 0 as libc::c_int;
    while i < worker_max {
        pthread_create(
            &mut (*workers.offset(i as isize)).th,
            0 as *const pthread_attr_t,
            Some(
                task_loop as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut *workers.offset(i as isize) as *mut strm_worker as *mut libc::c_void,
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn strm_loop() -> libc::c_int {
    if stream_count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    task_init();
    loop {
        sched_yield();
        if stream_count == 0 as libc::c_int {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_stream_new(
    mut mode: strm_stream_mode,
    mut start_func: strm_callback,
    mut close_func: strm_callback,
    mut data: *mut libc::c_void,
) -> *mut strm_stream {
    let mut s: *mut strm_stream = malloc(
        ::std::mem::size_of::<strm_stream>() as libc::c_ulong,
    ) as *mut strm_stream;
    (*s).type_0 = STRM_PTR_STREAM;
    (*s).mode = mode;
    (*s).start_func = start_func;
    (*s).close_func = close_func;
    (*s).data = data;
    (*s).dst = 0 as *mut strm_stream;
    (*s).rest = 0 as *mut *mut strm_stream;
    (*s).rsize = 0 as libc::c_int as size_t;
    (*s).rcapa = 0 as libc::c_int as size_t;
    (*s).flags = 0 as libc::c_int as libc::c_uint;
    (*s).exc = 0 as *mut node_error;
    (*s).refcnt = 0 as libc::c_int;
    (*s).excl = 0 as libc::c_int;
    (*s).queue = strm_queue_new();
    ::std::intrinsics::atomic_xadd::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(&mut stream_count, 1 as libc::c_int);
    return s;
}
pub unsafe extern "C" fn strm_stream_close(mut strm: *mut strm_stream) {
    let mut mode: strm_stream_mode = (*strm).mode;
    if mode as libc::c_uint == strm_killed as libc::c_int as libc::c_uint {
        return;
    }
    ::std::intrinsics::atomic_xsub::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(&mut (*strm).refcnt, 1 as libc::c_int);
    if (*strm).refcnt > 0 as libc::c_int {
        return;
    }
    if !(::std::intrinsics::atomic_cxchg::<_, { std::intrinsics::AtomicOrdering::SeqCst }, { std::intrinsics::AtomicOrdering::SeqCst }>(
        &mut (*strm).mode as *mut strm_stream_mode,
        mode,
        strm_killed,
    ))
        .1
    {
        return;
    }
    if ((*strm).close_func).is_some() {
        if (Some(((*strm).close_func).unwrap())).unwrap()(strm, strm_nil_value())
            == 1 as libc::c_int
        {
            return;
        }
    } else {
        free((*strm).data);
        (*strm).data = 0 as *mut libc::c_void;
    }
    if !((*strm).dst).is_null() {
        strm_task_push(
            (*strm).dst,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut strm_stream) -> ()>,
                strm_callback,
            >(Some(strm_stream_close as unsafe extern "C" fn(*mut strm_stream) -> ())),
            strm_nil_value(),
        );
    }
    if !((*strm).rest).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*strm).rsize {
            strm_task_push(
                *((*strm).rest).offset(i as isize),
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut strm_stream) -> ()>,
                    strm_callback,
                >(
                    Some(
                        strm_stream_close as unsafe extern "C" fn(*mut strm_stream) -> (),
                    ),
                ),
                strm_nil_value(),
            );
            i += 1;
            i;
        }
        free((*strm).rest as *mut libc::c_void);
    }
    ::std::intrinsics::atomic_xsub::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(&mut stream_count, 1 as libc::c_int);
}
