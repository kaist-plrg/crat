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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __int64_t = libc::c_long;
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
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ktf_worker_t {
    pub t: *mut kt_for_t,
    pub i: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kt_for_t {
    pub n_threads: libc::c_int,
    pub n: libc::c_long,
    pub w: *mut ktf_worker_t,
    pub func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
    >,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ktp_t {
    pub shared: *mut libc::c_void,
    pub func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub index: int64_t,
    pub n_workers: libc::c_int,
    pub n_steps: libc::c_int,
    pub workers: *mut ktp_worker_t,
    pub mutex: pthread_mutex_t,
    pub cv: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ktp_worker_t {
    pub pl: *mut ktp_t,
    pub index: int64_t,
    pub step: libc::c_int,
    pub data: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn steal_work(mut t: *mut kt_for_t) -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut min_i: libc::c_int = -(1 as libc::c_int);
    let mut k: libc::c_long = 0;
    let mut min: libc::c_long = 9223372036854775807 as libc::c_long;
    i = 0 as libc::c_int;
    while i < (*t).n_threads {
        if min > (*((*t).w).offset(i as isize)).i {
            min = (*((*t).w).offset(i as isize)).i;
            min_i = i;
        }
        i += 1;
        i;
    }
    k = ::std::intrinsics::atomic_xadd::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(
        &mut (*((*t).w).offset(min_i as isize)).i,
        (*t).n_threads as libc::c_long,
    );
    return if k >= (*t).n { -(1 as libc::c_int) as libc::c_long } else { k };
}
unsafe extern "C" fn ktf_worker(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut ktf_worker_t = data as *mut ktf_worker_t;
    let mut i: libc::c_long = 0;
    loop {
        i = ::std::intrinsics::atomic_xadd::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(
            &mut (*w).i,
            (*(*w).t).n_threads as libc::c_long,
        );
        if i >= (*(*w).t).n {
            break;
        }
        ((*(*w).t).func)
            .unwrap()(
            (*(*w).t).data,
            i,
            w.offset_from((*(*w).t).w) as libc::c_long as libc::c_int,
        );
    }
    loop {
        i = steal_work((*w).t);
        if !(i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        ((*(*w).t).func)
            .unwrap()(
            (*(*w).t).data,
            i,
            w.offset_from((*(*w).t).w) as libc::c_long as libc::c_int,
        );
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn kt_for(
    mut n_threads: libc::c_int,
    mut func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
    >,
    mut data: *mut libc::c_void,
    mut n: libc::c_long,
) {
    if n_threads > 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut t: kt_for_t = kt_for_t {
            n_threads: 0,
            n: 0,
            w: 0 as *mut ktf_worker_t,
            func: None,
            data: 0 as *mut libc::c_void,
        };
        let mut tid: *mut pthread_t = 0 as *mut pthread_t;
        t.func = func;
        t.data = data;
        t.n_threads = n_threads;
        t.n = n;
        t
            .w = calloc(
            n_threads as libc::c_ulong,
            ::std::mem::size_of::<ktf_worker_t>() as libc::c_ulong,
        ) as *mut ktf_worker_t;
        tid = calloc(
            n_threads as libc::c_ulong,
            ::std::mem::size_of::<pthread_t>() as libc::c_ulong,
        ) as *mut pthread_t;
        i = 0 as libc::c_int;
        while i < n_threads {
            let ref mut fresh0 = (*(t.w).offset(i as isize)).t;
            *fresh0 = &mut t;
            (*(t.w).offset(i as isize)).i = i as libc::c_long;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < n_threads {
            pthread_create(
                &mut *tid.offset(i as isize),
                0 as *const pthread_attr_t,
                Some(
                    ktf_worker
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut *(t.w).offset(i as isize) as *mut ktf_worker_t as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < n_threads {
            pthread_join(*tid.offset(i as isize), 0 as *mut *mut libc::c_void);
            i += 1;
            i;
        }
        free(tid as *mut libc::c_void);
        free(t.w as *mut libc::c_void);
    } else {
        let mut j: libc::c_long = 0;
        j = 0 as libc::c_int as libc::c_long;
        while j < n {
            func.unwrap()(data, j, 0 as libc::c_int);
            j += 1;
            j;
        }
    };
}
unsafe extern "C" fn ktp_worker(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut ktp_worker_t = data as *mut ktp_worker_t;
    let mut p: *mut ktp_t = (*w).pl;
    while (*w).step < (*p).n_steps {
        pthread_mutex_lock(&mut (*p).mutex);
        loop {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*p).n_workers {
                if !(w == &mut *((*p).workers).offset(i as isize) as *mut ktp_worker_t) {
                    if (*((*p).workers).offset(i as isize)).step <= (*w).step
                        && (*((*p).workers).offset(i as isize)).index < (*w).index
                    {
                        break;
                    }
                }
                i += 1;
                i;
            }
            if i == (*p).n_workers {
                break;
            }
            pthread_cond_wait(&mut (*p).cv, &mut (*p).mutex);
        }
        pthread_mutex_unlock(&mut (*p).mutex);
        (*w)
            .data = ((*p).func)
            .unwrap()(
            (*p).shared,
            (*w).step,
            if (*w).step != 0 { (*w).data } else { 0 as *mut libc::c_void },
        );
        pthread_mutex_lock(&mut (*p).mutex);
        (*w)
            .step = if (*w).step == (*p).n_steps - 1 as libc::c_int
            || !((*w).data).is_null()
        {
            ((*w).step + 1 as libc::c_int) % (*p).n_steps
        } else {
            (*p).n_steps
        };
        if (*w).step == 0 as libc::c_int {
            let fresh1 = (*p).index;
            (*p).index = (*p).index + 1;
            (*w).index = fresh1;
        }
        pthread_cond_broadcast(&mut (*p).cv);
        pthread_mutex_unlock(&mut (*p).mutex);
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn kt_pipeline(
    mut n_threads: libc::c_int,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    mut shared_data: *mut libc::c_void,
    mut n_steps: libc::c_int,
) {
    let mut aux: ktp_t = ktp_t {
        shared: 0 as *mut libc::c_void,
        func: None,
        index: 0,
        n_workers: 0,
        n_steps: 0,
        workers: 0 as *mut ktp_worker_t,
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
        cv: pthread_cond_t {
            __data: __pthread_cond_s {
                c2rust_unnamed: C2RustUnnamed_1 { __wseq: 0 },
                c2rust_unnamed_0: C2RustUnnamed { __g1_start: 0 },
                __g_refs: [0; 2],
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
            },
        },
    };
    let mut tid: *mut pthread_t = 0 as *mut pthread_t;
    let mut i: libc::c_int = 0;
    if n_threads < 1 as libc::c_int {
        n_threads = 1 as libc::c_int;
    }
    aux.n_workers = n_threads;
    aux.n_steps = n_steps;
    aux.func = func;
    aux.shared = shared_data;
    aux.index = 0 as libc::c_int as int64_t;
    pthread_mutex_init(&mut aux.mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut aux.cv, 0 as *const pthread_condattr_t);
    aux
        .workers = calloc(
        n_threads as libc::c_ulong,
        ::std::mem::size_of::<ktp_worker_t>() as libc::c_ulong,
    ) as *mut ktp_worker_t;
    i = 0 as libc::c_int;
    while i < n_threads {
        let mut w: *mut ktp_worker_t = &mut *(aux.workers).offset(i as isize)
            as *mut ktp_worker_t;
        (*w).step = 0 as libc::c_int;
        (*w).pl = &mut aux;
        (*w).data = 0 as *mut libc::c_void;
        let fresh2 = aux.index;
        aux.index = aux.index + 1;
        (*w).index = fresh2;
        i += 1;
        i;
    }
    tid = calloc(
        n_threads as libc::c_ulong,
        ::std::mem::size_of::<pthread_t>() as libc::c_ulong,
    ) as *mut pthread_t;
    i = 0 as libc::c_int;
    while i < n_threads {
        pthread_create(
            &mut *tid.offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                ktp_worker
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut *(aux.workers).offset(i as isize) as *mut ktp_worker_t
                as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n_threads {
        pthread_join(*tid.offset(i as isize), 0 as *mut *mut libc::c_void);
        i += 1;
        i;
    }
    free(tid as *mut libc::c_void);
    free(aux.workers as *mut libc::c_void);
    pthread_mutex_destroy(&mut aux.mutex);
    pthread_cond_destroy(&mut aux.cv);
}
