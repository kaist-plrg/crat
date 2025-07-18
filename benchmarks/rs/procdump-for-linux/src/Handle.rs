use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn SetEvent(Event: *mut Event) -> bool;
    fn InitNamedEvent(
        Event: *mut Event,
        IsManualReset: bool,
        InitialState: bool,
        Name: *mut libc::c_char,
    );
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_timedwait(__sem: *mut sem_t, __abstime: *const timespec) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub bTriggered: bool,
    pub bManualReset: bool,
    pub Name: [libc::c_char; 64],
    pub nWaiters: libc::c_int,
}
pub type EHandleType = libc::c_uint;
pub const SEMAPHORE: EHandleType = 1;
pub const EVENT: EHandleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub type_0: EHandleType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub event: Event,
    pub semaphore: sem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_result {
    pub retVal: libc::c_int,
    pub threadIndex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_args {
    pub handle: *mut Handle,
    pub coordinator: *mut coordinator,
    pub milliseconds: libc::c_int,
    pub retVal: libc::c_int,
    pub threadIndex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coordinator {
    pub condEventTriggered: pthread_cond_t,
    pub mutexEventTriggered: pthread_mutex_t,
    pub results: *mut thread_result,
    pub numberTriggered: libc::c_int,
    pub nWaiters: libc::c_int,
    pub stopIssued: libc::c_int,
    pub evtCanCleanUp: Handle,
    pub evtStartWaiting: Handle,
}
pub unsafe extern "C" fn WaitForSingleObject(
    mut Handle: *mut Handle,
    mut Milliseconds: libc::c_int,
) -> libc::c_int {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rc: libc::c_int = 0 as libc::c_int;
    if Milliseconds != -(1 as libc::c_int) {
        clock_gettime(0 as libc::c_int, &mut ts);
        ts.tv_sec += (Milliseconds / 1000 as libc::c_int) as libc::c_long;
        ts.tv_nsec
            += (Milliseconds % 1000 as libc::c_int * 1000000 as libc::c_int)
                as libc::c_long;
    }
    match (*Handle).type_0 as libc::c_uint {
        0 => {
            rc = pthread_mutex_lock(&mut (*Handle).c2rust_unnamed.event.mutex);
            if rc == 0 as libc::c_int {
                (*Handle).c2rust_unnamed.event.nWaiters += 1;
                (*Handle).c2rust_unnamed.event.nWaiters;
                while !(*Handle).c2rust_unnamed.event.bTriggered
                    && rc == 0 as libc::c_int
                {
                    rc = if Milliseconds == -(1 as libc::c_int) {
                        pthread_cond_wait(
                            &mut (*Handle).c2rust_unnamed.event.cond,
                            &mut (*Handle).c2rust_unnamed.event.mutex,
                        )
                    } else {
                        pthread_cond_timedwait(
                            &mut (*Handle).c2rust_unnamed.event.cond,
                            &mut (*Handle).c2rust_unnamed.event.mutex,
                            &mut ts,
                        )
                    };
                }
                (*Handle).c2rust_unnamed.event.nWaiters -= 1;
                (*Handle).c2rust_unnamed.event.nWaiters;
                if (*Handle).c2rust_unnamed.event.nWaiters == 0 as libc::c_int
                    && !(*Handle).c2rust_unnamed.event.bManualReset
                {
                    (*Handle).c2rust_unnamed.event.bTriggered = 0 as libc::c_int != 0;
                }
                pthread_mutex_unlock(&mut (*Handle).c2rust_unnamed.event.mutex);
            }
        }
        1 => {
            rc = if Milliseconds == -(1 as libc::c_int) {
                sem_wait(&mut (*Handle).c2rust_unnamed.semaphore)
            } else {
                sem_timedwait(&mut (*Handle).c2rust_unnamed.semaphore, &mut ts)
            };
        }
        _ => {
            rc = -(1 as libc::c_int);
        }
    }
    return rc;
}
pub unsafe extern "C" fn WaiterThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut rc: libc::c_int = 0;
    let mut input: *mut thread_args = thread_args as *mut thread_args;
    rc = WaitForSingleObject(
        &mut (*(*input).coordinator).evtStartWaiting,
        2000 as libc::c_int,
    );
    rc != 0 as libc::c_int;
    if (*input).milliseconds == -(1 as libc::c_int) {
        loop {
            rc = WaitForSingleObject((*input).handle, 5000 as libc::c_int);
            if !((*(*input).coordinator).stopIssued == 0 && rc == 110 as libc::c_int) {
                break;
            }
        }
    } else {
        rc = WaitForSingleObject((*input).handle, (*input).milliseconds);
    }
    pthread_mutex_lock(&mut (*(*input).coordinator).mutexEventTriggered);
    let mut result: thread_result = {
        let mut init = thread_result {
            retVal: rc,
            threadIndex: (*input).threadIndex,
        };
        init
    };
    let fresh0 = (*(*input).coordinator).numberTriggered;
    (*(*input).coordinator)
        .numberTriggered = (*(*input).coordinator).numberTriggered + 1;
    *((*(*input).coordinator).results).offset(fresh0 as isize) = result;
    pthread_mutex_unlock(&mut (*(*input).coordinator).mutexEventTriggered);
    pthread_cond_signal(&mut (*(*input).coordinator).condEventTriggered);
    WaitForSingleObject(&mut (*(*input).coordinator).evtCanCleanUp, -(1 as libc::c_int));
    pthread_mutex_lock(&mut (*(*input).coordinator).mutexEventTriggered);
    (*(*input).coordinator).nWaiters -= 1;
    (*(*input).coordinator).nWaiters;
    if (*(*input).coordinator).nWaiters == 0 as libc::c_int {
        pthread_mutex_unlock(&mut (*(*input).coordinator).mutexEventTriggered);
        pthread_mutex_destroy(&mut (*(*input).coordinator).mutexEventTriggered);
        pthread_cond_destroy(&mut (*(*input).coordinator).condEventTriggered);
        free((*(*input).coordinator).results as *mut libc::c_void);
        free((*input).coordinator as *mut libc::c_void);
        free(input as *mut libc::c_void);
    } else {
        pthread_mutex_unlock(&mut (*(*input).coordinator).mutexEventTriggered);
        free(input as *mut libc::c_void);
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn WaitForMultipleObjects(
    mut Count: libc::c_int,
    mut Handles: *mut *mut Handle,
    mut WaitAll: bool,
    mut Milliseconds: libc::c_int,
) -> libc::c_int {
    let mut coordinator: *mut coordinator = 0 as *mut coordinator;
    let mut results: *mut thread_result = 0 as *mut thread_result;
    let mut threads: *mut pthread_t = 0 as *mut pthread_t;
    let mut thread_args: *mut *mut thread_args = 0 as *mut *mut thread_args;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut t: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut retVal: libc::c_int = 0;
    threads = malloc(
        (::std::mem::size_of::<pthread_t>() as libc::c_ulong)
            .wrapping_mul(Count as libc::c_ulong),
    ) as *mut pthread_t;
    thread_args = malloc(
        (::std::mem::size_of::<*mut thread_args>() as libc::c_ulong)
            .wrapping_mul(Count as libc::c_ulong),
    ) as *mut *mut thread_args;
    coordinator = malloc(::std::mem::size_of::<coordinator>() as libc::c_ulong)
        as *mut coordinator;
    if coordinator.is_null() {
        printf(
            b"ERROR: Failed to malloc in %s\n\0" as *const u8 as *const libc::c_char,
            b"src/Handle.c\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*coordinator).numberTriggered = 0 as libc::c_int;
    (*coordinator).stopIssued = 0 as libc::c_int;
    (*coordinator).evtCanCleanUp.type_0 = EVENT;
    (*coordinator).evtStartWaiting.type_0 = EVENT;
    InitNamedEvent(
        &mut (*coordinator).evtCanCleanUp.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"CanCleanUp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    InitNamedEvent(
        &mut (*coordinator).evtStartWaiting.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"StartWaiting\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pthread_cond_init(
        &mut (*coordinator).condEventTriggered,
        0 as *const pthread_condattr_t,
    );
    pthread_mutex_init(
        &mut (*coordinator).mutexEventTriggered,
        0 as *const pthread_mutexattr_t,
    );
    (*coordinator)
        .results = malloc(
        (::std::mem::size_of::<thread_result>() as libc::c_ulong)
            .wrapping_mul(Count as libc::c_ulong),
    ) as *mut thread_result;
    results = (*coordinator).results;
    if Milliseconds != -(1 as libc::c_int) {
        clock_gettime(0 as libc::c_int, &mut ts);
        ts.tv_sec += (Milliseconds / 1000 as libc::c_int) as libc::c_long;
        ts.tv_nsec
            += (Milliseconds % 1000 as libc::c_int * 1000000 as libc::c_int)
                as libc::c_long;
    }
    pthread_mutex_lock(&mut (*coordinator).mutexEventTriggered);
    t = 0 as libc::c_int;
    while t < Count {
        let ref mut fresh1 = *thread_args.offset(t as isize);
        *fresh1 = malloc(::std::mem::size_of::<thread_args>() as libc::c_ulong)
            as *mut thread_args;
        if (*thread_args.offset(t as isize)).is_null() {
            printf(
                b"ERROR: Failed to alloc in %s\n\0" as *const u8 as *const libc::c_char,
                b"src/Handle.c\0" as *const u8 as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        let ref mut fresh2 = (**thread_args.offset(t as isize)).handle;
        *fresh2 = *Handles.offset(t as isize);
        (**thread_args.offset(t as isize))
            .milliseconds = if Milliseconds == -(1 as libc::c_int) {
            Milliseconds
        } else {
            Milliseconds + 100 as libc::c_int
        };
        (**thread_args.offset(t as isize)).threadIndex = t;
        let ref mut fresh3 = (**thread_args.offset(t as isize)).coordinator;
        *fresh3 = coordinator;
        rc = pthread_create(
            &mut *threads.offset(t as isize),
            0 as *const pthread_attr_t,
            Some(
                WaiterThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            *thread_args.offset(t as isize) as *mut libc::c_void,
        );
        if rc != 0 {
            printf(
                b"ERROR: pthread_create failed in %s with error %d\n\0" as *const u8
                    as *const libc::c_char,
                b"src/Handle.c\0" as *const u8 as *const libc::c_char,
                rc,
            );
            exit(-(1 as libc::c_int));
        }
        t += 1;
        t;
    }
    (*coordinator).nWaiters = Count;
    SetEvent(&mut (*coordinator).evtStartWaiting.c2rust_unnamed.event);
    while (WaitAll as libc::c_int != 0 && (*coordinator).numberTriggered < Count
        || !WaitAll && (*coordinator).numberTriggered == 0 as libc::c_int)
        && rc == 0 as libc::c_int
    {
        if Milliseconds == -(1 as libc::c_int) {
            rc = pthread_cond_wait(
                &mut (*coordinator).condEventTriggered,
                &mut (*coordinator).mutexEventTriggered,
            );
            if !(rc != 0 as libc::c_int) {
                continue;
            }
            break;
        } else {
            rc = pthread_cond_timedwait(
                &mut (*coordinator).condEventTriggered,
                &mut (*coordinator).mutexEventTriggered,
                &mut ts,
            );
            if rc != 0 as libc::c_int {
                break;
            }
        }
    }
    (*coordinator).stopIssued = 1 as libc::c_int;
    pthread_mutex_unlock(&mut (*coordinator).mutexEventTriggered);
    t = 0 as libc::c_int;
    while t < Count {
        pthread_detach(*threads.offset(t as isize));
        t += 1;
        t;
    }
    SetEvent(&mut (*coordinator).evtCanCleanUp.c2rust_unnamed.event);
    free(threads as *mut libc::c_void);
    free(thread_args as *mut libc::c_void);
    if rc != 0 {
        retVal = rc;
    } else {
        retVal = if WaitAll as libc::c_int != 0 {
            rc
        } else {
            (*results.offset(0 as libc::c_int as isize)).retVal
                + (*results.offset(0 as libc::c_int as isize)).threadIndex
        };
    }
    return retVal;
}
