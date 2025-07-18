use ::libc;
extern "C" {
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn quit(status: libc::c_int);
    fn iread(fd: libc::c_int, buf: *mut libc::c_uchar, len: libc::c_uint) -> libc::c_int;
    fn flush();
}
pub static mut tty: libc::c_int = 0;
unsafe extern "C" fn open_tty_device(mut dev: *const libc::c_char) -> libc::c_int {
    return open(dev, 0 as libc::c_int);
}
pub unsafe extern "C" fn open_tty() -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    if fd < 0 as libc::c_int {
        let mut dev: *const libc::c_char = ttyname(2 as libc::c_int);
        if !dev.is_null() {
            fd = open_tty_device(dev);
        }
    }
    if fd < 0 as libc::c_int {
        fd = open_tty_device(b"/dev/tty\0" as *const u8 as *const libc::c_char);
    }
    if fd < 0 as libc::c_int {
        fd = 2 as libc::c_int;
    }
    return fd;
}
pub unsafe extern "C" fn open_getchr() {
    tty = open_tty();
}
pub unsafe extern "C" fn close_getchr() {}
pub unsafe extern "C" fn default_wheel_lines() -> libc::c_int {
    let mut lines: libc::c_int = 1 as libc::c_int;
    return lines;
}
pub unsafe extern "C" fn getchr() -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut result: libc::c_int = 0;
    loop {
        flush();
        let mut uc: libc::c_uchar = 0;
        result = iread(
            tty,
            &mut uc,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
        );
        c = uc as libc::c_char;
        if result == -(2 as libc::c_int) {
            return -(2 as libc::c_int);
        }
        if result < 0 as libc::c_int {
            quit(1 as libc::c_int);
        }
        if c as libc::c_int == '\0' as i32 {
            c = -32i32 as libc::c_char;
        }
        if !(result != 1 as libc::c_int) {
            break;
        }
    }
    return c as libc::c_int & 0xff as libc::c_int;
}
