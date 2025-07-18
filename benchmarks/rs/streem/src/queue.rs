use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_queue {
    pub head: *mut strm_queue_node,
    pub tail: *mut strm_queue_node,
    pub mutex: pthread_mutex_t,
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_queue_node {
    pub next: *mut strm_queue_node,
    pub n: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub unsafe extern "C" fn strm_queue_new() -> *mut strm_queue {
    let mut q: *mut strm_queue = 0 as *mut strm_queue;
    q = malloc(::std::mem::size_of::<strm_queue>() as libc::c_ulong) as *mut strm_queue;
    if q.is_null() {
        return 0 as *mut strm_queue;
    }
    (*q).head = 0 as *mut strm_queue_node;
    (*q).tail = 0 as *mut strm_queue_node;
    pthread_mutex_init(&mut (*q).mutex, 0 as *const pthread_mutexattr_t);
    return q;
}
pub unsafe extern "C" fn strm_queue_add(
    mut q: *mut strm_queue,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    let mut node: *mut strm_queue_node = malloc(
        ::std::mem::size_of::<strm_queue_node>() as libc::c_ulong,
    ) as *mut strm_queue_node;
    if node.is_null() {
        return 0 as libc::c_int;
    }
    (*node).n = val;
    (*node).next = 0 as *mut strm_queue_node;
    pthread_mutex_lock(&mut (*q).mutex);
    if !((*q).tail).is_null() {
        (*(*q).tail).next = node;
    }
    (*q).tail = node;
    if ((*q).head).is_null() {
        (*q).head = node;
    }
    pthread_mutex_unlock(&mut (*q).mutex);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn strm_queue_get(mut q: *mut strm_queue) -> *mut libc::c_void {
    let mut t: *mut strm_queue_node = 0 as *mut strm_queue_node;
    let mut n: *mut libc::c_void = 0 as *mut libc::c_void;
    pthread_mutex_lock(&mut (*q).mutex);
    if ((*q).head).is_null() {
        pthread_mutex_unlock(&mut (*q).mutex);
        return 0 as *mut libc::c_void;
    }
    t = (*q).head;
    (*q).head = (*t).next;
    if ((*q).head).is_null() {
        (*q).tail = 0 as *mut strm_queue_node;
    }
    pthread_mutex_unlock(&mut (*q).mutex);
    n = (*t).n;
    free(t as *mut libc::c_void);
    return n;
}
pub unsafe extern "C" fn strm_queue_free(mut q: *mut strm_queue) {
    if q.is_null() {
        return;
    }
    if !((*q).head).is_null() {
        let mut t: *mut strm_queue_node = (*q).head;
        let mut tmp: *mut strm_queue_node = 0 as *mut strm_queue_node;
        while !t.is_null() {
            tmp = (*t).next;
            free(t as *mut libc::c_void);
            t = tmp;
        }
    }
    pthread_mutex_destroy(&mut (*q).mutex);
    free(q as *mut libc::c_void);
}
pub unsafe extern "C" fn strm_queue_empty_p(mut q: *mut strm_queue) -> libc::c_int {
    if ((*q).head).is_null() {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
