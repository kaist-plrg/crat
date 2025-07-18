use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type osip_thread;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type osip_thread_t = pthread_t;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type _trace_level = libc::c_uint;
pub const END_TRACE_LEVEL: _trace_level = 8;
pub const TRACE_LEVEL7: _trace_level = 7;
pub const TRACE_LEVEL6: _trace_level = 6;
pub const TRACE_LEVEL5: _trace_level = 5;
pub const TRACE_LEVEL4: _trace_level = 4;
pub const TRACE_LEVEL3: _trace_level = 3;
pub const TRACE_LEVEL2: _trace_level = 2;
pub const TRACE_LEVEL1: _trace_level = 1;
pub const TRACE_LEVEL0: _trace_level = 0;
pub type osip_trace_level_t = _trace_level;
pub unsafe extern "C" fn osip_thread_create(
    mut stacksize: libc::c_int,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> *mut osip_thread {
    let mut i: libc::c_int = 0;
    let mut thread: *mut osip_thread_t = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(::std::mem::size_of::<osip_thread_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_thread_t>() as libc::c_ulong)
    }) as *mut osip_thread_t;
    if thread.is_null() {
        return 0 as *mut osip_thread;
    }
    i = pthread_create(thread, 0 as *const pthread_attr_t, func, arg);
    if i != 0 as libc::c_int {
        osip_trace(
            b"port_thread.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"Error while creating a new thread\n\0" as *const u8 as *const libc::c_char,
        );
        if !thread.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(thread as *mut libc::c_void);
            } else {
                free(thread as *mut libc::c_void);
            }
        }
        return 0 as *mut osip_thread;
    }
    return thread as *mut osip_thread;
}
pub unsafe extern "C" fn osip_thread_set_priority(
    mut thread: *mut osip_thread,
    mut priority: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_thread_join(mut _thread: *mut osip_thread) -> libc::c_int {
    let mut thread: *mut osip_thread_t = _thread as *mut osip_thread_t;
    if thread.is_null() {
        return -(2 as libc::c_int);
    }
    return pthread_join(*thread, 0 as *mut *mut libc::c_void);
}
pub unsafe extern "C" fn osip_thread_exit() {
    pthread_exit(0 as *mut libc::c_void);
}
