use ::libc;
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn pth_init() -> libc::c_int;
    fn pth_kill() -> libc::c_int;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_cancel(_: pth_t) -> libc::c_int;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> libc::c_int;
    fn pth_timeout(_: libc::c_long, _: libc::c_long) -> pth_time_t;
    fn pth_event(_: libc::c_ulong, _: ...) -> pth_event_t;
    fn pth_event_free(_: pth_event_t, _: libc::c_int) -> libc::c_int;
    fn pth_mutex_init(_: *mut pth_mutex_t) -> libc::c_int;
    fn pth_mutex_acquire(
        _: *mut pth_mutex_t,
        _: libc::c_int,
        _: pth_event_t,
    ) -> libc::c_int;
    fn pth_mutex_release(_: *mut pth_mutex_t) -> libc::c_int;
    fn pth_sigwait_ev(
        _: *const sigset_t,
        _: *mut libc::c_int,
        _: pth_event_t,
    ) -> libc::c_int;
    fn pth_sleep(_: libc::c_uint) -> libc::c_uint;
    fn pth_cond_init(_: *mut pth_cond_t) -> libc::c_int;
    fn pth_cond_await(
        _: *mut pth_cond_t,
        _: *mut pth_mutex_t,
        _: pth_event_t,
    ) -> libc::c_int;
    fn pth_cond_notify(_: *mut pth_cond_t, _: libc::c_int) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pth_time_t = timeval;
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
pub type pth_event_t = *mut pth_event_st;
pub type C2RustUnnamed = libc::c_uint;
pub const PTH_FREE_ALL: C2RustUnnamed = 1;
pub const PTH_FREE_THIS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mutex_st {
    pub mx_node: pth_ringnode_t,
    pub mx_state: libc::c_int,
    pub mx_owner: pth_t,
    pub mx_count: libc::c_ulong,
}
pub type pth_mutex_t = pth_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_cond_st {
    pub cn_state: libc::c_ulong,
    pub cn_waiters: libc::c_uint,
}
pub type pth_cond_t = pth_cond_st;
pub type philstat = libc::c_uint;
pub const eating: philstat = 2;
pub const hungry: philstat = 1;
pub const thinking: philstat = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tablestruct {
    pub tid: [pth_t; 5],
    pub self_0: [libc::c_int; 5],
    pub mutex: pth_mutex_t,
    pub condition: [pth_cond_t; 5],
    pub status: [philstat; 5],
}
pub type table = tablestruct;
static mut philstatstr: [*const libc::c_char; 3] = [
    b"thinking\0" as *const u8 as *const libc::c_char,
    b"hungry  \0" as *const u8 as *const libc::c_char,
    b"EATING  \0" as *const u8 as *const libc::c_char,
];
static mut tab: *mut table = 0 as *const table as *mut table;
unsafe extern "C" fn printstate() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        printf(
            b"| %s \0" as *const u8 as *const libc::c_char,
            philstatstr[(*tab).status[i as usize] as libc::c_int as usize],
        );
        i += 1;
        i;
    }
    printf(b"|\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn test(mut i: libc::c_uint) -> libc::c_int {
    if (*tab).status[i as usize] as libc::c_uint == hungry as libc::c_int as libc::c_uint
        && (*tab)
            .status[i
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(5 as libc::c_int as libc::c_uint) as usize] as libc::c_uint
            != eating as libc::c_int as libc::c_uint
        && (*tab)
            .status[i
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_add(5 as libc::c_int as libc::c_uint)
            .wrapping_rem(5 as libc::c_int as libc::c_uint) as usize] as libc::c_uint
            != eating as libc::c_int as libc::c_uint
    {
        (*tab).status[i as usize] = eating;
        pth_cond_notify(
            &mut *((*tab).condition).as_mut_ptr().offset(i as isize),
            0 as libc::c_int,
        );
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pickup(mut k: libc::c_uint) {
    pth_mutex_acquire(&mut (*tab).mutex, 0 as libc::c_int, 0 as pth_event_t);
    (*tab).status[k as usize] = hungry;
    printstate();
    if test(k) == 0 {
        pth_cond_await(
            &mut *((*tab).condition).as_mut_ptr().offset(k as isize),
            &mut (*tab).mutex,
            0 as pth_event_t,
        );
    }
    printstate();
    pth_mutex_release(&mut (*tab).mutex);
}
unsafe extern "C" fn putdown(mut k: libc::c_uint) {
    pth_mutex_acquire(&mut (*tab).mutex, 0 as libc::c_int, 0 as pth_event_t);
    (*tab).status[k as usize] = thinking;
    printstate();
    test(
        k
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(5 as libc::c_int as libc::c_uint),
    );
    test(
        k
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_add(5 as libc::c_int as libc::c_uint)
            .wrapping_rem(5 as libc::c_int as libc::c_uint),
    );
    pth_mutex_release(&mut (*tab).mutex);
}
unsafe extern "C" fn philosopher(mut _who: *mut libc::c_void) -> *mut libc::c_void {
    let mut who: *mut libc::c_uint = _who as *mut libc::c_uint;
    loop {
        pth_sleep((*who).wrapping_add(1 as libc::c_int as libc::c_uint));
        pickup(*who);
        pth_sleep(1 as libc::c_int as libc::c_uint);
        putdown(*who);
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: libc::c_int = 0;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    pth_init();
    printf(
        b"This is TEST_PHILO, a Pth test showing the Five Dining Philosophers\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"This is a demonstration showing the famous concurrency problem of the\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Five Dining Philosophers as analysed 1965 by E.W.Dijkstra:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Five philosophers are sitting around a round table, each with a bowl of\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Chinese food in front of him. Between periods of talking they may start\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"eating whenever they want to, with their bowls being filled frequently.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"But there are only five chopsticks available, one each to the left of\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"each bowl - and for eating Chinese food one needs two chopsticks. When\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"a philosopher wants to start eating, he must pick up the chopstick to\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"the left of his bowl and the chopstick to the right of his bowl. He\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"may find, however, that either one (or even both) of the chopsticks is\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"unavailable as it is being used by another philosopher sitting on his\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"right or left, so he has to wait.\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"This situation shows classical contention under concurrency (the\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"philosophers want to grab the chopsticks) and the possibility of a\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"deadlock (all philosophers wait that the chopstick to their left becomes\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"available).\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"The demonstration runs max. 60 seconds. To stop before, press CTRL-C.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"+----P1----+----P2----+----P3----+----P4----+----P5----+\n\0" as *const u8
            as *const libc::c_char,
    );
    tab = malloc(::std::mem::size_of::<table>() as libc::c_ulong) as *mut table;
    if pth_mutex_init(&mut (*tab).mutex) == 0 {
        perror(b"pth_mutex_init\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        (*tab).self_0[i as usize] = i;
        (*tab).status[i as usize] = thinking;
        if pth_cond_init(&mut *((*tab).condition).as_mut_ptr().offset(i as isize)) == 0 {
            perror(b"pth_cond_init\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        (*tab)
            .tid[i
            as usize] = pth_spawn(
            0 as pth_attr_t,
            Some(
                philosopher
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut *((*tab).self_0).as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as *mut libc::c_void,
        );
        if ((*tab).tid[i as usize]).is_null() {
            perror(b"pth_spawn\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    sigemptyset(&mut ss);
    sigaddset(&mut ss, 2 as libc::c_int);
    ev = pth_event(
        ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong,
        pth_timeout(60 as libc::c_int as libc::c_long, 0 as libc::c_int as libc::c_long),
    );
    pth_sigwait_ev(&mut ss, &mut sig, ev);
    pth_event_free(ev, PTH_FREE_ALL as libc::c_int);
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        pth_cancel((*tab).tid[i as usize]);
        i += 1;
        i;
    }
    while pth_join(0 as pth_t, 0 as *mut *mut libc::c_void) != 0 {}
    printf(
        b"+----------+----------+----------+----------+----------+\n\0" as *const u8
            as *const libc::c_char,
    );
    free(tab as *mut libc::c_void);
    pth_kill();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
