use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn gh_log_die() -> !;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct brubeck_slab {
    pub current: *mut brubeck_slab_node,
    pub total_alloc: size_t,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_slab_node {
    pub next: *mut brubeck_slab_node,
    pub alloc: size_t,
    pub heap: [libc::c_char; 0],
}
#[inline]
unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(size);
    if (ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
unsafe extern "C" fn push_node(mut slab: *mut brubeck_slab) -> *mut brubeck_slab_node {
    let mut node: *mut brubeck_slab_node = xmalloc(
        (32 as libc::c_int * 128 as libc::c_int) as size_t,
    ) as *mut brubeck_slab_node;
    (*node).alloc = 0 as libc::c_int as size_t;
    (*node).next = (*slab).current;
    (*slab).current = node;
    return node;
}
pub unsafe extern "C" fn brubeck_slab_alloc(
    mut slab: *mut brubeck_slab,
    mut need: size_t,
) -> *mut libc::c_void {
    let mut node: *mut brubeck_slab_node = 0 as *mut brubeck_slab_node;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    need = need
        .wrapping_add(32 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    pthread_mutex_lock(&mut (*slab).lock);
    node = (*slab).current;
    if ((*node).alloc).wrapping_add(need)
        > (32 as libc::c_int * (128 as libc::c_int - 1 as libc::c_int)) as libc::c_ulong
    {
        node = push_node(slab);
    }
    (*slab)
        .total_alloc = ((*slab).total_alloc as libc::c_ulong).wrapping_add(need)
        as size_t as size_t;
    ptr = ((*node).heap).as_mut_ptr().offset((*node).alloc as isize)
        as *mut libc::c_void;
    (*node)
        .alloc = ((*node).alloc as libc::c_ulong).wrapping_add(need) as size_t as size_t;
    pthread_mutex_unlock(&mut (*slab).lock);
    return ptr;
}
pub unsafe extern "C" fn brubeck_slab_init(mut slab: *mut brubeck_slab) {
    push_node(slab);
    pthread_mutex_init(&mut (*slab).lock, 0 as *const pthread_mutexattr_t);
}
