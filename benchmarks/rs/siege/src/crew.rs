use ::libc;
extern "C" {
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
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work {
    pub routine: Option::<unsafe extern "C" fn() -> ()>,
    pub arg: *mut libc::c_void,
    pub next: *mut work,
}
pub type WORK = work;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CREW_T {
    pub size: libc::c_int,
    pub maxsize: libc::c_int,
    pub cursize: libc::c_int,
    pub total: libc::c_int,
    pub head: *mut WORK,
    pub tail: *mut WORK,
    pub block: BOOLEAN,
    pub closed: BOOLEAN,
    pub shutdown: BOOLEAN,
    pub threads: *mut pthread_t,
    pub lock: pthread_mutex_t,
    pub not_empty: pthread_cond_t,
    pub not_full: pthread_cond_t,
    pub empty: pthread_cond_t,
}
pub type CREW = *mut CREW_T;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub unsafe extern "C" fn new_crew(
    mut size: libc::c_int,
    mut maxsize: libc::c_int,
    mut block: BOOLEAN,
) -> CREW {
    let mut x: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut this: CREW = 0 as *mut CREW_T;
    this = calloc(
        ::std::mem::size_of::<CREW_T>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as CREW;
    if this.is_null() {
        return 0 as CREW;
    }
    (*this)
        .threads = malloc(
        (::std::mem::size_of::<pthread_t>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut pthread_t;
    if ((*this).threads).is_null() {
        return 0 as CREW;
    }
    (*this).size = size;
    (*this).maxsize = maxsize;
    (*this).cursize = 0 as libc::c_int;
    (*this).total = 0 as libc::c_int;
    (*this).block = block;
    (*this).head = 0 as *mut WORK;
    (*this).tail = 0 as *mut WORK;
    (*this).closed = boolean_false;
    (*this).shutdown = boolean_false;
    c = pthread_mutex_init(&mut (*this).lock, 0 as *const pthread_mutexattr_t);
    if c != 0 as libc::c_int {
        return 0 as CREW;
    }
    c = pthread_cond_init(&mut (*this).not_empty, 0 as *const pthread_condattr_t);
    if c != 0 as libc::c_int {
        return 0 as CREW;
    }
    c = pthread_cond_init(&mut (*this).not_full, 0 as *const pthread_condattr_t);
    if c != 0 as libc::c_int {
        return 0 as CREW;
    }
    c = pthread_cond_init(&mut (*this).empty, 0 as *const pthread_condattr_t);
    if c != 0 as libc::c_int {
        return 0 as CREW;
    }
    x = 0 as libc::c_int;
    while x != size {
        c = pthread_create(
            &mut *((*this).threads).offset(x as isize),
            0 as *const pthread_attr_t,
            Some(
                crew_thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            this as *mut libc::c_void,
        );
        if c != 0 as libc::c_int {
            match *__errno_location() {
                22 => {
                    NOTIFY(
                        ERROR,
                        b"Error creating additional threads %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"crew.c\0" as *const u8 as *const libc::c_char,
                        86 as libc::c_int,
                    );
                }
                1 => {
                    NOTIFY(
                        ERROR,
                        b"Inadequate permission to create pool %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"crew.c\0" as *const u8 as *const libc::c_char,
                        87 as libc::c_int,
                    );
                }
                11 => {
                    NOTIFY(
                        ERROR,
                        b"Inadequate resources to create pool %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"crew.c\0" as *const u8 as *const libc::c_char,
                        88 as libc::c_int,
                    );
                }
                12 => {
                    NOTIFY(
                        ERROR,
                        b"Exceeded thread limit for this system %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"crew.c\0" as *const u8 as *const libc::c_char,
                        89 as libc::c_int,
                    );
                }
                _ => {
                    NOTIFY(
                        ERROR,
                        b"Unknown error building thread pool %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"crew.c\0" as *const u8 as *const libc::c_char,
                        90 as libc::c_int,
                    );
                }
            }
            return 0 as CREW;
        }
        x += 1;
        x;
    }
    return this;
}
unsafe extern "C" fn crew_thread(mut crew: *mut libc::c_void) -> *mut libc::c_void {
    let mut c: libc::c_int = 0;
    let mut workptr: *mut WORK = 0 as *mut WORK;
    let mut this: CREW = crew as CREW;
    while boolean_true as libc::c_int != 0 {
        c = pthread_mutex_lock(&mut (*this).lock);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"mutex lock\0" as *const u8 as *const libc::c_char);
        }
        while (*this).cursize == 0 as libc::c_int && (*this).shutdown as u64 == 0 {
            c = pthread_cond_wait(&mut (*this).not_empty, &mut (*this).lock);
            if c != 0 as libc::c_int {
                NOTIFY(FATAL, b"pthread wait\0" as *const u8 as *const libc::c_char);
            }
        }
        if (*this).shutdown as libc::c_uint
            == boolean_true as libc::c_int as libc::c_uint
        {
            c = pthread_mutex_unlock(&mut (*this).lock);
            if c != 0 as libc::c_int {
                NOTIFY(FATAL, b"mutex unlock\0" as *const u8 as *const libc::c_char);
            }
            pthread_exit(0 as *mut libc::c_void);
        }
        workptr = (*this).head;
        (*this).cursize -= 1;
        (*this).cursize;
        if (*this).cursize == 0 as libc::c_int {
            (*this).tail = 0 as *mut WORK;
            (*this).head = (*this).tail;
        } else {
            (*this).head = (*workptr).next;
        }
        if (*this).block as libc::c_uint != 0
            && (*this).cursize == (*this).maxsize - 1 as libc::c_int
        {
            c = pthread_cond_broadcast(&mut (*this).not_full);
            if c != 0 as libc::c_int {
                NOTIFY(
                    FATAL,
                    b"pthread broadcast\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if (*this).cursize == 0 as libc::c_int {
            c = pthread_cond_signal(&mut (*this).empty);
            if c != 0 as libc::c_int {
                NOTIFY(FATAL, b"pthread signal\0" as *const u8 as *const libc::c_char);
            }
        }
        c = pthread_mutex_unlock(&mut (*this).lock);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread unlock\0" as *const u8 as *const libc::c_char);
        }
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(((*workptr).routine).unwrap())).unwrap())((*workptr).arg);
        xfree(workptr as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn crew_add(
    mut crew: CREW,
    mut routine: Option::<unsafe extern "C" fn() -> ()>,
    mut arg: *mut libc::c_void,
) -> BOOLEAN {
    let mut c: libc::c_int = 0;
    let mut workptr: *mut WORK = 0 as *mut WORK;
    c = pthread_mutex_lock(&mut (*crew).lock);
    if c != 0 as libc::c_int {
        NOTIFY(FATAL, b"pthread lock\0" as *const u8 as *const libc::c_char);
    }
    if (*crew).cursize == (*crew).maxsize && (*crew).block as u64 == 0 {
        c = pthread_mutex_unlock(&mut (*crew).lock);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread unlock\0" as *const u8 as *const libc::c_char);
        }
        return boolean_false;
    }
    while (*crew).cursize == (*crew).maxsize
        && !((*crew).shutdown as libc::c_uint != 0
            || (*crew).closed as libc::c_uint != 0)
    {
        c = pthread_cond_wait(&mut (*crew).not_full, &mut (*crew).lock);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread wait\0" as *const u8 as *const libc::c_char);
        }
    }
    if (*crew).shutdown as libc::c_uint != 0 || (*crew).closed as libc::c_uint != 0 {
        c = pthread_mutex_unlock(&mut (*crew).lock);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread unlock\0" as *const u8 as *const libc::c_char);
        }
        return boolean_false;
    }
    workptr = malloc(::std::mem::size_of::<WORK>() as libc::c_ulong) as *mut WORK;
    if workptr.is_null() {
        NOTIFY(FATAL, b"out of memory\0" as *const u8 as *const libc::c_char);
    }
    (*workptr).routine = routine;
    (*workptr).arg = arg;
    (*workptr).next = 0 as *mut work;
    if (*crew).cursize == 0 as libc::c_int {
        (*crew).head = workptr;
        (*crew).tail = (*crew).head;
        c = pthread_cond_broadcast(&mut (*crew).not_empty);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread signal\0" as *const u8 as *const libc::c_char);
        }
    } else {
        (*(*crew).tail).next = workptr;
        (*crew).tail = workptr;
    }
    (*crew).cursize += 1;
    (*crew).cursize;
    (*crew).total += 1;
    (*crew).total;
    c = pthread_mutex_unlock(&mut (*crew).lock);
    if c != 0 as libc::c_int {
        NOTIFY(FATAL, b"pthread unlock\0" as *const u8 as *const libc::c_char);
    }
    return boolean_true;
}
pub unsafe extern "C" fn crew_cancel(mut this: CREW) -> BOOLEAN {
    let mut x: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    size = (*this).size;
    crew_set_shutdown(this, boolean_true);
    x = 0 as libc::c_int;
    while x < size {
        pthread_cancel(*((*this).threads).offset(x as isize));
        x += 1;
        x;
    }
    return boolean_true;
}
pub unsafe extern "C" fn crew_join(
    mut crew: CREW,
    mut finish: BOOLEAN,
    mut payload: *mut *mut libc::c_void,
) -> BOOLEAN {
    let mut x: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = pthread_mutex_lock(&mut (*crew).lock);
    if c != 0 as libc::c_int {
        NOTIFY(FATAL, b"pthread lock\0" as *const u8 as *const libc::c_char);
    }
    if (*crew).closed as libc::c_uint != 0 || (*crew).shutdown as libc::c_uint != 0 {
        c = pthread_mutex_unlock(&mut (*crew).lock);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread unlock\0" as *const u8 as *const libc::c_char);
        }
        return boolean_false;
    }
    (*crew).closed = boolean_true;
    if finish as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        while (*crew).cursize != 0 as libc::c_int && (*crew).shutdown as u64 == 0 {
            let mut rc: libc::c_int = 0;
            let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut tp: timeval = timeval { tv_sec: 0, tv_usec: 0 };
            rc = gettimeofday(&mut tp, 0 as *mut libc::c_void);
            if rc != 0 as libc::c_int {
                perror(b"gettimeofday\0" as *const u8 as *const libc::c_char);
            }
            ts.tv_sec = tp.tv_sec + 60 as libc::c_int as libc::c_long;
            ts.tv_nsec = tp.tv_usec * 1000 as libc::c_int as libc::c_long;
            rc = pthread_cond_timedwait(&mut (*crew).empty, &mut (*crew).lock, &mut ts);
            if rc == 110 as libc::c_int {
                pthread_mutex_unlock(&mut (*crew).lock);
            }
            if rc != 0 as libc::c_int {
                NOTIFY(FATAL, b"pthread wait\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    (*crew).shutdown = boolean_true;
    c = pthread_mutex_unlock(&mut (*crew).lock);
    if c != 0 as libc::c_int {
        NOTIFY(FATAL, b"pthread_mutex_unlock\0" as *const u8 as *const libc::c_char);
    }
    c = pthread_cond_broadcast(&mut (*crew).not_empty);
    if c != 0 as libc::c_int {
        NOTIFY(FATAL, b"pthread broadcast\0" as *const u8 as *const libc::c_char);
    }
    c = pthread_cond_broadcast(&mut (*crew).not_full);
    if c != 0 as libc::c_int {
        NOTIFY(FATAL, b"pthread broadcast\0" as *const u8 as *const libc::c_char);
    }
    x = 0 as libc::c_int;
    while x < (*crew).size {
        c = pthread_join(*((*crew).threads).offset(x as isize), payload);
        if c != 0 as libc::c_int {
            NOTIFY(FATAL, b"pthread_join\0" as *const u8 as *const libc::c_char);
        }
        x += 1;
        x;
    }
    return boolean_true;
}
pub unsafe extern "C" fn crew_destroy(mut crew: CREW) {
    let mut workptr: *mut WORK = 0 as *mut WORK;
    xfree((*crew).threads as *mut libc::c_void);
    while !((*crew).head).is_null() {
        workptr = (*crew).head;
        (*crew).head = (*(*crew).head).next;
        xfree(workptr as *mut libc::c_void);
    }
    xfree(crew as *mut libc::c_void);
}
pub unsafe extern "C" fn crew_set_shutdown(mut this: CREW, mut shutdown: BOOLEAN) {
    (*this).shutdown = shutdown;
    pthread_cond_broadcast(&mut (*this).not_empty);
    pthread_cond_broadcast(&mut (*this).not_full);
    pthread_cond_broadcast(&mut (*this).empty);
}
pub unsafe extern "C" fn crew_get_size(mut this: CREW) -> libc::c_int {
    return (*this).size;
}
pub unsafe extern "C" fn crew_get_total(mut this: CREW) -> libc::c_int {
    return (*this).total;
}
pub unsafe extern "C" fn crew_get_shutdown(mut this: CREW) -> BOOLEAN {
    return (*this).shutdown;
}
