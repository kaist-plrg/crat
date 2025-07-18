use ::libc;
extern "C" {
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_t {
    pub sock: libc::c_int,
}
pub unsafe extern "C" fn get_dryrun_socket() -> sock_t {
    let mut sock: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
    );
    if sock <= 0 as libc::c_int {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"couldn't create socket. Error: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    let mut s: sock_t = sock_t { sock: 0 };
    s.sock = sock;
    return s;
}
