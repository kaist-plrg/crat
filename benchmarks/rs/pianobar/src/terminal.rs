use ::libc;
extern "C" {
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
static mut restore: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
unsafe extern "C" fn BarTermHandleCont(mut sig: libc::c_int) {
    BarTermInit();
}
pub unsafe extern "C" fn BarTermInit() {
    let mut newopt: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    tcgetattr(0 as libc::c_int, &mut restore);
    memcpy(
        &mut newopt as *mut termios as *mut libc::c_void,
        &mut restore as *mut termios as *const libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    newopt.c_lflag &= !(0o10 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut newopt);
    signal(
        18 as libc::c_int,
        Some(BarTermHandleCont as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
pub unsafe extern "C" fn BarTermRestore() {
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut restore);
}
