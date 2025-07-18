use ::libc;
extern "C" {
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type strm_sighandler_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> (),
>;
pub type sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sig_list {
    pub sig: libc::c_int,
    pub func: strm_sighandler_t,
    pub arg: *mut libc::c_void,
    pub next: *mut sig_list,
}
pub static mut sig_list: *mut sig_list = 0 as *const sig_list as *mut sig_list;
unsafe extern "C" fn handler(mut sig: libc::c_int) {
    let mut list: *mut sig_list = 0 as *mut sig_list;
    list = sig_list;
    while !list.is_null() {
        if (*list).sig == sig {
            (Some(((*list).func).unwrap())).unwrap()(sig, (*list).arg);
        }
        list = (*list).next;
    }
}
unsafe extern "C" fn sigcall(mut sig: libc::c_int, mut f: *mut libc::c_void) {
    (Some((::std::mem::transmute::<*mut libc::c_void, sighandler_t>(f)).unwrap()))
        .unwrap()(sig);
}
unsafe extern "C" fn add_sig(
    mut sig: libc::c_int,
    mut func: strm_sighandler_t,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut node: *mut sig_list = malloc(
        ::std::mem::size_of::<sig_list>() as libc::c_ulong,
    ) as *mut sig_list;
    if node.is_null() {
        return 1 as libc::c_int;
    }
    (*node).next = sig_list;
    (*node).sig = sig;
    (*node).func = func;
    (*node).arg = arg;
    sig_list = node;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_signal(
    mut sig: libc::c_int,
    mut func: strm_sighandler_t,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut r: sighandler_t = signal(
        sig,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if r
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        return 1 as libc::c_int;
    }
    if r.is_some() && r != Some(handler as unsafe extern "C" fn(libc::c_int) -> ()) {
        if add_sig(
            sig,
            Some(sigcall as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()),
            ::std::mem::transmute::<sighandler_t, *mut libc::c_void>(r),
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    if add_sig(sig, func, arg) == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    r = signal(sig, Some(handler as unsafe extern "C" fn(libc::c_int) -> ()));
    if r
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_unsignal(
    mut sig: libc::c_int,
    mut func: strm_sighandler_t,
) -> libc::c_int {
    let mut r: sighandler_t = signal(
        sig,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    let mut list: *mut sig_list = 0 as *mut sig_list;
    let mut tmp: *mut sig_list = 0 as *mut sig_list;
    if r
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        return 1 as libc::c_int;
    }
    list = sig_list;
    while !list.is_null() {
        if (*list).sig == sig {
            if tmp.is_null() {
                sig_list = (*list).next;
            } else {
                (*tmp).next = (*list).next;
                tmp = list;
            }
        }
        list = (*list).next;
    }
    signal(sig, Some(handler as unsafe extern "C" fn(libc::c_int) -> ()));
    return 0 as libc::c_int;
}
