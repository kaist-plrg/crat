use ::libc;
extern "C" {
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub lastTime: time_t,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when_sec: libc::c_long,
    pub when_ms: libc::c_long,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub next: *mut aeTimeEvent,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeApiState {
    pub epfd: libc::c_int,
    pub events: *mut epoll_event,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type uint64_t = __uint64_t;
pub type uint32_t = __uint32_t;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLIN: EPOLL_EVENTS = 1;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
unsafe extern "C" fn aeApiCreate(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    let mut state: *mut aeApiState = zmalloc(
        ::std::mem::size_of::<aeApiState>() as libc::c_ulong,
    ) as *mut aeApiState;
    if state.is_null() {
        return -(1 as libc::c_int);
    }
    (*state)
        .events = zmalloc(
        (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
            .wrapping_mul((*eventLoop).setsize as libc::c_ulong),
    ) as *mut epoll_event;
    if ((*state).events).is_null() {
        zfree(state as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*state).epfd = epoll_create(1024 as libc::c_int);
    if (*state).epfd == -(1 as libc::c_int) {
        zfree((*state).events as *mut libc::c_void);
        zfree(state as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*eventLoop).apidata = state as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiName() -> *mut libc::c_char {
    return b"epoll\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
unsafe extern "C" fn aeApiResize(
    mut eventLoop: *mut aeEventLoop,
    mut setsize: libc::c_int,
) -> libc::c_int {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    (*state)
        .events = zrealloc(
        (*state).events as *mut libc::c_void,
        (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    ) as *mut epoll_event;
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiPoll(
    mut eventLoop: *mut aeEventLoop,
    mut tvp: *mut timeval,
) -> libc::c_int {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    let mut retval: libc::c_int = 0;
    let mut numevents: libc::c_int = 0 as libc::c_int;
    retval = epoll_wait(
        (*state).epfd,
        (*state).events,
        (*eventLoop).setsize,
        (if !tvp.is_null() {
            (*tvp).tv_sec * 1000 as libc::c_int as libc::c_long
                + (*tvp).tv_usec / 1000 as libc::c_int as libc::c_long
        } else {
            -(1 as libc::c_int) as libc::c_long
        }) as libc::c_int,
    );
    if retval > 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        numevents = retval;
        j = 0 as libc::c_int;
        while j < numevents {
            let mut mask: libc::c_int = 0 as libc::c_int;
            let mut e: *mut epoll_event = ((*state).events).offset(j as isize);
            if (*e).events & EPOLLIN as libc::c_int as libc::c_uint != 0 {
                mask |= 1 as libc::c_int;
            }
            if (*e).events & EPOLLOUT as libc::c_int as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            if (*e).events & EPOLLERR as libc::c_int as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            if (*e).events & EPOLLHUP as libc::c_int as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            (*((*eventLoop).fired).offset(j as isize)).fd = (*e).data.fd;
            (*((*eventLoop).fired).offset(j as isize)).mask = mask;
            j += 1;
            j;
        }
    }
    return numevents;
}
unsafe extern "C" fn aeApiFree(mut eventLoop: *mut aeEventLoop) {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    close((*state).epfd);
    zfree((*state).events as *mut libc::c_void);
    zfree(state as *mut libc::c_void);
}
unsafe extern "C" fn aeApiAddEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) -> libc::c_int {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    let mut ee: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    let mut op: libc::c_int = if (*((*eventLoop).events).offset(fd as isize)).mask
        == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        3 as libc::c_int
    };
    ee.events = 0 as libc::c_int as uint32_t;
    mask |= (*((*eventLoop).events).offset(fd as isize)).mask;
    if mask & 1 as libc::c_int != 0 {
        ee.events |= EPOLLIN as libc::c_int as libc::c_uint;
    }
    if mask & 2 as libc::c_int != 0 {
        ee.events |= EPOLLOUT as libc::c_int as libc::c_uint;
    }
    ee.data.fd = fd;
    if epoll_ctl((*state).epfd, op, fd, &mut ee) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiDelEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut delmask: libc::c_int,
) {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    let mut ee: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    let mut mask: libc::c_int = (*((*eventLoop).events).offset(fd as isize)).mask
        & !delmask;
    ee.events = 0 as libc::c_int as uint32_t;
    if mask & 1 as libc::c_int != 0 {
        ee.events |= EPOLLIN as libc::c_int as libc::c_uint;
    }
    if mask & 2 as libc::c_int != 0 {
        ee.events |= EPOLLOUT as libc::c_int as libc::c_uint;
    }
    ee.data.fd = fd;
    if mask != 0 as libc::c_int {
        epoll_ctl((*state).epfd, 3 as libc::c_int, fd, &mut ee);
    } else {
        epoll_ctl((*state).epfd, 2 as libc::c_int, fd, &mut ee);
    };
}
pub unsafe extern "C" fn aeCreateEventLoop(
    mut setsize: libc::c_int,
) -> *mut aeEventLoop {
    let mut eventLoop: *mut aeEventLoop = 0 as *mut aeEventLoop;
    let mut i: libc::c_int = 0;
    eventLoop = zmalloc(::std::mem::size_of::<aeEventLoop>() as libc::c_ulong)
        as *mut aeEventLoop;
    if !eventLoop.is_null() {
        (*eventLoop)
            .events = zmalloc(
            (::std::mem::size_of::<aeFileEvent>() as libc::c_ulong)
                .wrapping_mul(setsize as libc::c_ulong),
        ) as *mut aeFileEvent;
        (*eventLoop)
            .fired = zmalloc(
            (::std::mem::size_of::<aeFiredEvent>() as libc::c_ulong)
                .wrapping_mul(setsize as libc::c_ulong),
        ) as *mut aeFiredEvent;
        if !(((*eventLoop).events).is_null() || ((*eventLoop).fired).is_null()) {
            (*eventLoop).setsize = setsize;
            (*eventLoop).lastTime = time(0 as *mut time_t);
            (*eventLoop).timeEventHead = 0 as *mut aeTimeEvent;
            (*eventLoop).timeEventNextId = 0 as libc::c_int as libc::c_longlong;
            (*eventLoop).stop = 0 as libc::c_int;
            (*eventLoop).maxfd = -(1 as libc::c_int);
            (*eventLoop).beforesleep = None;
            if !(aeApiCreate(eventLoop) == -(1 as libc::c_int)) {
                i = 0 as libc::c_int;
                while i < setsize {
                    (*((*eventLoop).events).offset(i as isize)).mask = 0 as libc::c_int;
                    i += 1;
                    i;
                }
                return eventLoop;
            }
        }
    }
    if !eventLoop.is_null() {
        zfree((*eventLoop).events as *mut libc::c_void);
        zfree((*eventLoop).fired as *mut libc::c_void);
        zfree(eventLoop as *mut libc::c_void);
    }
    return 0 as *mut aeEventLoop;
}
pub unsafe extern "C" fn aeGetSetSize(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    return (*eventLoop).setsize;
}
pub unsafe extern "C" fn aeResizeSetSize(
    mut eventLoop: *mut aeEventLoop,
    mut setsize: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if setsize == (*eventLoop).setsize {
        return 0 as libc::c_int;
    }
    if (*eventLoop).maxfd >= setsize {
        return -(1 as libc::c_int);
    }
    if aeApiResize(eventLoop, setsize) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    (*eventLoop)
        .events = zrealloc(
        (*eventLoop).events as *mut libc::c_void,
        (::std::mem::size_of::<aeFileEvent>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    ) as *mut aeFileEvent;
    (*eventLoop)
        .fired = zrealloc(
        (*eventLoop).fired as *mut libc::c_void,
        (::std::mem::size_of::<aeFiredEvent>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    ) as *mut aeFiredEvent;
    (*eventLoop).setsize = setsize;
    i = (*eventLoop).maxfd + 1 as libc::c_int;
    while i < setsize {
        (*((*eventLoop).events).offset(i as isize)).mask = 0 as libc::c_int;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn aeDeleteEventLoop(mut eventLoop: *mut aeEventLoop) {
    aeApiFree(eventLoop);
    zfree((*eventLoop).events as *mut libc::c_void);
    zfree((*eventLoop).fired as *mut libc::c_void);
    zfree(eventLoop as *mut libc::c_void);
}
pub unsafe extern "C" fn aeStop(mut eventLoop: *mut aeEventLoop) {
    (*eventLoop).stop = 1 as libc::c_int;
}
pub unsafe extern "C" fn aeCreateFileEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut proc_0: Option::<aeFileProc>,
    mut clientData: *mut libc::c_void,
) -> libc::c_int {
    if fd >= (*eventLoop).setsize {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    if aeApiAddEvent(eventLoop, fd, mask) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    (*fe).mask |= mask;
    if mask & 1 as libc::c_int != 0 {
        (*fe).rfileProc = proc_0;
    }
    if mask & 2 as libc::c_int != 0 {
        (*fe).wfileProc = proc_0;
    }
    (*fe).clientData = clientData;
    if fd > (*eventLoop).maxfd {
        (*eventLoop).maxfd = fd;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn aeDeleteFileEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) {
    if fd >= (*eventLoop).setsize {
        return;
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    if (*fe).mask == 0 as libc::c_int {
        return;
    }
    aeApiDelEvent(eventLoop, fd, mask);
    (*fe).mask = (*fe).mask & !mask;
    if fd == (*eventLoop).maxfd && (*fe).mask == 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        j = (*eventLoop).maxfd - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            if (*((*eventLoop).events).offset(j as isize)).mask != 0 as libc::c_int {
                break;
            }
            j -= 1;
            j;
        }
        (*eventLoop).maxfd = j;
    }
}
pub unsafe extern "C" fn aeGetFileEvents(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
) -> libc::c_int {
    if fd >= (*eventLoop).setsize {
        return 0 as libc::c_int;
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    return (*fe).mask;
}
unsafe extern "C" fn aeGetTime(
    mut seconds: *mut libc::c_long,
    mut milliseconds: *mut libc::c_long,
) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    *seconds = tv.tv_sec;
    *milliseconds = tv.tv_usec / 1000 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn aeAddMillisecondsToNow(
    mut milliseconds: libc::c_longlong,
    mut sec: *mut libc::c_long,
    mut ms: *mut libc::c_long,
) {
    let mut cur_sec: libc::c_long = 0;
    let mut cur_ms: libc::c_long = 0;
    let mut when_sec: libc::c_long = 0;
    let mut when_ms: libc::c_long = 0;
    aeGetTime(&mut cur_sec, &mut cur_ms);
    when_sec = (cur_sec as libc::c_longlong
        + milliseconds / 1000 as libc::c_int as libc::c_longlong) as libc::c_long;
    when_ms = (cur_ms as libc::c_longlong
        + milliseconds % 1000 as libc::c_int as libc::c_longlong) as libc::c_long;
    if when_ms >= 1000 as libc::c_int as libc::c_long {
        when_sec += 1;
        when_sec;
        when_ms -= 1000 as libc::c_int as libc::c_long;
    }
    *sec = when_sec;
    *ms = when_ms;
}
pub unsafe extern "C" fn aeCreateTimeEvent(
    mut eventLoop: *mut aeEventLoop,
    mut milliseconds: libc::c_longlong,
    mut proc_0: Option::<aeTimeProc>,
    mut clientData: *mut libc::c_void,
    mut finalizerProc: Option::<aeEventFinalizerProc>,
) -> libc::c_longlong {
    let fresh0 = (*eventLoop).timeEventNextId;
    (*eventLoop).timeEventNextId = (*eventLoop).timeEventNextId + 1;
    let mut id: libc::c_longlong = fresh0;
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    te = zmalloc(::std::mem::size_of::<aeTimeEvent>() as libc::c_ulong)
        as *mut aeTimeEvent;
    if te.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    (*te).id = id;
    aeAddMillisecondsToNow(milliseconds, &mut (*te).when_sec, &mut (*te).when_ms);
    (*te).timeProc = proc_0;
    (*te).finalizerProc = finalizerProc;
    (*te).clientData = clientData;
    (*te).next = (*eventLoop).timeEventHead;
    (*eventLoop).timeEventHead = te;
    return id;
}
pub unsafe extern "C" fn aeDeleteTimeEvent(
    mut eventLoop: *mut aeEventLoop,
    mut id: libc::c_longlong,
) -> libc::c_int {
    let mut te: *mut aeTimeEvent = (*eventLoop).timeEventHead;
    while !te.is_null() {
        if (*te).id == id {
            (*te).id = -(1 as libc::c_int) as libc::c_longlong;
            return 0 as libc::c_int;
        }
        te = (*te).next;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn aeSearchNearestTimer(
    mut eventLoop: *mut aeEventLoop,
) -> *mut aeTimeEvent {
    let mut te: *mut aeTimeEvent = (*eventLoop).timeEventHead;
    let mut nearest: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    while !te.is_null() {
        if nearest.is_null() || (*te).when_sec < (*nearest).when_sec
            || (*te).when_sec == (*nearest).when_sec
                && (*te).when_ms < (*nearest).when_ms
        {
            nearest = te;
        }
        te = (*te).next;
    }
    return nearest;
}
unsafe extern "C" fn processTimeEvents(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    let mut processed: libc::c_int = 0 as libc::c_int;
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut prev: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut maxId: libc::c_longlong = 0;
    let mut now: time_t = time(0 as *mut time_t);
    if now < (*eventLoop).lastTime {
        te = (*eventLoop).timeEventHead;
        while !te.is_null() {
            (*te).when_sec = 0 as libc::c_int as libc::c_long;
            te = (*te).next;
        }
    }
    (*eventLoop).lastTime = now;
    prev = 0 as *mut aeTimeEvent;
    te = (*eventLoop).timeEventHead;
    maxId = (*eventLoop).timeEventNextId - 1 as libc::c_int as libc::c_longlong;
    while !te.is_null() {
        let mut now_sec: libc::c_long = 0;
        let mut now_ms: libc::c_long = 0;
        let mut id: libc::c_longlong = 0;
        if (*te).id == -(1 as libc::c_int) as libc::c_longlong {
            let mut next: *mut aeTimeEvent = (*te).next;
            if prev.is_null() {
                (*eventLoop).timeEventHead = (*te).next;
            } else {
                (*prev).next = (*te).next;
            }
            if ((*te).finalizerProc).is_some() {
                ((*te).finalizerProc).unwrap()(eventLoop, (*te).clientData);
            }
            zfree(te as *mut libc::c_void);
            te = next;
        } else if (*te).id > maxId {
            te = (*te).next;
        } else {
            aeGetTime(&mut now_sec, &mut now_ms);
            if now_sec > (*te).when_sec
                || now_sec == (*te).when_sec && now_ms >= (*te).when_ms
            {
                let mut retval: libc::c_int = 0;
                id = (*te).id;
                retval = ((*te).timeProc).unwrap()(eventLoop, id, (*te).clientData);
                processed += 1;
                processed;
                if retval != -(1 as libc::c_int) {
                    aeAddMillisecondsToNow(
                        retval as libc::c_longlong,
                        &mut (*te).when_sec,
                        &mut (*te).when_ms,
                    );
                } else {
                    (*te).id = -(1 as libc::c_int) as libc::c_longlong;
                }
            }
            prev = te;
            te = (*te).next;
        }
    }
    return processed;
}
pub unsafe extern "C" fn aeProcessEvents(
    mut eventLoop: *mut aeEventLoop,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut processed: libc::c_int = 0 as libc::c_int;
    let mut numevents: libc::c_int = 0;
    if flags & 2 as libc::c_int == 0 && flags & 1 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    if (*eventLoop).maxfd != -(1 as libc::c_int)
        || flags & 2 as libc::c_int != 0 && flags & 4 as libc::c_int == 0
    {
        let mut j: libc::c_int = 0;
        let mut shortest: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut tvp: *mut timeval = 0 as *mut timeval;
        if flags & 2 as libc::c_int != 0 && flags & 4 as libc::c_int == 0 {
            shortest = aeSearchNearestTimer(eventLoop);
        }
        if !shortest.is_null() {
            let mut now_sec: libc::c_long = 0;
            let mut now_ms: libc::c_long = 0;
            aeGetTime(&mut now_sec, &mut now_ms);
            tvp = &mut tv;
            let mut ms: libc::c_longlong = (((*shortest).when_sec - now_sec)
                * 1000 as libc::c_int as libc::c_long + (*shortest).when_ms - now_ms)
                as libc::c_longlong;
            if ms > 0 as libc::c_int as libc::c_longlong {
                (*tvp)
                    .tv_sec = (ms / 1000 as libc::c_int as libc::c_longlong) as __time_t;
                (*tvp)
                    .tv_usec = (ms % 1000 as libc::c_int as libc::c_longlong
                    * 1000 as libc::c_int as libc::c_longlong) as __suseconds_t;
            } else {
                (*tvp).tv_sec = 0 as libc::c_int as __time_t;
                (*tvp).tv_usec = 0 as libc::c_int as __suseconds_t;
            }
        } else if flags & 4 as libc::c_int != 0 {
            tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            tv.tv_sec = tv.tv_usec;
            tvp = &mut tv;
        } else {
            tvp = 0 as *mut timeval;
        }
        numevents = aeApiPoll(eventLoop, tvp);
        j = 0 as libc::c_int;
        while j < numevents {
            let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events)
                .offset((*((*eventLoop).fired).offset(j as isize)).fd as isize)
                as *mut aeFileEvent;
            let mut mask: libc::c_int = (*((*eventLoop).fired).offset(j as isize)).mask;
            let mut fd: libc::c_int = (*((*eventLoop).fired).offset(j as isize)).fd;
            let mut rfired: libc::c_int = 0 as libc::c_int;
            if (*fe).mask & mask & 1 as libc::c_int != 0 {
                rfired = 1 as libc::c_int;
                ((*fe).rfileProc).unwrap()(eventLoop, fd, (*fe).clientData, mask);
            }
            if (*fe).mask & mask & 2 as libc::c_int != 0 {
                if rfired == 0 || (*fe).wfileProc != (*fe).rfileProc {
                    ((*fe).wfileProc).unwrap()(eventLoop, fd, (*fe).clientData, mask);
                }
            }
            processed += 1;
            processed;
            j += 1;
            j;
        }
    }
    if flags & 2 as libc::c_int != 0 {
        processed += processTimeEvents(eventLoop);
    }
    return processed;
}
pub unsafe extern "C" fn aeWait(
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut milliseconds: libc::c_longlong,
) -> libc::c_int {
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut retmask: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int = 0;
    memset(
        &mut pfd as *mut pollfd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pollfd>() as libc::c_ulong,
    );
    pfd.fd = fd;
    if mask & 1 as libc::c_int != 0 {
        pfd.events = (pfd.events as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
    }
    if mask & 2 as libc::c_int != 0 {
        pfd.events = (pfd.events as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    retval = poll(&mut pfd, 1 as libc::c_int as nfds_t, milliseconds as libc::c_int);
    if retval == 1 as libc::c_int {
        if pfd.revents as libc::c_int & 0x1 as libc::c_int != 0 {
            retmask |= 1 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 0x4 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 0x8 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 0x10 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        return retmask;
    } else {
        return retval
    };
}
pub unsafe extern "C" fn aeMain(mut eventLoop: *mut aeEventLoop) {
    (*eventLoop).stop = 0 as libc::c_int;
    while (*eventLoop).stop == 0 {
        if ((*eventLoop).beforesleep).is_some() {
            ((*eventLoop).beforesleep).unwrap()(eventLoop);
        }
        aeProcessEvents(eventLoop, 1 as libc::c_int | 2 as libc::c_int);
    }
}
pub unsafe extern "C" fn aeGetApiName() -> *mut libc::c_char {
    return aeApiName();
}
pub unsafe extern "C" fn aeSetBeforeSleepProc(
    mut eventLoop: *mut aeEventLoop,
    mut beforesleep: Option::<aeBeforeSleepProc>,
) {
    (*eventLoop).beforesleep = beforesleep;
}
