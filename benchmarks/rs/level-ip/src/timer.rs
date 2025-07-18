use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn socket_debug();
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
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
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
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
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_RWLOCK_DEFAULT_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer {
    pub list: list_head,
    pub refcnt: libc::c_int,
    pub expires: uint32_t,
    pub cancelled: libc::c_int,
    pub handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub arg: *mut libc::c_void,
    pub lock: pthread_mutex_t,
}
#[inline]
unsafe extern "C" fn list_add_tail(mut new: *mut list_head, mut head: *mut list_head) {
    (*(*head).prev).next = new;
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*head).prev = new;
}
#[inline]
unsafe extern "C" fn list_del(mut elem: *mut list_head) {
    let mut prev: *mut list_head = (*elem).prev;
    let mut next: *mut list_head = (*elem).next;
    (*prev).next = next;
    (*next).prev = prev;
}
static mut timers: list_head = unsafe {
    {
        let mut init = list_head {
            next: &timers as *const list_head as *mut list_head,
            prev: &timers as *const list_head as *mut list_head,
        };
        init
    }
};
static mut tick: libc::c_int = 0 as libc::c_int;
static mut lock: pthread_mutex_t = pthread_mutex_t {
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
static mut rwlock: pthread_rwlock_t = pthread_rwlock_t {
    __data: {
        let mut init = __pthread_rwlock_arch_t {
            __readers: 0 as libc::c_int as libc::c_uint,
            __writers: 0 as libc::c_int as libc::c_uint,
            __wrphase_futex: 0 as libc::c_int as libc::c_uint,
            __writers_futex: 0 as libc::c_int as libc::c_uint,
            __pad3: 0 as libc::c_int as libc::c_uint,
            __pad4: 0 as libc::c_int as libc::c_uint,
            __cur_writer: 0 as libc::c_int,
            __shared: 0 as libc::c_int,
            __rwelision: 0 as libc::c_int as libc::c_schar,
            __pad1: [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ],
            __pad2: 0 as libc::c_int as libc::c_ulong,
            __flags: PTHREAD_RWLOCK_DEFAULT_NP as libc::c_int as libc::c_uint,
        };
        init
    },
};
unsafe extern "C" fn timer_debug() {}
unsafe extern "C" fn timer_free(mut t: *mut timer) {
    pthread_mutex_destroy(&mut (*t).lock);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn timer_alloc() -> *mut timer {
    let mut t: *mut timer = calloc(
        ::std::mem::size_of::<timer>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut timer;
    pthread_mutex_init(&mut (*t).lock, 0 as *const pthread_mutexattr_t);
    return t;
}
unsafe extern "C" fn timers_tick() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut t: *mut timer = 0 as *mut timer;
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = pthread_mutex_lock(&mut lock);
    if rc != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Timer tick lock not acquired: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(rc),
        );
        return;
    }
    item = timers.next;
    tmp = (*item).next;
    while item != &mut timers as *mut list_head {
        if !item.is_null() {
            t = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut timer;
            rc = pthread_mutex_trylock(&mut (*t).lock);
            if rc != 0 as libc::c_int {
                if rc != 16 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Timer free mutex lock: %s\n\0" as *const u8
                            as *const libc::c_char,
                        strerror(rc),
                    );
                }
            } else {
                if (*t).cancelled == 0 && (*t).expires < tick as libc::c_uint {
                    (*t).cancelled = 1 as libc::c_int;
                    let mut th: pthread_t = 0;
                    pthread_create(
                        &mut th,
                        0 as *const pthread_attr_t,
                        (*t).handler,
                        (*t).arg,
                    );
                }
                if (*t).cancelled != 0 && (*t).refcnt == 0 as libc::c_int {
                    list_del(&mut (*t).list);
                    pthread_mutex_unlock(&mut (*t).lock);
                    timer_free(t);
                } else {
                    pthread_mutex_unlock(&mut (*t).lock);
                }
            }
        }
        item = tmp;
        tmp = (*item).next;
    }
    pthread_mutex_unlock(&mut lock);
}
pub unsafe extern "C" fn timer_oneshot(
    mut expire: uint32_t,
    mut handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) {
    let mut t: *mut timer = timer_alloc();
    let mut tick_0: libc::c_int = timer_get_tick();
    (*t).refcnt = 0 as libc::c_int;
    (*t).expires = (tick_0 as libc::c_uint).wrapping_add(expire);
    (*t).cancelled = 0 as libc::c_int;
    if (*t).expires < tick_0 as libc::c_uint {
        fprintf(
            stderr,
            b"ERR: Timer expiry integer wrap around\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*t).handler = handler;
    (*t).arg = arg;
    pthread_mutex_lock(&mut lock);
    list_add_tail(&mut (*t).list, &mut timers);
    pthread_mutex_unlock(&mut lock);
}
pub unsafe extern "C" fn timer_add(
    mut expire: uint32_t,
    mut handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> *mut timer {
    let mut t: *mut timer = timer_alloc();
    let mut tick_0: libc::c_int = timer_get_tick();
    (*t).refcnt = 1 as libc::c_int;
    (*t).expires = (tick_0 as libc::c_uint).wrapping_add(expire);
    (*t).cancelled = 0 as libc::c_int;
    if (*t).expires < tick_0 as libc::c_uint {
        fprintf(
            stderr,
            b"ERR: Timer expiry integer wrap around\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*t).handler = handler;
    (*t).arg = arg;
    pthread_mutex_lock(&mut lock);
    list_add_tail(&mut (*t).list, &mut timers);
    pthread_mutex_unlock(&mut lock);
    return t;
}
pub unsafe extern "C" fn timer_release(mut t: *mut timer) {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if t.is_null() {
        return;
    }
    rc = pthread_mutex_lock(&mut (*t).lock);
    if rc != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Timer release lock: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(rc),
        );
        return;
    }
    (*t).refcnt -= 1;
    (*t).refcnt;
    pthread_mutex_unlock(&mut (*t).lock);
}
pub unsafe extern "C" fn timer_cancel(mut t: *mut timer) {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if t.is_null() {
        return;
    }
    rc = pthread_mutex_lock(&mut (*t).lock);
    if rc != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Timer cancel lock: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(rc),
        );
        return;
    }
    (*t).refcnt -= 1;
    (*t).refcnt;
    (*t).cancelled = 1 as libc::c_int;
    pthread_mutex_unlock(&mut (*t).lock);
}
pub unsafe extern "C" fn timers_start() -> *mut libc::c_void {
    loop {
        if usleep(10000 as libc::c_int as __useconds_t) != 0 as libc::c_int {
            perror(b"Timer usleep\0" as *const u8 as *const libc::c_char);
        }
        pthread_rwlock_wrlock(&mut rwlock);
        tick += 10 as libc::c_int;
        pthread_rwlock_unlock(&mut rwlock);
        timers_tick();
        if tick % 5000 as libc::c_int == 0 as libc::c_int {
            socket_debug();
            timer_debug();
        }
    };
}
pub unsafe extern "C" fn timer_get_tick() -> libc::c_int {
    let mut copy: libc::c_int = 0 as libc::c_int;
    pthread_rwlock_rdlock(&mut rwlock);
    copy = tick;
    pthread_rwlock_unlock(&mut rwlock);
    return copy;
}
