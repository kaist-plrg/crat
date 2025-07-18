use ::libc;
extern "C" {
    pub type osip_mutex;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_cond {
    pub cv: pthread_cond_t,
}
pub type osip_cond_t = osip_cond;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub unsafe extern "C" fn osip_cond_init() -> *mut osip_cond {
    let mut cond: *mut osip_cond_t = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_cond_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_cond_t>() as libc::c_ulong)
    }) as *mut osip_cond_t;
    if !cond.is_null()
        && pthread_cond_init(&mut (*cond).cv, 0 as *const pthread_condattr_t)
            == 0 as libc::c_int
    {
        return cond as *mut osip_cond;
    }
    if !cond.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(cond as *mut libc::c_void);
        } else {
            free(cond as *mut libc::c_void);
        }
    }
    return 0 as *mut osip_cond;
}
pub unsafe extern "C" fn osip_cond_destroy(mut _cond: *mut osip_cond) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if _cond.is_null() {
        return -(2 as libc::c_int);
    }
    ret = pthread_cond_destroy(&mut (*_cond).cv);
    if !_cond.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(_cond as *mut libc::c_void);
        } else {
            free(_cond as *mut libc::c_void);
        }
    }
    return ret;
}
pub unsafe extern "C" fn osip_cond_signal(mut _cond: *mut osip_cond) -> libc::c_int {
    if _cond.is_null() {
        return -(2 as libc::c_int);
    }
    return pthread_cond_signal(&mut (*_cond).cv);
}
pub unsafe extern "C" fn osip_cond_wait(
    mut _cond: *mut osip_cond,
    mut _mut: *mut osip_mutex,
) -> libc::c_int {
    if _cond.is_null() {
        return -(2 as libc::c_int);
    }
    return pthread_cond_wait(&mut (*_cond).cv, _mut as *mut pthread_mutex_t);
}
pub unsafe extern "C" fn osip_cond_timedwait(
    mut _cond: *mut osip_cond,
    mut _mut: *mut osip_mutex,
    mut abstime: *const timespec,
) -> libc::c_int {
    if _cond.is_null() {
        return -(2 as libc::c_int);
    }
    return pthread_cond_timedwait(
        &mut (*_cond).cv,
        _mut as *mut pthread_mutex_t,
        abstime,
    );
}
