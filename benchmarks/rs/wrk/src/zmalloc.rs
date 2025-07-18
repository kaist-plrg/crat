use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub unsafe extern "C" fn zlibc_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
static mut used_memory: size_t = 0 as libc::c_int as size_t;
pub static mut used_memory_mutex: pthread_mutex_t = pthread_mutex_t {
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
};
unsafe extern "C" fn zmalloc_default_oom(mut size: size_t) {
    fprintf(
        stderr,
        b"zmalloc: Out of memory trying to allocate %zu bytes\n\0" as *const u8
            as *const libc::c_char,
        size,
    );
    fflush(stderr);
    abort();
}
static mut zmalloc_oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()> = unsafe {
    Some(zmalloc_default_oom as unsafe extern "C" fn(size_t) -> ())
};
pub unsafe extern "C" fn zmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(
        size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    if ptr.is_null() {
        zmalloc_oom_handler.unwrap()(size);
    }
    *(ptr as *mut size_t) = size;
    let mut _n: size_t = size
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong)
        .wrapping_add(
            size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
pub unsafe extern "C" fn zcalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = calloc(
        1 as libc::c_int as libc::c_ulong,
        size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    if ptr.is_null() {
        zmalloc_oom_handler.unwrap()(size);
    }
    *(ptr as *mut size_t) = size;
    let mut _n: size_t = size
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong)
        .wrapping_add(
            size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
pub unsafe extern "C" fn zrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut realptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldsize: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if ptr.is_null() {
        return zmalloc(size);
    }
    realptr = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize))
        as *mut libc::c_void;
    oldsize = *(realptr as *mut size_t);
    newptr = realloc(
        realptr,
        size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    if newptr.is_null() {
        zmalloc_oom_handler.unwrap()(size);
    }
    *(newptr as *mut size_t) = size;
    let mut _n: size_t = oldsize;
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong).wrapping_sub(oldsize) as size_t
        as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    let mut _n_0: size_t = size;
    if _n_0
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        _n_0 = (_n_0 as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n_0
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return (newptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
pub unsafe extern "C" fn zmalloc_size(mut ptr: *mut libc::c_void) -> size_t {
    let mut realptr: *mut libc::c_void = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize))
        as *mut libc::c_void;
    let mut size: size_t = *(realptr as *mut size_t);
    if size
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        size = (size as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        size
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    return size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
}
pub unsafe extern "C" fn zfree(mut ptr: *mut libc::c_void) {
    let mut realptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldsize: size_t = 0;
    if ptr.is_null() {
        return;
    }
    realptr = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize))
        as *mut libc::c_void;
    oldsize = *(realptr as *mut size_t);
    let mut _n: size_t = oldsize
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong)
        .wrapping_sub(
            oldsize.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    free(realptr);
}
pub unsafe extern "C" fn zstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut l: size_t = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut p: *mut libc::c_char = zmalloc(l) as *mut libc::c_char;
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, l);
    return p;
}
pub unsafe extern "C" fn zmalloc_used_memory() -> size_t {
    let mut um: size_t = 0;
    pthread_mutex_lock(&mut used_memory_mutex);
    um = used_memory;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return um;
}
pub unsafe extern "C" fn zmalloc_set_oom_handler(
    mut oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()>,
) {
    zmalloc_oom_handler = oom_handler;
}
pub unsafe extern "C" fn zmalloc_get_rss() -> size_t {
    return zmalloc_used_memory();
}
pub unsafe extern "C" fn zmalloc_get_fragmentation_ratio(
    mut rss: size_t,
) -> libc::c_float {
    return rss as libc::c_float / zmalloc_used_memory() as libc::c_float;
}
pub unsafe extern "C" fn zmalloc_get_smap_bytes_by_field(
    mut field: *mut libc::c_char,
    mut pid: libc::c_long,
) -> size_t {
    return 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn zmalloc_get_private_dirty(mut pid: libc::c_long) -> size_t {
    return zmalloc_get_smap_bytes_by_field(
        b"Private_Dirty:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pid,
    );
}
pub unsafe extern "C" fn zmalloc_get_memory_size() -> size_t {
    return 0 as libc::c_long as size_t;
}
