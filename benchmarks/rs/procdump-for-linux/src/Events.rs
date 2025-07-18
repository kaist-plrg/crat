use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn Log(logLevel: LogLevel, message: *const libc::c_char, _: ...);
    fn DiagTrace(message: *const libc::c_char, _: ...);
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
pub type LogLevel = libc::c_uint;
pub const error: LogLevel = 4;
pub const crit: LogLevel = 3;
pub const warn: LogLevel = 2;
pub const info: LogLevel = 1;
pub const debug: LogLevel = 0;
pub unsafe extern "C" fn CreateEvent(
    mut IsManualReset: bool,
    mut InitialState: bool,
) -> *mut Event {
    let mut event: *mut Event = malloc(::std::mem::size_of::<Event>() as libc::c_ulong)
        as *mut Event;
    if event.is_null() {
        Log(error, b"INTERNAL_ERROR\0" as *const u8 as *const libc::c_char);
        DiagTrace(
            b"CreateEvent: failed memory allocation. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 24\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    InitEvent(event, IsManualReset, InitialState);
    return event;
}
pub unsafe extern "C" fn CreateNamedEvent(
    mut IsManualReset: bool,
    mut InitialState: bool,
    mut Name: *mut libc::c_char,
) -> *mut Event {
    let mut event: *mut Event = malloc(::std::mem::size_of::<Event>() as libc::c_ulong)
        as *mut Event;
    if event.is_null() {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"CreateNamedEvent: failed memory allocation. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 43\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    InitNamedEvent(event, IsManualReset, InitialState, Name);
    return event;
}
pub unsafe extern "C" fn InitEvent(
    mut Event: *mut Event,
    mut IsManualReset: bool,
    mut InitialState: bool,
) {
    InitNamedEvent(Event, IsManualReset, InitialState, 0 as *mut libc::c_char);
}
pub unsafe extern "C" fn InitNamedEvent(
    mut Event: *mut Event,
    mut IsManualReset: bool,
    mut InitialState: bool,
    mut Name: *mut libc::c_char,
) {
    static mut unamedEventId: libc::c_int = 0 as libc::c_int;
    pthread_mutex_init(&mut (*Event).mutex, 0 as *const pthread_mutexattr_t);
    if pthread_cond_init(&mut (*Event).cond, 0 as *const pthread_condattr_t)
        != 0 as libc::c_int
    {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"InitNamedEvent: failed pthread_cond_init. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 76\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*Event).bManualReset = IsManualReset;
    (*Event).bTriggered = InitialState;
    (*Event).nWaiters = 0 as libc::c_int;
    if Name.is_null() {
        unamedEventId += 1;
        sprintf(
            ((*Event).Name).as_mut_ptr(),
            b"Unnamed Event %d\0" as *const u8 as *const libc::c_char,
            unamedEventId,
        );
    } else if strlen(Name) >= 64 as libc::c_int as libc::c_ulong {
        strncpy(((*Event).Name).as_mut_ptr(), Name, 64 as libc::c_int as libc::c_ulong);
        (*Event)
            .Name[(64 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
    } else {
        strcpy(((*Event).Name).as_mut_ptr(), Name);
    };
}
pub unsafe extern "C" fn DestroyEvent(mut Event: *mut Event) {
    if pthread_cond_destroy(&mut (*Event).cond) != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"DestroyEvent: failed pthread_cond_destroy. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 104\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if pthread_mutex_destroy(&mut (*Event).mutex) != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"DestroyEvent: failed pthread_mutex_destroy. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 109\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
}
pub unsafe extern "C" fn SetEvent(mut Event: *mut Event) -> bool {
    let mut success: libc::c_int = 0 as libc::c_int;
    success = pthread_mutex_lock(&mut (*Event).mutex);
    if success == 0 as libc::c_int {
        (*Event).bTriggered = 1 as libc::c_int != 0;
        if (*Event).bManualReset as libc::c_int != 0 {
            pthread_cond_broadcast(&mut (*Event).cond);
        } else {
            pthread_cond_signal(&mut (*Event).cond);
        };
        pthread_mutex_unlock(&mut (*Event).mutex);
    } else {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"SetEvent: failed pthread_mutex_lock. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 135\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return success == 0 as libc::c_int;
}
pub unsafe extern "C" fn ResetEvent(mut Event: *mut Event) -> bool {
    let mut success: libc::c_int = 0 as libc::c_int;
    success = pthread_mutex_lock(&mut (*Event).mutex);
    if success == 0 as libc::c_int {
        (*Event).bTriggered = 0 as libc::c_int != 0;
        if pthread_mutex_unlock(&mut (*Event).mutex) != 0 as libc::c_int {
            Log(
                error,
                b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                    as *const u8 as *const libc::c_char,
            );
            DiagTrace(
                b"ResetEvent: failed pthread_mutex_unlock. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/Events.c, at line 157\0" as *const u8 as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    } else {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"ResetEvent: failed pthread_mutex_lock. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 163\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return success == 0 as libc::c_int;
}
