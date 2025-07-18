use ::libc;
extern "C" {
    pub type osip_sem;
    pub type osip_mutex;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn sem_trywait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
    static mut osip_free_func: Option::<osip_free_func_t>;
}
pub type size_t = libc::c_ulong;
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
pub union pthread_mutexattr_t {
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
pub type osip_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
pub type osip_sem_t = sem_t;
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub unsafe extern "C" fn osip_mutex_init() -> *mut osip_mutex {
    let mut mut_0: *mut osip_mutex_t = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_mutex_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_mutex_t>() as libc::c_ulong)
    }) as *mut osip_mutex_t;
    if mut_0.is_null() {
        return 0 as *mut osip_mutex;
    }
    pthread_mutex_init(mut_0, 0 as *const pthread_mutexattr_t);
    return mut_0 as *mut osip_mutex;
}
pub unsafe extern "C" fn osip_mutex_destroy(mut _mut: *mut osip_mutex) {
    let mut mut_0: *mut osip_mutex_t = _mut as *mut osip_mutex_t;
    if mut_0.is_null() {
        return;
    }
    pthread_mutex_destroy(mut_0);
    if !mut_0.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(mut_0 as *mut libc::c_void);
        } else {
            free(mut_0 as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn osip_mutex_lock(mut _mut: *mut osip_mutex) -> libc::c_int {
    let mut mut_0: *mut osip_mutex_t = _mut as *mut osip_mutex_t;
    if mut_0.is_null() {
        return -(2 as libc::c_int);
    }
    return pthread_mutex_lock(mut_0);
}
pub unsafe extern "C" fn osip_mutex_unlock(mut _mut: *mut osip_mutex) -> libc::c_int {
    let mut mut_0: *mut osip_mutex_t = _mut as *mut osip_mutex_t;
    if mut_0.is_null() {
        return -(2 as libc::c_int);
    }
    return pthread_mutex_unlock(mut_0);
}
pub unsafe extern "C" fn osip_sem_init(mut value: libc::c_uint) -> *mut osip_sem {
    let mut sem: *mut osip_sem_t = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<osip_sem_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<osip_sem_t>() as libc::c_ulong)
    }) as *mut osip_sem_t;
    if sem.is_null() {
        return 0 as *mut osip_sem;
    }
    if sem_init(sem, 0 as libc::c_int, value) == 0 as libc::c_int {
        return sem as *mut osip_sem;
    }
    if !sem.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(sem as *mut libc::c_void);
        } else {
            free(sem as *mut libc::c_void);
        }
    }
    return 0 as *mut osip_sem;
}
pub unsafe extern "C" fn osip_sem_destroy(mut _sem: *mut osip_sem) -> libc::c_int {
    let mut sem: *mut osip_sem_t = _sem as *mut osip_sem_t;
    if sem.is_null() {
        return 0 as libc::c_int;
    }
    sem_destroy(sem);
    if !sem.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(sem as *mut libc::c_void);
        } else {
            free(sem as *mut libc::c_void);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_sem_post(mut _sem: *mut osip_sem) -> libc::c_int {
    let mut sem: *mut osip_sem_t = _sem as *mut osip_sem_t;
    if sem.is_null() {
        return -(2 as libc::c_int);
    }
    return sem_post(sem);
}
pub unsafe extern "C" fn osip_sem_wait(mut _sem: *mut osip_sem) -> libc::c_int {
    let mut sem: *mut osip_sem_t = _sem as *mut osip_sem_t;
    if sem.is_null() {
        return -(2 as libc::c_int);
    }
    return sem_wait(sem);
}
pub unsafe extern "C" fn osip_sem_trywait(mut _sem: *mut osip_sem) -> libc::c_int {
    let mut sem: *mut osip_sem_t = _sem as *mut osip_sem_t;
    if sem.is_null() {
        return -(2 as libc::c_int);
    }
    return sem_trywait(sem);
}
