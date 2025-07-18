use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __jmp_buf_tag;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: libc::c_int,
    ) -> libc::c_int;
    fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_CREATE_DETACHED: C2RustUnnamed_3 = 1;
pub const PTHREAD_CREATE_JOINABLE: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_4 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [C2RustUnnamed_5; 1],
    pub __pad: [*mut libc::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
}
pub type free_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type malloc_t = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_s {
    pub id: pthread_t,
    pub done: libc::c_int,
    pub next: *mut thread,
}
pub type thread = thread_s;
pub type lock = lock_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lock_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub value: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct capsule {
    pub probe: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub payload: *mut libc::c_void,
    pub file: *const libc::c_char,
    pub line: libc::c_long,
}
pub type twist_op = libc::c_uint;
pub const BY: twist_op = 1;
pub const TO: twist_op = 0;
pub type wait_op = libc::c_uint;
pub const TO_BE_LESS_THAN: wait_op = 3;
pub const TO_BE_MORE_THAN: wait_op = 2;
pub const NOT_TO_BE: wait_op = 1;
pub const TO_BE: wait_op = 0;
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
pub static mut yarn_prefix: *mut libc::c_char = b"yarn\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut yarn_abort: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn fail(
    mut err: libc::c_int,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
    mut func: *const libc::c_char,
) {
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, yarn_prefix);
    match err {
        1 => {
            fputs(b"already unlocked\0" as *const u8 as *const libc::c_char, stderr);
        }
        3 => {
            fputs(b"no such thread\0" as *const u8 as *const libc::c_char, stderr);
        }
        35 => {
            fputs(b"resource deadlock\0" as *const u8 as *const libc::c_char, stderr);
        }
        12 => {
            fputs(b"out of memory\0" as *const u8 as *const libc::c_char, stderr);
        }
        16 => {
            fputs(
                b"can't destroy locked resource\0" as *const u8 as *const libc::c_char,
                stderr,
            );
        }
        22 => {
            fputs(b"invalid request\0" as *const u8 as *const libc::c_char, stderr);
        }
        11 => {
            fputs(b"resource unavailable\0" as *const u8 as *const libc::c_char, stderr);
        }
        _ => {
            fprintf(
                stderr,
                b"internal error %d\0" as *const u8 as *const libc::c_char,
                err,
            );
        }
    }
    fprintf(
        stderr,
        b" (%s:%ld:%s)\n\0" as *const u8 as *const libc::c_char,
        file,
        line,
        func,
    );
    if yarn_abort.is_some() {
        yarn_abort.unwrap()(err);
    }
    exit(err);
}
static mut my_malloc_f: malloc_t = unsafe {
    Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void)
};
static mut my_free: free_t = unsafe {
    Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())
};
pub unsafe extern "C" fn yarn_mem(mut lease: malloc_t, mut vacate: free_t) {
    my_malloc_f = lease;
    my_free = vacate;
}
unsafe extern "C" fn my_malloc(
    mut size: size_t,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> *mut libc::c_void {
    let mut block: *mut libc::c_void = 0 as *mut libc::c_void;
    block = my_malloc_f.unwrap()(size);
    if block.is_null() {
        fail(
            12 as libc::c_int,
            file,
            line,
            b"malloc\0" as *const u8 as *const libc::c_char,
        );
    }
    return block;
}
pub unsafe extern "C" fn new_lock_(
    mut initial: libc::c_long,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> *mut lock {
    let mut bolt: *mut lock = my_malloc(
        ::std::mem::size_of::<lock_s>() as libc::c_ulong,
        file,
        line,
    ) as *mut lock;
    let mut ret: libc::c_int = pthread_mutex_init(
        &mut (*bolt).mutex,
        0 as *const pthread_mutexattr_t,
    );
    if ret != 0 {
        fail(ret, file, line, b"mutex_init\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_cond_init(&mut (*bolt).cond, 0 as *const pthread_condattr_t);
    if ret != 0 {
        fail(ret, file, line, b"cond_init\0" as *const u8 as *const libc::c_char);
    }
    (*bolt).value = initial;
    return bolt;
}
pub unsafe extern "C" fn possess_(
    mut bolt: *mut lock,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = pthread_mutex_lock(&mut (*bolt).mutex);
    if ret != 0 {
        fail(ret, file, line, b"mutex_lock\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn release_(
    mut bolt: *mut lock,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = pthread_mutex_unlock(&mut (*bolt).mutex);
    if ret != 0 {
        fail(ret, file, line, b"mutex_unlock\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn twist_(
    mut bolt: *mut lock,
    mut op: twist_op,
    mut val: libc::c_long,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    if op as libc::c_uint == TO as libc::c_int as libc::c_uint {
        (*bolt).value = val;
    } else if op as libc::c_uint == BY as libc::c_int as libc::c_uint {
        (*bolt).value += val;
    }
    let mut ret: libc::c_int = pthread_cond_broadcast(&mut (*bolt).cond);
    if ret != 0 {
        fail(ret, file, line, b"cond_broadcast\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_mutex_unlock(&mut (*bolt).mutex);
    if ret != 0 {
        fail(ret, file, line, b"mutex_unlock\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn wait_for_(
    mut bolt: *mut lock,
    mut op: wait_op,
    mut val: libc::c_long,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    match op as libc::c_uint {
        0 => {
            while !((*bolt).value == val) {
                let mut ret: libc::c_int = pthread_cond_wait(
                    &mut (*bolt).cond,
                    &mut (*bolt).mutex,
                );
                if ret != 0 {
                    fail(
                        ret,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        1 => {
            while !((*bolt).value != val) {
                let mut ret_0: libc::c_int = pthread_cond_wait(
                    &mut (*bolt).cond,
                    &mut (*bolt).mutex,
                );
                if ret_0 != 0 {
                    fail(
                        ret_0,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        2 => {
            while !((*bolt).value > val) {
                let mut ret_1: libc::c_int = pthread_cond_wait(
                    &mut (*bolt).cond,
                    &mut (*bolt).mutex,
                );
                if ret_1 != 0 {
                    fail(
                        ret_1,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        3 => {
            while !((*bolt).value < val) {
                let mut ret_2: libc::c_int = pthread_cond_wait(
                    &mut (*bolt).cond,
                    &mut (*bolt).mutex,
                );
                if ret_2 != 0 {
                    fail(
                        ret_2,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn peek_lock(mut bolt: *mut lock) -> libc::c_long {
    return (*bolt).value;
}
pub unsafe extern "C" fn free_lock_(
    mut bolt: *mut lock,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    if bolt.is_null() {
        return;
    }
    let mut ret: libc::c_int = pthread_cond_destroy(&mut (*bolt).cond);
    if ret != 0 {
        fail(ret, file, line, b"cond_destroy\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_mutex_destroy(&mut (*bolt).mutex);
    if ret != 0 {
        fail(ret, file, line, b"mutex_destroy\0" as *const u8 as *const libc::c_char);
    }
    my_free.unwrap()(bolt as *mut libc::c_void);
}
static mut threads_lock: lock = {
    let mut init = lock_s {
        mutex: pthread_mutex_t {
            __data: {
                let mut init = __pthread_mutex_s {
                    __lock: 0 as libc::c_int,
                    __count: 0 as libc::c_int as libc::c_uint,
                    __owner: 0 as libc::c_int,
                    __nusers: 0 as libc::c_int as libc::c_uint,
                    __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                    __spins: 0 as libc::c_int as libc::c_short,
                    __elision: 0 as libc::c_int as libc::c_short,
                    __list: {
                        let mut init = __pthread_internal_list {
                            __prev: 0 as *const __pthread_internal_list
                                as *mut __pthread_internal_list,
                            __next: 0 as *const __pthread_internal_list
                                as *mut __pthread_internal_list,
                        };
                        init
                    },
                };
                init
            },
        },
        cond: pthread_cond_t {
            __data: {
                let mut init = __pthread_cond_s {
                    c2rust_unnamed: C2RustUnnamed_1 {
                        __wseq: 0 as libc::c_int as libc::c_ulonglong,
                    },
                    c2rust_unnamed_0: C2RustUnnamed {
                        __g1_start: 0 as libc::c_int as libc::c_ulonglong,
                    },
                    __g_refs: [
                        0 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                    ],
                    __g_size: [
                        0 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                    ],
                    __g1_orig_size: 0 as libc::c_int as libc::c_uint,
                    __wrefs: 0 as libc::c_int as libc::c_uint,
                    __g_signals: [
                        0 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                    ],
                };
                init
            },
        },
        value: 0 as libc::c_int as libc::c_long,
    };
    init
};
static mut threads: *mut thread = 0 as *const thread as *mut thread;
unsafe extern "C" fn reenter(mut arg: *mut libc::c_void) {
    let mut capsule: *mut capsule = arg as *mut capsule;
    let mut me: pthread_t = pthread_self();
    possess_(&mut threads_lock, (*capsule).file, (*capsule).line);
    let mut prior: *mut *mut thread = &mut threads;
    let mut match_0: *mut thread = 0 as *mut thread;
    loop {
        match_0 = *prior;
        if match_0.is_null() {
            break;
        }
        if pthread_equal((*match_0).id, me) != 0 {
            break;
        }
        prior = &mut (*match_0).next;
    }
    if match_0.is_null() {
        fail(
            3 as libc::c_int,
            (*capsule).file,
            (*capsule).line,
            b"reenter lost\0" as *const u8 as *const libc::c_char,
        );
    }
    (*match_0).done = 1 as libc::c_int;
    if threads != match_0 {
        *prior = (*match_0).next;
        (*match_0).next = threads;
        threads = match_0;
    }
    twist_(
        &mut threads_lock,
        BY,
        1 as libc::c_int as libc::c_long,
        (*capsule).file,
        (*capsule).line,
    );
    my_free.unwrap()(capsule as *mut libc::c_void);
}
unsafe extern "C" fn ignition(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut capsule: *mut capsule = arg as *mut capsule;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [C2RustUnnamed_5 {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = Some(
        reenter as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let mut __cancel_arg: *mut libc::c_void = arg;
    let mut __not_first_call: libc::c_int = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0 as libc::c_int,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.unwrap()(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    ((*capsule).probe).unwrap()((*capsule).payload);
    __pthread_unregister_cancel(&mut __cancel_buf);
    __cancel_routine.unwrap()(__cancel_arg);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn launch_(
    mut probe: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut payload: *mut libc::c_void,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> *mut thread {
    let mut capsule: *mut capsule = my_malloc(
        ::std::mem::size_of::<capsule>() as libc::c_ulong,
        file,
        line,
    ) as *mut capsule;
    (*capsule).probe = probe;
    (*capsule).payload = payload;
    (*capsule).file = file;
    (*capsule).line = line;
    possess_(&mut threads_lock, file, line);
    let mut th: *mut thread = my_malloc(
        ::std::mem::size_of::<thread_s>() as libc::c_ulong,
        file,
        line,
    ) as *mut thread;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut ret: libc::c_int = pthread_attr_init(&mut attr);
    if ret != 0 {
        fail(ret, file, line, b"attr_init\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_attr_setdetachstate(&mut attr, PTHREAD_CREATE_JOINABLE as libc::c_int);
    if ret != 0 {
        fail(
            ret,
            file,
            line,
            b"attr_setdetachstate\0" as *const u8 as *const libc::c_char,
        );
    }
    ret = pthread_create(
        &mut (*th).id,
        &mut attr,
        Some(ignition as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        capsule as *mut libc::c_void,
    );
    if ret != 0 {
        fail(ret, file, line, b"create\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_attr_destroy(&mut attr);
    if ret != 0 {
        fail(ret, file, line, b"attr_destroy\0" as *const u8 as *const libc::c_char);
    }
    (*th).done = 0 as libc::c_int;
    (*th).next = threads;
    threads = th;
    release_(&mut threads_lock, file, line);
    return th;
}
pub unsafe extern "C" fn join_(
    mut ally: *mut thread,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = pthread_join((*ally).id, 0 as *mut *mut libc::c_void);
    if ret != 0 {
        fail(ret, file, line, b"join\0" as *const u8 as *const libc::c_char);
    }
    possess_(&mut threads_lock, file, line);
    let mut prior: *mut *mut thread = &mut threads;
    let mut match_0: *mut thread = 0 as *mut thread;
    loop {
        match_0 = *prior;
        if match_0.is_null() {
            break;
        }
        if match_0 == ally {
            break;
        }
        prior = &mut (*match_0).next;
    }
    if match_0.is_null() {
        fail(
            3 as libc::c_int,
            file,
            line,
            b"join lost\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*match_0).done != 0 {
        threads_lock.value -= 1;
        threads_lock.value;
    }
    *prior = (*match_0).next;
    release_(&mut threads_lock, file, line);
    my_free.unwrap()(ally as *mut libc::c_void);
}
pub unsafe extern "C" fn join_all_(
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    possess_(&mut threads_lock, file, line);
    while !threads.is_null() {
        wait_for_(
            &mut threads_lock,
            NOT_TO_BE,
            0 as libc::c_int as libc::c_long,
            file,
            line,
        );
        let mut prior: *mut *mut thread = &mut threads;
        let mut match_0: *mut thread = 0 as *mut thread;
        loop {
            match_0 = *prior;
            if match_0.is_null() {
                break;
            }
            if (*match_0).done != 0 {
                break;
            }
            prior = &mut (*match_0).next;
        }
        if match_0.is_null() {
            fail(
                3 as libc::c_int,
                file,
                line,
                b"join_all lost\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut ret: libc::c_int = pthread_join(
            (*match_0).id,
            0 as *mut *mut libc::c_void,
        );
        if ret != 0 {
            fail(ret, file, line, b"join\0" as *const u8 as *const libc::c_char);
        }
        threads_lock.value -= 1;
        threads_lock.value;
        *prior = (*match_0).next;
        my_free.unwrap()(match_0 as *mut libc::c_void);
        count += 1;
        count;
    }
    release_(&mut threads_lock, file, line);
    return count;
}
