use ::libc;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
}
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_DGRAM: __socket_type = 2;
pub type __socket_type = libc::c_uint;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_STREAM: __socket_type = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
pub unsafe extern "C" fn snoopy_output_socketoutput(
    logMessage: *const libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut remote: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut remoteLength: libc::c_int = 0;
    if 0 as libc::c_int as libc::c_ulong == strlen(logMessage) {
        return 0 as libc::c_int;
    }
    s = socket(
        1 as libc::c_int,
        SOCK_DGRAM as libc::c_int | SOCK_CLOEXEC as libc::c_int
            | SOCK_NONBLOCK as libc::c_int,
        0 as libc::c_int,
    );
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    remote.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy((remote.sun_path).as_mut_ptr(), arg, 107 as libc::c_int as libc::c_ulong);
    if strlen(arg) > 107 as libc::c_int as libc::c_ulong {
        remote.sun_path[107 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    remoteLength = strlen((remote.sun_path).as_mut_ptr()) as libc::c_int
        + ::std::mem::size_of::<sa_family_t>() as libc::c_ulong as libc::c_int;
    if connect(
        s,
        &mut remote as *mut sockaddr_un as *mut sockaddr,
        remoteLength as socklen_t,
    ) == -(1 as libc::c_int)
    {
        close(s);
        return -(1 as libc::c_int);
    }
    if send(
        s,
        logMessage as *const libc::c_void,
        strlen(logMessage),
        MSG_DONTWAIT as libc::c_int | MSG_NOSIGNAL as libc::c_int,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        close(s);
        return -(1 as libc::c_int);
    }
    close(s);
    return strlen(logMessage) as libc::c_int;
}
