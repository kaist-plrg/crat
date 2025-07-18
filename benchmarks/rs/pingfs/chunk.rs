use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn net_send(
        host: *mut host,
        id: uint16_t,
        seqno: uint16_t,
        data: *const uint8_t,
        len: size_t,
    );
    fn net_inc_rx(packetsize: libc::c_int);
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
pub type clockid_t = __clockid_t;
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
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host {
    pub next: *mut host,
    pub sockaddr: sockaddr_storage,
    pub sockaddr_len: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io {
    pub fs_cond: pthread_cond_t,
    pub net_cond: pthread_cond_t,
    pub mutex: pthread_mutex_t,
    pub owner: io_owner,
    pub data: *mut uint8_t,
    pub len: size_t,
}
pub type io_owner = libc::c_uint;
pub const OWNER_NET: io_owner = 2;
pub const OWNER_FS: io_owner = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub next_active: *mut chunk,
    pub next_file: *mut chunk,
    pub host: *mut host,
    pub io: *mut io,
    pub id: uint16_t,
    pub seqno: uint16_t,
    pub len: uint16_t,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
static mut icmp_id: uint16_t = 0;
static mut timeout: libc::c_int = 0;
static mut chunk_head: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunk_mutex: pthread_mutex_t = pthread_mutex_t {
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
pub unsafe extern "C" fn chunk_set_timeout(mut t: libc::c_int) {
    timeout = t;
}
pub unsafe extern "C" fn chunk_create() -> *mut chunk {
    let mut c: *mut chunk = 0 as *mut chunk;
    c = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<chunk>() as libc::c_ulong,
    ) as *mut chunk;
    if c.is_null() {
        return 0 as *mut chunk;
    }
    let fresh0 = icmp_id;
    icmp_id = icmp_id.wrapping_add(1);
    (*c).id = fresh0;
    return c;
}
pub unsafe extern "C" fn chunk_free(mut c: *mut chunk) {
    free(c as *mut libc::c_void);
}
pub unsafe extern "C" fn chunk_add(mut c: *mut chunk) {
    pthread_mutex_lock(&mut chunk_mutex);
    (*c).next_active = chunk_head;
    chunk_head = c;
    pthread_mutex_unlock(&mut chunk_mutex);
}
pub unsafe extern "C" fn chunk_remove(mut c: *mut chunk) {
    let mut prev: *mut chunk = 0 as *mut chunk;
    let mut curr: *mut chunk = 0 as *mut chunk;
    prev = 0 as *mut chunk;
    pthread_mutex_lock(&mut chunk_mutex);
    curr = chunk_head;
    while !curr.is_null() {
        if curr == c {
            if !prev.is_null() {
                (*prev).next_active = (*curr).next_active;
            } else {
                chunk_head = (*curr).next_active;
            }
            break;
        } else {
            prev = curr;
            curr = (*curr).next_active;
        }
    }
    pthread_mutex_unlock(&mut chunk_mutex);
}
unsafe extern "C" fn process_chunk(mut c: *mut chunk, mut data: *mut *mut uint8_t) {
    (*c).seqno = ((*c).seqno).wrapping_add(1);
    (*c).seqno;
    if !((*c).io).is_null() {
        let mut io: *mut io = (*c).io;
        pthread_mutex_lock(&mut (*io).mutex);
        (*io).data = *data;
        (*io).len = (*c).len as size_t;
        (*io).owner = OWNER_FS;
        pthread_cond_signal(&mut (*io).fs_cond);
        while (*io).owner as libc::c_uint != OWNER_NET as libc::c_int as libc::c_uint {
            pthread_cond_wait(&mut (*io).net_cond, &mut (*io).mutex);
        }
        *data = (*io).data;
        pthread_mutex_unlock(&mut (*io).mutex);
        free((*c).io as *mut libc::c_void);
        (*c).io = 0 as *mut io;
    }
    net_send((*c).host, (*c).id, (*c).seqno, *data, (*c).len as size_t);
}
pub unsafe extern "C" fn chunk_reply(
    mut userdata: *mut libc::c_void,
    mut addr: *mut sockaddr_storage,
    mut addrlen: size_t,
    mut id: uint16_t,
    mut seqno: uint16_t,
    mut data: *mut *mut uint8_t,
    mut len: size_t,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    pthread_mutex_lock(&mut chunk_mutex);
    c = chunk_head;
    while !c.is_null() {
        if (*c).id as libc::c_int == id as libc::c_int {
            net_inc_rx(len as libc::c_int);
            if len == (*c).len as libc::c_ulong
                && seqno as libc::c_int == (*c).seqno as libc::c_int
            {
                process_chunk(c, data);
            }
            break;
        } else {
            c = (*c).next_active;
        }
    }
    pthread_mutex_unlock(&mut chunk_mutex);
}
pub unsafe extern "C" fn chunk_wait_for(
    mut c: *mut chunk,
    mut data: *mut *mut uint8_t,
) -> libc::c_int {
    pthread_mutex_lock(&mut chunk_mutex);
    if !((*c).io).is_null() {
        pthread_mutex_unlock(&mut chunk_mutex);
        return -(16 as libc::c_int);
    }
    (*c)
        .io = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<io>() as libc::c_ulong,
    ) as *mut io;
    if ((*c).io).is_null() {
        pthread_mutex_unlock(&mut chunk_mutex);
        return -(12 as libc::c_int);
    }
    pthread_mutex_unlock(&mut chunk_mutex);
    (*(*c).io).owner = OWNER_NET;
    pthread_cond_init(&mut (*(*c).io).fs_cond, 0 as *const pthread_condattr_t);
    pthread_cond_init(&mut (*(*c).io).net_cond, 0 as *const pthread_condattr_t);
    if pthread_mutex_init(&mut (*(*c).io).mutex, 0 as *const pthread_mutexattr_t) != 0 {
        pthread_mutex_unlock(&mut chunk_mutex);
        free((*c).io as *mut libc::c_void);
        return -*__errno_location();
    }
    pthread_mutex_lock(&mut (*(*c).io).mutex);
    while (*(*c).io).owner as libc::c_uint != OWNER_FS as libc::c_int as libc::c_uint {
        let mut res: libc::c_int = 0;
        let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(0 as libc::c_int, &mut ts);
        ts.tv_sec += timeout as libc::c_long;
        res = pthread_cond_timedwait(
            &mut (*(*c).io).fs_cond,
            &mut (*(*c).io).mutex,
            &mut ts,
        );
        if res != 0 {
            pthread_mutex_unlock(&mut (*(*c).io).mutex);
            pthread_mutex_lock(&mut chunk_mutex);
            free((*c).io as *mut libc::c_void);
            (*c).io = 0 as *mut io;
            pthread_mutex_unlock(&mut chunk_mutex);
            return 0 as libc::c_int;
        }
    }
    *data = (*(*c).io).data;
    return (*(*c).io).len as libc::c_int;
}
pub unsafe extern "C" fn chunk_done(
    mut c: *mut chunk,
    mut data: *mut uint8_t,
    mut len: size_t,
) {
    (*(*c).io).data = data;
    (*(*c).io).len = len;
    (*c).len = (*(*c).io).len as uint16_t;
    (*(*c).io).owner = OWNER_NET;
    pthread_cond_signal(&mut (*(*c).io).net_cond);
    pthread_mutex_unlock(&mut (*(*c).io).mutex);
}
