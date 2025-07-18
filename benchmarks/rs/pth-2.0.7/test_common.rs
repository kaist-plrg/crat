use ::libc;
extern "C" {
    pub type pth_event_st;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pth_once(
        _: *mut pth_once_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pth_key_create(
        _: *mut pth_key_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn pth_key_setdata(_: pth_key_t, _: *const libc::c_void) -> libc::c_int;
    fn pth_key_getdata(_: pth_key_t) -> *mut libc::c_void;
    fn pth_read_ev(
        _: libc::c_int,
        _: *mut libc::c_void,
        _: size_t,
        _: pth_event_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type pth_event_t = *mut pth_event_st;
pub type pth_key_t = libc::c_int;
pub type pth_once_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readline_buf {
    pub rl_cnt: libc::c_int,
    pub rl_bufptr: *mut libc::c_char,
    pub rl_buf: [libc::c_char; 1024],
}
static mut readline_key: pth_key_t = 0;
static mut readline_once_ctrl: pth_once_t = 0 as libc::c_int;
unsafe extern "C" fn readline_buf_destroy(mut vp: *mut libc::c_void) {
    free(vp);
}
unsafe extern "C" fn readline_init(mut vp: *mut libc::c_void) {
    pth_key_create(
        &mut readline_key,
        Some(readline_buf_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
pub unsafe extern "C" fn pth_readline(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut buflen: size_t,
) -> ssize_t {
    return pth_readline_ev(fd, buf, buflen, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_readline_ev(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut buflen: size_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut n: size_t = 0;
    let mut rc: ssize_t = 0;
    let mut c: libc::c_char = '\0' as i32 as libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rl: *mut readline_buf = 0 as *mut readline_buf;
    pth_once(
        &mut readline_once_ctrl,
        Some(readline_init as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    rl = pth_key_getdata(readline_key) as *mut readline_buf;
    if rl.is_null() {
        rl = malloc(::std::mem::size_of::<readline_buf>() as libc::c_ulong)
            as *mut readline_buf;
        (*rl).rl_cnt = 0 as libc::c_int;
        (*rl).rl_bufptr = 0 as *mut libc::c_char;
        pth_key_setdata(readline_key, rl as *const libc::c_void);
    }
    cp = buf as *mut libc::c_char;
    n = 1 as libc::c_int as size_t;
    while n < buflen {
        rc = 1 as libc::c_int as ssize_t;
        if (*rl).rl_cnt <= 0 as libc::c_int {
            (*rl)
                .rl_cnt = pth_read_ev(
                fd,
                ((*rl).rl_buf).as_mut_ptr() as *mut libc::c_void,
                1024 as libc::c_int as size_t,
                ev_extra,
            ) as libc::c_int;
            if (*rl).rl_cnt < 0 as libc::c_int {
                rc = -(1 as libc::c_int) as ssize_t;
            } else if (*rl).rl_cnt == 0 as libc::c_int {
                rc = 0 as libc::c_int as ssize_t;
            } else {
                (*rl).rl_bufptr = ((*rl).rl_buf).as_mut_ptr();
            }
        }
        if rc == 1 as libc::c_int as libc::c_long {
            (*rl).rl_cnt -= 1;
            (*rl).rl_cnt;
            let fresh0 = (*rl).rl_bufptr;
            (*rl).rl_bufptr = ((*rl).rl_bufptr).offset(1);
            c = *fresh0;
        }
        if rc == 1 as libc::c_int as libc::c_long {
            if c as libc::c_int == '\r' as i32 {
                n = n.wrapping_sub(1);
                n;
            } else {
                let fresh1 = cp;
                cp = cp.offset(1);
                *fresh1 = c;
                if c as libc::c_int == '\n' as i32 {
                    break;
                }
            }
            n = n.wrapping_add(1);
            n;
        } else if rc == 0 as libc::c_int as libc::c_long {
            if !(n == 1 as libc::c_int as libc::c_ulong) {
                break;
            }
            return 0 as libc::c_int as ssize_t;
        } else {
            return -(1 as libc::c_int) as ssize_t
        }
    }
    *cp = '\0' as i32 as libc::c_char;
    return n as ssize_t;
}
