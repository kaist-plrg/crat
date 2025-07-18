use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn at_fatal_signal(
        function: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> libc::c_int;
    fn xalloc_die();
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __sig_atomic_t = libc::c_int;
pub type pid_t = __pid_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slaves_entry_t {
    pub used: sig_atomic_t,
    pub child: pid_t,
}
static mut static_slaves: [slaves_entry_t; 32] = [slaves_entry_t {
    used: 0,
    child: 0,
}; 32];
static mut slaves: *mut slaves_entry_t = unsafe { static_slaves.as_ptr() as *mut _ };
static mut slaves_count: sig_atomic_t = 0 as libc::c_int;
static mut slaves_allocated: size_t = 0;
unsafe extern "C" fn cleanup_slaves() {
    loop {
        let mut n: size_t = slaves_count as size_t;
        if n == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        n = n.wrapping_sub(1);
        n;
        ::std::ptr::write_volatile(
            &mut slaves_count as *mut sig_atomic_t,
            n as sig_atomic_t,
        );
        if (*slaves.offset(n as isize)).used != 0 {
            let mut slave: pid_t = (*slaves.offset(n as isize)).child;
            kill(slave, 1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn cleanup_slaves_action(mut sig: libc::c_int) {
    cleanup_slaves();
}
pub unsafe extern "C" fn register_slave_subprocess(mut child: pid_t) {
    static mut cleanup_slaves_registered: bool = 0 as libc::c_int != 0;
    if !cleanup_slaves_registered {
        atexit(Some(cleanup_slaves as unsafe extern "C" fn() -> ()));
        if at_fatal_signal(
            Some(cleanup_slaves_action as unsafe extern "C" fn(libc::c_int) -> ()),
        ) < 0 as libc::c_int
        {
            xalloc_die();
        }
        cleanup_slaves_registered = 1 as libc::c_int != 0;
    }
    let mut s: *mut slaves_entry_t = slaves;
    let mut s_end: *mut slaves_entry_t = s.offset(slaves_count as isize);
    while s < s_end {
        if (*s).used == 0 {
            ::std::ptr::write_volatile(&mut (*s).child as *mut pid_t, child);
            ::std::ptr::write_volatile(
                &mut (*s).used as *mut sig_atomic_t,
                1 as libc::c_int,
            );
            return;
        }
        s = s.offset(1);
        s;
    }
    if slaves_count as libc::c_ulong == slaves_allocated {
        let mut old_slaves: *mut slaves_entry_t = slaves;
        let mut new_slaves_allocated: size_t = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(slaves_allocated);
        let mut new_slaves: *mut slaves_entry_t = malloc(
            new_slaves_allocated
                .wrapping_mul(::std::mem::size_of::<slaves_entry_t>() as libc::c_ulong),
        ) as *mut slaves_entry_t;
        if new_slaves.is_null() {
            kill(child, 1 as libc::c_int);
            xalloc_die();
        }
        memcpy(
            new_slaves as *mut libc::c_void,
            old_slaves as *const libc::c_void,
            slaves_allocated
                .wrapping_mul(::std::mem::size_of::<slaves_entry_t>() as libc::c_ulong),
        );
        ::std::ptr::write_volatile(&mut slaves as *mut *mut slaves_entry_t, new_slaves);
        slaves_allocated = new_slaves_allocated;
        if old_slaves != static_slaves.as_mut_ptr() {
            rpl_free(old_slaves as *mut libc::c_void);
        }
    }
    ::std::ptr::write_volatile(
        &mut (*slaves.offset(slaves_count as isize)).child as *mut pid_t,
        child,
    );
    ::std::ptr::write_volatile(
        &mut (*slaves.offset(slaves_count as isize)).used as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    ::std::ptr::write_volatile(
        &mut slaves_count as *mut sig_atomic_t,
        ::std::ptr::read_volatile::<sig_atomic_t>(&slaves_count as *const sig_atomic_t)
            + 1,
    );
    ::std::ptr::read_volatile::<sig_atomic_t>(&slaves_count as *const sig_atomic_t);
}
unsafe extern "C" fn unregister_slave_subprocess(mut child: pid_t) {
    let mut s: *mut slaves_entry_t = slaves;
    let mut s_end: *mut slaves_entry_t = s.offset(slaves_count as isize);
    while s < s_end {
        if (*s).used != 0 && (*s).child == child {
            ::std::ptr::write_volatile(
                &mut (*s).used as *mut sig_atomic_t,
                0 as libc::c_int,
            );
        }
        s = s.offset(1);
        s;
    }
}
pub unsafe extern "C" fn wait_subprocess(
    mut child: pid_t,
    mut progname: *const libc::c_char,
    mut ignore_sigpipe: bool,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut termsigp: *mut libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if !termsigp.is_null() {
        *termsigp = 0 as libc::c_int;
    }
    status = 0 as libc::c_int;
    loop {
        let mut result: libc::c_int = waitpid(child, &mut status, 0 as libc::c_int);
        if result != child {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if exit_on_error as libc::c_int != 0 || !null_stderr {
                error(
                    if exit_on_error as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                    *__errno_location(),
                    dcgettext(
                        b"wget-gnulib\0" as *const u8 as *const libc::c_char,
                        b"%s subprocess\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progname,
                );
            }
            return 127 as libc::c_int;
        } else if !(status & 0xff as libc::c_int == 0x7f as libc::c_int) {
            break;
        }
    }
    if slave_process {
        unregister_slave_subprocess(child);
    }
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        if !termsigp.is_null() {
            *termsigp = status & 0x7f as libc::c_int;
        }
        if status & 0x7f as libc::c_int == 13 as libc::c_int
            && ignore_sigpipe as libc::c_int != 0
        {
            return 0 as libc::c_int;
        }
        if exit_on_error as libc::c_int != 0 || !null_stderr && termsigp.is_null() {
            error(
                if exit_on_error as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                0 as libc::c_int,
                dcgettext(
                    b"wget-gnulib\0" as *const u8 as *const libc::c_char,
                    b"%s subprocess got fatal signal %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progname,
                status & 0x7f as libc::c_int,
            );
        }
        return 127 as libc::c_int;
    }
    if !(status & 0x7f as libc::c_int == 0 as libc::c_int) {
        abort();
    }
    if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int == 127 as libc::c_int {
        if exit_on_error as libc::c_int != 0 || !null_stderr {
            error(
                if exit_on_error as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                0 as libc::c_int,
                dcgettext(
                    b"wget-gnulib\0" as *const u8 as *const libc::c_char,
                    b"%s subprocess failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progname,
            );
        }
        return 127 as libc::c_int;
    }
    return (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    slaves_allocated = (::std::mem::size_of::<[slaves_entry_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<slaves_entry_t>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
