use ::libc;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type acl;
    pub type event_base;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn redisAsyncCommand(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisCallbackFn>,
        privdata: *mut libc::c_void,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn redisAsyncHandleTimeout(ac: *mut redisAsyncContext);
    fn redisAsyncHandleWrite(ac: *mut redisAsyncContext);
    fn redisAsyncHandleRead(ac: *mut redisAsyncContext);
    fn redisAsyncFree(ac: *mut redisAsyncContext);
    fn redisAsyncDisconnect(ac: *mut redisAsyncContext);
    fn redisAsyncSetDisconnectCallback(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisDisconnectCallback>,
    ) -> libc::c_int;
    fn redisAsyncSetConnectCallback(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisConnectCallback>,
    ) -> libc::c_int;
    fn redisAsyncConnectUnix(path: *const libc::c_char) -> *mut redisAsyncContext;
    fn redisAsyncConnect(
        ip: *const libc::c_char,
        port: libc::c_int,
    ) -> *mut redisAsyncContext;
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn slog(s: *mut server, level: log_level, body: *const libc::c_char, sz: size_t);
    fn event_base_set(_: *mut event_base, _: *mut event) -> libc::c_int;
    fn event_new(
        _: *mut event_base,
        _: libc::c_int,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
    ) -> *mut event;
    fn event_assign(
        _: *mut event,
        _: *mut event_base,
        _: libc::c_int,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn event_free(_: *mut event);
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createArray: Option::<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option::<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_int) -> *mut libc::c_void,
    >,
    pub freeObject: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub buf: *mut libc::c_char,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: libc::c_int,
    pub ridx: libc::c_int,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hiredisAllocFuncs {
    pub mallocFn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub callocFn: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub reallocFn: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub strdupFn: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >,
    pub freeFn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: libc::c_int,
    pub errstr: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_0,
    pub onDisconnect: Option::<redisDisconnectCallback>,
    pub onConnect: Option::<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed,
    pub push_cb: Option::<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub invalid: redisCallbackList,
    pub channels: *mut dict,
    pub patterns: *mut dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallbackList {
    pub head: *mut redisCallback,
    pub tail: *mut redisCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallback {
    pub next: *mut redisCallback,
    pub fn_0: Option::<redisCallbackFn>,
    pub pending_subs: libc::c_int,
    pub privdata: *mut libc::c_void,
}
pub type redisCallbackFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub type redisConnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    libc::c_int,
) -> ();
pub type redisDisconnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub data: *mut libc::c_void,
    pub addRead: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delRead: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub addWrite: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delWrite: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scheduleTimer: Option::<unsafe extern "C" fn(*mut libc::c_void, timeval) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContext {
    pub funcs: *const redisContextFuncs,
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub fd: redisFD,
    pub flags: libc::c_int,
    pub obuf: *mut libc::c_char,
    pub reader: *mut redisReader,
    pub connection_type: redisConnectionType,
    pub connect_timeout: *mut timeval,
    pub command_timeout: *mut timeval,
    pub tcp: C2RustUnnamed_2,
    pub unix_sock: C2RustUnnamed_1,
    pub saddr: *mut sockadr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option::<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub host: *mut libc::c_char,
    pub source_addr: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type redisConnectionType = libc::c_uint;
pub const REDIS_CONN_USERFD: redisConnectionType = 2;
pub const REDIS_CONN_UNIX: redisConnectionType = 1;
pub const REDIS_CONN_TCP: redisConnectionType = 0;
pub type redisFD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContextFuncs {
    pub free_privctx: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub async_read: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub async_write: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut redisContext, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<unsafe extern "C" fn(*mut redisContext) -> ssize_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReply {
    pub type_0: libc::c_int,
    pub integer: libc::c_longlong,
    pub dval: libc::c_double,
    pub len: size_t,
    pub str_0: *mut libc::c_char,
    pub vtype: [libc::c_char; 4],
    pub elements: size_t,
    pub element: *mut *mut redisReply,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub redis_host: *mut libc::c_char,
    pub redis_port: libc::c_int,
    pub redis_auth: *mut auth,
    pub http_host: *mut libc::c_char,
    pub http_port: libc::c_int,
    pub http_threads: libc::c_int,
    pub http_max_request_size: size_t,
    pub pool_size_per_thread: libc::c_int,
    pub daemonize: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub websockets: libc::c_int,
    pub database: libc::c_int,
    pub perms: *mut acl,
    pub user: uid_t,
    pub group: gid_t,
    pub logfile: *mut libc::c_char,
    pub verbosity: log_level,
    pub log_fsync: C2RustUnnamed_4,
    pub hiredis_opts: C2RustUnnamed_3,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub mode: log_fsync_mode,
    pub period_millis: libc::c_int,
}
pub type log_fsync_mode = libc::c_uint;
pub const LOG_FSYNC_ALL: log_fsync_mode = 2;
pub const LOG_FSYNC_MILLIS: log_fsync_mode = 1;
pub const LOG_FSYNC_AUTO: log_fsync_mode = 0;
pub type log_level = libc::c_uint;
pub const WEBDIS_TRACE: log_level = 8;
pub const WEBDIS_DEBUG: log_level = 4;
pub const WEBDIS_INFO: log_level = 3;
pub const WEBDIS_NOTICE: log_level = 2;
pub const WEBDIS_WARNING: log_level = 1;
pub const WEBDIS_ERROR: log_level = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: libc::c_int,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker {
    pub thread: pthread_t,
    pub base: *mut event_base,
    pub s: *mut server,
    pub link: [libc::c_int; 2],
    pub pool: *mut pool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub w: *mut worker,
    pub cfg: *mut conf,
    pub ac: *mut *const redisAsyncContext,
    pub count: libc::c_int,
    pub cur: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub fd: libc::c_int,
    pub ev: event,
    pub base: *mut event_base,
    pub cfg: *mut conf,
    pub w: *mut *mut worker,
    pub next_worker: libc::c_int,
    pub log: C2RustUnnamed_5,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub self_0: pid_t,
    pub fd: libc::c_int,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_11,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_6,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ev_io: C2RustUnnamed_9,
    pub ev_signal: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub ev_signal_next: C2RustUnnamed_8,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub ev_io_next: C2RustUnnamed_10,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ev_next_with_common_timeout: C2RustUnnamed_12,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_14,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_13,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub evcb_callback: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
    >,
    pub evcb_selfcb: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
    pub evcb_evfinalize: Option::<
        unsafe extern "C" fn(*mut event, *mut libc::c_void) -> (),
    >,
    pub evcb_cbfinalize: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_reconnect {
    pub ev: event,
    pub p: *mut pool,
    pub tv: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisLibeventEvents {
    pub context: *mut redisAsyncContext,
    pub ev: *mut event,
    pub base: *mut event_base,
    pub tv: timeval,
    pub flags: libc::c_short,
    pub state: libc::c_short,
}
pub type event_callback_fn = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
>;
#[inline]
unsafe extern "C" fn hi_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return (hiredisAllocFns.callocFn).unwrap()(nmemb, size);
}
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).unwrap()(ptr);
}
unsafe extern "C" fn redisLibeventDestroy(mut e: *mut redisLibeventEvents) {
    hi_free(e as *mut libc::c_void);
}
unsafe extern "C" fn redisLibeventHandler(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut e: *mut redisLibeventEvents = arg as *mut redisLibeventEvents;
    (*e).state = ((*e).state as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    if event as libc::c_int & 0x1 as libc::c_int != 0
        && (*e).state as libc::c_int & 0x1 as libc::c_int == 0 as libc::c_int
    {
        redisAsyncHandleTimeout((*e).context);
        if (*e).state as libc::c_int & 0x1 as libc::c_int != 0 {
            redisLibeventDestroy(e);
            return;
        }
    }
    if event as libc::c_int & 0x2 as libc::c_int != 0 && !((*e).context).is_null()
        && (*e).state as libc::c_int & 0x1 as libc::c_int == 0 as libc::c_int
    {
        redisAsyncHandleRead((*e).context);
        if (*e).state as libc::c_int & 0x1 as libc::c_int != 0 {
            redisLibeventDestroy(e);
            return;
        }
    }
    if event as libc::c_int & 0x4 as libc::c_int != 0 && !((*e).context).is_null()
        && (*e).state as libc::c_int & 0x1 as libc::c_int == 0 as libc::c_int
    {
        redisAsyncHandleWrite((*e).context);
        if (*e).state as libc::c_int & 0x1 as libc::c_int != 0 {
            redisLibeventDestroy(e);
            return;
        }
    }
    (*e).state = ((*e).state as libc::c_int & !(0x2 as libc::c_int)) as libc::c_short;
}
unsafe extern "C" fn redisLibeventUpdate(
    mut privdata: *mut libc::c_void,
    mut flag: libc::c_short,
    mut isRemove: libc::c_int,
) {
    let mut e: *mut redisLibeventEvents = privdata as *mut redisLibeventEvents;
    let mut tv: *const timeval = if (*e).tv.tv_sec != 0 || (*e).tv.tv_usec != 0 {
        &mut (*e).tv
    } else {
        0 as *mut timeval
    };
    if isRemove != 0 {
        if (*e).flags as libc::c_int & flag as libc::c_int == 0 as libc::c_int {
            return
        } else {
            (*e)
                .flags = ((*e).flags as libc::c_int & !(flag as libc::c_int))
                as libc::c_short;
        }
    } else if (*e).flags as libc::c_int & flag as libc::c_int != 0 {
        return
    } else {
        (*e).flags = ((*e).flags as libc::c_int | flag as libc::c_int) as libc::c_short;
    }
    event_del((*e).ev);
    event_assign(
        (*e).ev,
        (*e).base,
        (*(*e).context).c.fd,
        ((*e).flags as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            redisLibeventHandler
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        privdata,
    );
    event_add((*e).ev, tv);
}
unsafe extern "C" fn redisLibeventAddRead(mut privdata: *mut libc::c_void) {
    redisLibeventUpdate(privdata, 0x2 as libc::c_int as libc::c_short, 0 as libc::c_int);
}
unsafe extern "C" fn redisLibeventDelRead(mut privdata: *mut libc::c_void) {
    redisLibeventUpdate(privdata, 0x2 as libc::c_int as libc::c_short, 1 as libc::c_int);
}
unsafe extern "C" fn redisLibeventAddWrite(mut privdata: *mut libc::c_void) {
    redisLibeventUpdate(privdata, 0x4 as libc::c_int as libc::c_short, 0 as libc::c_int);
}
unsafe extern "C" fn redisLibeventDelWrite(mut privdata: *mut libc::c_void) {
    redisLibeventUpdate(privdata, 0x4 as libc::c_int as libc::c_short, 1 as libc::c_int);
}
unsafe extern "C" fn redisLibeventCleanup(mut privdata: *mut libc::c_void) {
    let mut e: *mut redisLibeventEvents = privdata as *mut redisLibeventEvents;
    if e.is_null() {
        return;
    }
    event_del((*e).ev);
    event_free((*e).ev);
    (*e).ev = 0 as *mut event;
    if (*e).state as libc::c_int & 0x2 as libc::c_int != 0 {
        (*e).state = ((*e).state as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
    } else {
        redisLibeventDestroy(e);
    };
}
unsafe extern "C" fn redisLibeventSetTimeout(
    mut privdata: *mut libc::c_void,
    mut tv: timeval,
) {
    let mut e: *mut redisLibeventEvents = privdata as *mut redisLibeventEvents;
    let mut flags: libc::c_short = (*e).flags;
    (*e).flags = 0 as libc::c_int as libc::c_short;
    (*e).tv = tv;
    redisLibeventUpdate(e as *mut libc::c_void, flags, 0 as libc::c_int);
}
unsafe extern "C" fn redisLibeventAttach(
    mut ac: *mut redisAsyncContext,
    mut base: *mut event_base,
) -> libc::c_int {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut e: *mut redisLibeventEvents = 0 as *mut redisLibeventEvents;
    if !((*ac).ev.data).is_null() {
        return -(1 as libc::c_int);
    }
    e = hi_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<redisLibeventEvents>() as libc::c_ulong,
    ) as *mut redisLibeventEvents;
    if e.is_null() {
        return -(1 as libc::c_int);
    }
    (*e).context = ac;
    (*ac)
        .ev
        .addRead = Some(
        redisLibeventAddRead as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .delRead = Some(
        redisLibeventDelRead as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .addWrite = Some(
        redisLibeventAddWrite as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .delWrite = Some(
        redisLibeventDelWrite as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .cleanup = Some(
        redisLibeventCleanup as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .scheduleTimer = Some(
        redisLibeventSetTimeout as unsafe extern "C" fn(*mut libc::c_void, timeval) -> (),
    );
    (*ac).ev.data = e as *mut libc::c_void;
    (*e)
        .ev = event_new(
        base,
        (*c).fd,
        (0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_short,
        Some(
            redisLibeventHandler
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        e as *mut libc::c_void,
    );
    (*e).base = base;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pool_new(
    mut w: *mut worker,
    mut count: libc::c_int,
) -> *mut pool {
    let mut p: *mut pool = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pool>() as libc::c_ulong,
    ) as *mut pool;
    (*p).count = count;
    (*p)
        .ac = calloc(
        count as libc::c_ulong,
        ::std::mem::size_of::<*mut redisAsyncContext>() as libc::c_ulong,
    ) as *mut *const redisAsyncContext;
    (*p).w = w;
    (*p).cfg = (*(*w).s).cfg;
    return p;
}
pub unsafe extern "C" fn pool_free_context(mut ac: *mut redisAsyncContext) {
    if !ac.is_null() {
        redisAsyncDisconnect(ac);
    }
}
unsafe extern "C" fn pool_on_connect(
    mut ac: *const redisAsyncContext,
    mut status: libc::c_int,
) {
    let mut p: *mut pool = (*ac).data as *mut pool;
    let mut i: libc::c_int = 0 as libc::c_int;
    if p.is_null() || status == -(1 as libc::c_int) || (*ac).err != 0 {
        if !p.is_null() {
            pool_schedule_reconnect(p);
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < (*p).count {
        if (*((*p).ac).offset(i as isize)).is_null() {
            let ref mut fresh0 = *((*p).ac).offset(i as isize);
            *fresh0 = ac;
            return;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn pool_can_connect(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut ptr: *mut libc::c_void,
) {
    let mut pr: *mut pool_reconnect = ptr as *mut pool_reconnect;
    let mut p: *mut pool = (*pr).p;
    free(pr as *mut libc::c_void);
    pool_connect(p, (*(*p).cfg).database, 1 as libc::c_int);
}
unsafe extern "C" fn pool_schedule_reconnect(mut p: *mut pool) {
    let mut pr: *mut pool_reconnect = malloc(
        ::std::mem::size_of::<pool_reconnect>() as libc::c_ulong,
    ) as *mut pool_reconnect;
    (*pr).p = p;
    (*pr).tv.tv_sec = 0 as libc::c_int as __time_t;
    (*pr).tv.tv_usec = (100 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
    event_set(
        &mut (*pr).ev,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            pool_can_connect
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        pr as *mut libc::c_void,
    );
    event_base_set((*(*p).w).base, &mut (*pr).ev);
    event_add(&mut (*pr).ev, &mut (*pr).tv);
}
unsafe extern "C" fn pool_on_disconnect(
    mut ac: *const redisAsyncContext,
    mut status: libc::c_int,
) {
    let mut p: *mut pool = (*ac).data as *mut pool;
    let mut i: libc::c_int = 0 as libc::c_int;
    if p.is_null() {
        return;
    }
    if status != 0 as libc::c_int {
        let mut format: [libc::c_char; 24] = *::std::mem::transmute::<
            &[u8; 24],
            &mut [libc::c_char; 24],
        >(b"Error disconnecting: %s\0");
        let mut msg_sz: size_t = (::std::mem::size_of::<[libc::c_char; 24]>()
            as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if !ac.is_null() && !((*ac).errstr).is_null() {
                    strlen((*ac).errstr)
                } else {
                    6 as libc::c_int as libc::c_ulong
                }),
            );
        let mut log_msg: *mut libc::c_char = calloc(
            msg_sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        if !log_msg.is_null() {
            snprintf(
                log_msg,
                msg_sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
                format.as_mut_ptr(),
                if !ac.is_null() && !((*ac).errstr).is_null() {
                    (*ac).errstr as *const libc::c_char
                } else {
                    b"(null)\0" as *const u8 as *const libc::c_char
                },
            );
            slog(
                (*(*p).w).s,
                WEBDIS_ERROR,
                log_msg,
                msg_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            free(log_msg as *mut libc::c_void);
        }
    }
    i = 0 as libc::c_int;
    while i < (*p).count {
        if *((*p).ac).offset(i as isize) == ac {
            let ref mut fresh1 = *((*p).ac).offset(i as isize);
            *fresh1 = 0 as *const redisAsyncContext;
            break;
        } else {
            i += 1;
            i;
        }
    }
    pool_schedule_reconnect(p);
}
unsafe extern "C" fn pool_log_auth(
    mut s: *mut server,
    mut level: log_level,
    mut format: *const libc::c_char,
    mut format_len: size_t,
    mut str: *const libc::c_char,
) {
    let mut msg_size: size_t = format_len
        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (if !str.is_null() {
                strlen(str)
            } else {
                6 as libc::c_int as libc::c_ulong
            }),
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut msg: *mut libc::c_char = calloc(1 as libc::c_int as libc::c_ulong, msg_size)
        as *mut libc::c_char;
    if !msg.is_null() {
        snprintf(
            msg,
            msg_size,
            format,
            if !str.is_null() {
                str
            } else {
                b"(null)\0" as *const u8 as *const libc::c_char
            },
        );
        slog(s, level, msg, msg_size.wrapping_sub(1 as libc::c_int as libc::c_ulong));
        free(msg as *mut libc::c_void);
    }
}
unsafe extern "C" fn pool_on_auth_complete(
    mut c: *mut redisAsyncContext,
    mut r: *mut libc::c_void,
    mut data: *mut libc::c_void,
) {
    let mut reply: *mut redisReply = r as *mut redisReply;
    let mut p: *mut pool = data as *mut pool;
    let err_format: [libc::c_char; 26] = *::std::mem::transmute::<
        &[u8; 26],
        &[libc::c_char; 26],
    >(b"Authentication failed: %s\0");
    let ok_format: [libc::c_char; 29] = *::std::mem::transmute::<
        &[u8; 29],
        &[libc::c_char; 29],
    >(b"Authentication succeeded: %s\0");
    let mut s: *mut server = (*(*p).w).s;
    if reply.is_null() {
        return;
    }
    pthread_mutex_lock(&mut (*s).auth_log_mutex);
    if (*s).auth_logged != 0 {
        pthread_mutex_unlock(&mut (*s).auth_log_mutex);
        return;
    }
    if (*reply).type_0 == 6 as libc::c_int {
        pool_log_auth(
            s,
            WEBDIS_ERROR,
            err_format.as_ptr(),
            (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            (*reply).str_0,
        );
    } else if (*reply).type_0 == 5 as libc::c_int {
        pool_log_auth(
            s,
            WEBDIS_INFO,
            ok_format.as_ptr(),
            (::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            (*reply).str_0,
        );
    }
    (*s).auth_logged += 1;
    (*s).auth_logged;
    pthread_mutex_unlock(&mut (*s).auth_log_mutex);
}
pub unsafe extern "C" fn pool_connect(
    mut p: *mut pool,
    mut db_num: libc::c_int,
    mut attach: libc::c_int,
) -> *mut redisAsyncContext {
    let mut ac: *mut redisAsyncContext = 0 as *mut redisAsyncContext;
    if *((*(*p).cfg).redis_host).offset(0 as libc::c_int as isize) as libc::c_int
        == '/' as i32
    {
        ac = redisAsyncConnectUnix((*(*p).cfg).redis_host);
    } else {
        ac = redisAsyncConnect((*(*p).cfg).redis_host, (*(*p).cfg).redis_port);
    }
    if attach != 0 {
        (*ac).data = p as *mut libc::c_void;
    } else {
        (*ac).data = 0 as *mut libc::c_void;
    }
    if (*ac).err != 0 {
        let mut msg: [libc::c_char; 22] = *::std::mem::transmute::<
            &[u8; 22],
            &mut [libc::c_char; 22],
        >(b"Connection failed: %s\0");
        let mut errlen: size_t = strlen((*ac).errstr);
        let mut err: *mut libc::c_char = malloc(
            (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                .wrapping_add(errlen),
        ) as *mut libc::c_char;
        if !err.is_null() {
            let mut sz: size_t = sprintf(err, msg.as_mut_ptr(), (*ac).errstr) as size_t;
            slog((*(*p).w).s, WEBDIS_ERROR, err, sz);
            free(err as *mut libc::c_void);
        }
        redisAsyncFree(ac);
        pool_schedule_reconnect(p);
        return 0 as *mut redisAsyncContext;
    }
    redisLibeventAttach(ac, (*(*p).w).base);
    redisAsyncSetConnectCallback(
        ac,
        Some(
            pool_on_connect
                as unsafe extern "C" fn(*const redisAsyncContext, libc::c_int) -> (),
        ),
    );
    redisAsyncSetDisconnectCallback(
        ac,
        Some(
            pool_on_disconnect
                as unsafe extern "C" fn(*const redisAsyncContext, libc::c_int) -> (),
        ),
    );
    if !((*(*p).cfg).redis_auth).is_null() {
        if (*(*(*p).cfg).redis_auth).use_legacy_auth != 0 {
            redisAsyncCommand(
                ac,
                Some(
                    pool_on_auth_complete
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                p as *mut libc::c_void,
                b"AUTH %s\0" as *const u8 as *const libc::c_char,
                (*(*(*p).cfg).redis_auth).password,
            );
        } else {
            redisAsyncCommand(
                ac,
                Some(
                    pool_on_auth_complete
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                p as *mut libc::c_void,
                b"AUTH %s %s\0" as *const u8 as *const libc::c_char,
                (*(*(*p).cfg).redis_auth).username,
                (*(*(*p).cfg).redis_auth).password,
            );
        }
    }
    if db_num != 0 {
        redisAsyncCommand(
            ac,
            None,
            0 as *mut libc::c_void,
            b"SELECT %d\0" as *const u8 as *const libc::c_char,
            db_num,
        );
    }
    return ac;
}
pub unsafe extern "C" fn pool_get_context(mut p: *mut pool) -> *const redisAsyncContext {
    let fresh2 = (*p).cur;
    (*p).cur = (*p).cur + 1;
    let mut orig: libc::c_int = fresh2;
    loop {
        (*p).cur += 1;
        (*p).cur;
        (*p).cur %= (*p).count;
        if !(*((*p).ac).offset((*p).cur as isize)).is_null() {
            return *((*p).ac).offset((*p).cur as isize);
        }
        if !((*p).cur != orig) {
            break;
        }
    }
    return 0 as *const redisAsyncContext;
}
