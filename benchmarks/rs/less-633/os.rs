use ::libc;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn quit(status: libc::c_int);
    fn ungetcc_back(c: LWCHAR);
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn getchr() -> libc::c_int;
    fn flush();
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn _longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    static mut sigs: libc::c_int;
    static mut ignore_eoi: libc::c_int;
    static mut exit_F_on_close: libc::c_int;
    static mut follow_mode: libc::c_int;
    static mut scanning_eof: libc::c_int;
    static mut intr_char: libc::c_char;
    static mut tty: libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type nfds_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut use_poll: libc::c_int = 1 as libc::c_int;
pub static mut reading: libc::c_int = 0;
pub static mut waiting_for_data: libc::c_int = 0;
pub static mut consecutive_nulls: libc::c_int = 0 as libc::c_int;
static mut waiting_for_data_delay: libc::c_int = 4000 as libc::c_int;
static mut read_label: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
pub unsafe extern "C" fn init_poll() {
    let mut delay: *mut libc::c_char = lgetenv(
        b"LESS_DATA_DELAY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut idelay: libc::c_int = if delay.is_null() {
        0 as libc::c_int
    } else {
        atoi(delay)
    };
    if idelay > 0 as libc::c_int {
        waiting_for_data_delay = idelay;
    }
}
unsafe extern "C" fn check_poll(
    mut fd: libc::c_int,
    mut tty_0: libc::c_int,
) -> libc::c_int {
    let mut poller: [pollfd; 2] = [
        {
            let mut init = pollfd {
                fd: fd,
                events: 0x1 as libc::c_int as libc::c_short,
                revents: 0 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = pollfd {
                fd: tty_0,
                events: 0x1 as libc::c_int as libc::c_short,
                revents: 0 as libc::c_int as libc::c_short,
            };
            init
        },
    ];
    let mut timeout: libc::c_int = if waiting_for_data != 0
        && !(scanning_eof != 0 && follow_mode == 1 as libc::c_int)
    {
        -(1 as libc::c_int)
    } else {
        waiting_for_data_delay
    };
    poll(poller.as_mut_ptr(), 2 as libc::c_int as nfds_t, timeout);
    if poller[1 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int != 0
    {
        let mut ch: LWCHAR = getchr() as LWCHAR;
        if ch == intr_char as libc::c_ulong {
            return -(2 as libc::c_int);
        }
        ungetcc_back(ch);
    }
    if ignore_eoi != 0 && exit_F_on_close != 0
        && poller[0 as libc::c_int as usize].revents as libc::c_int
            & (0x10 as libc::c_int | 0x1 as libc::c_int) == 0x10 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    if poller[0 as libc::c_int as usize].revents as libc::c_int
        & (0x1 as libc::c_int | 0x10 as libc::c_int | 0x8 as libc::c_int)
        == 0 as libc::c_int
    {
        return -(3 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn supports_ctrl_x() -> libc::c_int {
    return use_poll;
}
pub unsafe extern "C" fn iread(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    loop {
        if reading == 0 && _setjmp(read_label.as_mut_ptr()) != 0 {
            reading = 0 as libc::c_int;
            let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
            sigemptyset(&mut mask);
            sigprocmask(2 as libc::c_int, &mut mask, 0 as *mut sigset_t);
            return -(2 as libc::c_int);
        }
        flush();
        reading = 1 as libc::c_int;
        if fd != tty && use_poll != 0 {
            let mut ret: libc::c_int = check_poll(fd, tty);
            if ret != 0 as libc::c_int {
                if ret == -(2 as libc::c_int) {
                    sigs |= 0o1 as libc::c_int;
                }
                reading = 0 as libc::c_int;
                return ret;
            }
        }
        n = read(fd, buf as *mut libc::c_void, len as size_t) as libc::c_int;
        reading = 0 as libc::c_int;
        if ignore_eoi == 0 {
            if n == 0 as libc::c_int {
                consecutive_nulls += 1;
                consecutive_nulls;
            } else {
                consecutive_nulls = 0 as libc::c_int;
            }
            if consecutive_nulls > 20 as libc::c_int {
                quit(1 as libc::c_int);
            }
        }
        if n < 0 as libc::c_int {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if *__errno_location() == 11 as libc::c_int {
                continue;
            }
            return -(1 as libc::c_int);
        } else {
            return n
        }
    };
}
pub unsafe extern "C" fn intread() {
    _longjmp(read_label.as_mut_ptr(), 1 as libc::c_int);
}
pub unsafe extern "C" fn get_time() -> time_t {
    let mut t: time_t = 0;
    time(&mut t);
    return t;
}
pub unsafe extern "C" fn errno_message(
    mut filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    p = strerror(*__errno_location());
    len = (strlen(filename))
        .wrapping_add(strlen(p))
        .wrapping_add(3 as libc::c_int as libc::c_ulong) as libc::c_int;
    m = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    snprintf(
        m,
        len as libc::c_ulong,
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        filename,
        p,
    );
    return m;
}
pub unsafe extern "C" fn signal_message(mut sig: libc::c_int) -> *mut libc::c_char {
    static mut sigbuf: [libc::c_char; 20] = [0; 20];
    let mut description: *mut libc::c_char = strsignal(sig);
    if !description.is_null() {
        return description;
    }
    sprintf(
        sigbuf.as_mut_ptr(),
        b"Signal %d\0" as *const u8 as *const libc::c_char,
        sig,
    );
    return sigbuf.as_mut_ptr();
}
pub unsafe extern "C" fn muldiv(
    mut val: uintmax,
    mut num: uintmax,
    mut den: uintmax,
) -> uintmax {
    let mut q: uintmax = val.wrapping_div(den);
    let mut r: uintmax = val.wrapping_rem(den);
    let mut qnum: uintmax = q.wrapping_mul(num);
    let mut rnum: uintmax = r.wrapping_mul(num);
    let mut quot: uintmax = qnum.wrapping_add(rnum.wrapping_div(den));
    let mut rem: uintmax = rnum.wrapping_rem(den);
    return quot
        .wrapping_add(
            (den.wrapping_div(2 as libc::c_int as libc::c_ulong)
                < rem.wrapping_add(quot & !den & 1 as libc::c_int as libc::c_ulong))
                as libc::c_int as libc::c_ulong,
        );
}
pub unsafe extern "C" fn percentage(
    mut num: POSITION,
    mut den: POSITION,
) -> libc::c_int {
    return muldiv(
        num as uintmax,
        100 as libc::c_int as POSITION as uintmax,
        den as uintmax,
    ) as libc::c_int;
}
pub unsafe extern "C" fn percent_pos(
    mut pos: POSITION,
    mut percent: libc::c_int,
    mut fraction: libc::c_long,
) -> POSITION {
    let mut pctden: POSITION = (percent * 1000000 as libc::c_int) as libc::c_long
        + fraction;
    return muldiv(
        pos as uintmax,
        pctden as uintmax,
        (100 as libc::c_int as libc::c_long * 1000000 as libc::c_int as POSITION)
            as uintmax,
    ) as POSITION;
}
pub unsafe extern "C" fn sleep_ms(mut ms: libc::c_int) {
    let mut sec: libc::c_int = ms / 1000 as libc::c_int;
    let mut t: timespec = {
        let mut init = timespec {
            tv_sec: sec as __time_t,
            tv_nsec: ((ms - sec * 1000 as libc::c_int) * 1000000 as libc::c_int)
                as __syscall_slong_t,
        };
        init
    };
    nanosleep(&mut t, 0 as *mut timespec);
}
