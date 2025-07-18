use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub next: *mut event,
    pub handler: Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
    pub data: *mut libc::c_char,
    pub fd: libc::c_int,
    pub type_0: libc::c_int,
    pub pri: libc::c_int,
    pub timeout: timeval,
    pub queued: libc::c_int,
    pub active: libc::c_int,
    pub condpos: *mut libc::c_int,
    pub condneg: *mut libc::c_int,
}
static mut evs: *mut event = 0 as *const event as *mut event;
static mut tevs: *mut event = 0 as *const event as *mut event;
static mut nextev: *mut event = 0 as *const event as *mut event;
static mut calctimeout: libc::c_int = 0;
pub unsafe extern "C" fn evenq(mut ev: *mut event) {
    let mut evp: *mut event = 0 as *mut event;
    let mut evpp: *mut *mut event = 0 as *mut *mut event;
    if (*ev).queued != 0 {
        return;
    }
    evpp = &mut evs;
    if (*ev).type_0 == 0 as libc::c_int {
        calctimeout = 1 as libc::c_int;
        evpp = &mut tevs;
    }
    loop {
        evp = *evpp;
        if evp.is_null() {
            break;
        }
        if (*ev).pri > (*evp).pri {
            break;
        }
        evpp = &mut (*evp).next;
    }
    (*ev).next = evp;
    *evpp = ev;
    (*ev).queued = 1 as libc::c_int;
}
pub unsafe extern "C" fn evdeq(mut ev: *mut event) {
    let mut evp: *mut event = 0 as *mut event;
    let mut evpp: *mut *mut event = 0 as *mut *mut event;
    if (*ev).queued == 0 {
        return;
    }
    evpp = &mut evs;
    if (*ev).type_0 == 0 as libc::c_int {
        calctimeout = 1 as libc::c_int;
        evpp = &mut tevs;
    }
    loop {
        evp = *evpp;
        if evp.is_null() {
            break;
        }
        if evp == ev {
            break;
        }
        evpp = &mut (*evp).next;
    }
    *evpp = (*ev).next;
    (*ev).queued = 0 as libc::c_int;
    if ev == nextev {
        nextev = (*nextev).next;
    }
}
unsafe extern "C" fn calctimo() -> *mut event {
    let mut ev: *mut event = 0 as *mut event;
    let mut min: *mut event = 0 as *mut event;
    let mut mins: libc::c_long = 0;
    min = tevs;
    if min.is_null() {
        return 0 as *mut event;
    }
    mins = (*min).timeout.tv_sec;
    ev = (*tevs).next;
    while !ev.is_null() {
        if !(mins < (*ev).timeout.tv_sec) {
            if mins > (*ev).timeout.tv_sec
                || (*min).timeout.tv_usec > (*ev).timeout.tv_usec
            {
                min = ev;
                mins = (*ev).timeout.tv_sec;
            }
        }
        ev = (*ev).next;
    }
    return min;
}
pub unsafe extern "C" fn sched() {
    let mut ev: *mut event = 0 as *mut event;
    let mut r: fd_set = fd_set { fds_bits: [0; 16] };
    let mut w: fd_set = fd_set { fds_bits: [0; 16] };
    let mut set: *mut fd_set = 0 as *mut fd_set;
    let mut timeoutev: *mut event = 0 as *mut event;
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut nsel: libc::c_int = 0;
    loop {
        if calctimeout != 0 {
            timeoutev = calctimo();
        }
        if !timeoutev.is_null() {
            gettimeofday(&mut timeout, 0 as *mut libc::c_void);
            timeout.tv_sec = (*timeoutev).timeout.tv_sec - timeout.tv_sec;
            timeout.tv_usec = (*timeoutev).timeout.tv_usec - timeout.tv_usec;
            if timeout.tv_usec < 0 as libc::c_int as libc::c_long {
                timeout.tv_usec += 1000000 as libc::c_int as libc::c_long;
                timeout.tv_sec -= 1;
                timeout.tv_sec;
            }
            if timeout.tv_sec < 0 as libc::c_int as libc::c_long {
                timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
                timeout.tv_sec = 0 as libc::c_int as __time_t;
            }
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(r.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
            fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
            fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        let mut __d0_0: libc::c_int = 0;
        let mut __d1_0: libc::c_int = 0;
        let fresh6 = &mut __d0_0;
        let fresh7;
        let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh9 = &mut __d1_0;
        let fresh10;
        let fresh11 = &mut *(w.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
            fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
            fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
        ev = evs;
        while !ev.is_null() {
            if !(!((*ev).condpos).is_null()
                && *(*ev).condpos
                    <= (if !((*ev).condneg).is_null() {
                        *(*ev).condneg
                    } else {
                        0 as libc::c_int
                    }))
            {
                if (*ev).type_0 == 1 as libc::c_int {
                    r
                        .fds_bits[((*ev).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*ev).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                } else if (*ev).type_0 == 2 as libc::c_int {
                    w
                        .fds_bits[((*ev).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*ev).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                }
            }
            ev = (*ev).next;
        }
        nsel = select(
            1024 as libc::c_int,
            &mut r,
            &mut w,
            0 as *mut fd_set,
            if !timeoutev.is_null() { &mut timeout } else { 0 as *mut timeval },
        );
        if nsel < 0 as libc::c_int {
            if *__errno_location() != 4 as libc::c_int {
                Panic(
                    *__errno_location(),
                    b"select\0" as *const u8 as *const libc::c_char,
                );
            }
            nsel = 0 as libc::c_int;
        } else if nsel == 0 as libc::c_int {
            evdeq(timeoutev);
            ((*timeoutev).handler).unwrap()(timeoutev, (*timeoutev).data);
        }
        let mut current_block_50: u64;
        ev = evs;
        while !ev.is_null() {
            nextev = (*ev).next;
            if (*ev).type_0 != 3 as libc::c_int {
                set = if (*ev).type_0 == 1 as libc::c_int { &mut r } else { &mut w };
                if nsel == 0 as libc::c_int
                    || !((*set)
                        .fds_bits[((*ev).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        & ((1 as libc::c_ulong)
                            << (*ev).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask
                        != 0 as libc::c_int as libc::c_long)
                {
                    current_block_50 = 10758786907990354186;
                } else {
                    nsel -= 1;
                    nsel;
                    current_block_50 = 12497913735442871383;
                }
            } else {
                current_block_50 = 12497913735442871383;
            }
            match current_block_50 {
                12497913735442871383 => {
                    if !(!((*ev).condpos).is_null()
                        && *(*ev).condpos
                            <= (if !((*ev).condneg).is_null() {
                                *(*ev).condneg
                            } else {
                                0 as libc::c_int
                            }))
                    {
                        ((*ev).handler).unwrap()(ev, (*ev).data);
                    }
                }
                _ => {}
            }
            ev = nextev;
        }
    };
}
pub unsafe extern "C" fn SetTimeout(mut ev: *mut event, mut timo: libc::c_int) {
    gettimeofday(&mut (*ev).timeout, 0 as *mut libc::c_void);
    (*ev).timeout.tv_sec += (timo / 1000 as libc::c_int) as libc::c_long;
    (*ev).timeout.tv_usec
        += (timo % 1000 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
    if (*ev).timeout.tv_usec > 1000000 as libc::c_int as libc::c_long {
        (*ev).timeout.tv_usec -= 1000000 as libc::c_int as libc::c_long;
        (*ev).timeout.tv_sec += 1;
        (*ev).timeout.tv_sec;
    }
    if (*ev).queued != 0 {
        calctimeout = 1 as libc::c_int;
    }
}
