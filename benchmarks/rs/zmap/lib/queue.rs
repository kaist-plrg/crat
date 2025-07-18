use ::libc;
extern "C" {
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
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
pub struct zqueue_node {
    pub data: *mut libc::c_char,
    pub prev: *mut zqueue_node,
    pub next: *mut zqueue_node,
}
pub type znode_t = zqueue_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zqueue {
    pub front: *mut zqueue_node,
    pub back: *mut zqueue_node,
    pub size: size_t,
    pub lock: pthread_mutex_t,
    pub empty: pthread_cond_t,
}
pub type zqueue_t = zqueue;
pub unsafe extern "C" fn queue_init() -> *mut zqueue_t {
    let mut p: *mut zqueue_t = xmalloc(
        ::std::mem::size_of::<zqueue_t>() as libc::c_ulong,
    ) as *mut zqueue_t;
    (*p).front = 0 as *mut zqueue_node;
    (*p).back = 0 as *mut zqueue_node;
    (*p).size = 0 as libc::c_int as size_t;
    pthread_mutex_init(&mut (*p).lock, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*p).empty, 0 as *const pthread_condattr_t);
    return p;
}
pub unsafe extern "C" fn is_empty(mut queue: *mut zqueue_t) -> libc::c_int {
    return ((*queue).size == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn push_back(
    mut data: *mut libc::c_char,
    mut queue: *mut zqueue_t,
) {
    let mut new_node: *mut znode_t = xmalloc(
        ::std::mem::size_of::<znode_t>() as libc::c_ulong,
    ) as *mut znode_t;
    (*new_node).prev = 0 as *mut zqueue_node;
    (*new_node).next = 0 as *mut zqueue_node;
    (*new_node).data = strdup(data);
    pthread_mutex_lock(&mut (*queue).lock);
    if is_empty(queue) != 0 {
        (*queue).front = new_node;
        (*queue).back = new_node;
    } else {
        (*(*queue).back).next = new_node;
        (*new_node).prev = (*queue).back;
        (*queue).back = new_node;
    }
    (*queue).size = ((*queue).size).wrapping_add(1);
    (*queue).size;
    pthread_cond_signal(&mut (*queue).empty);
    pthread_mutex_unlock(&mut (*queue).lock);
}
pub unsafe extern "C" fn pop_front(mut queue: *mut zqueue_t) -> *mut znode_t {
    pthread_mutex_lock(&mut (*queue).lock);
    while is_empty(queue) != 0 {
        pthread_cond_wait(&mut (*queue).empty, &mut (*queue).lock);
    }
    let mut temp: *mut znode_t = pop_front_unsafe(queue);
    pthread_mutex_unlock(&mut (*queue).lock);
    return temp;
}
pub unsafe extern "C" fn pop_front_unsafe(mut queue: *mut zqueue_t) -> *mut znode_t {
    let mut temp: *mut znode_t = (*queue).front;
    (*queue).front = (*temp).next;
    if !((*queue).front).is_null() {
        (*(*queue).front).prev = 0 as *mut zqueue_node;
    }
    (*queue).size = ((*queue).size).wrapping_sub(1);
    (*queue).size;
    return temp;
}
pub unsafe extern "C" fn get_front(mut queue: *mut zqueue_t) -> *mut znode_t {
    pthread_mutex_lock(&mut (*queue).lock);
    while is_empty(queue) != 0 {
        pthread_cond_wait(&mut (*queue).empty, &mut (*queue).lock);
    }
    let mut temp: *mut znode_t = xmalloc(
        ::std::mem::size_of::<znode_t>() as libc::c_ulong,
    ) as *mut znode_t;
    temp = (*queue).front;
    pthread_mutex_unlock(&mut (*queue).lock);
    return temp;
}
pub unsafe extern "C" fn get_back(mut queue: *mut zqueue_t) -> *mut znode_t {
    pthread_mutex_lock(&mut (*queue).lock);
    while is_empty(queue) != 0 {
        pthread_cond_wait(&mut (*queue).empty, &mut (*queue).lock);
    }
    let mut temp: *mut znode_t = xmalloc(
        ::std::mem::size_of::<znode_t>() as libc::c_ulong,
    ) as *mut znode_t;
    temp = (*queue).back;
    pthread_mutex_unlock(&mut (*queue).lock);
    return temp;
}
pub unsafe extern "C" fn get_size(mut queue: *mut zqueue_t) -> size_t {
    return (*queue).size;
}
