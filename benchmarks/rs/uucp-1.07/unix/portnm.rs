use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;
pub type boolean = libc::c_int;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub unsafe extern "C" fn zsysdep_port_name(
    mut ftcp_port: *mut boolean,
) -> *const libc::c_char {
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    *ftcp_port = 0 as libc::c_int;
    let mut clen: size_t = 0;
    let mut s: sockaddr = sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    };
    clen = ::std::mem::size_of::<sockaddr>() as libc::c_ulong;
    if getsockname(0 as libc::c_int, &mut s, &mut clen as *mut size_t as *mut socklen_t)
        == 0 as libc::c_int
    {
        *ftcp_port = 1 as libc::c_int;
    }
    z = ttyname(0 as libc::c_int);
    if z.is_null() {
        return 0 as *const libc::c_char;
    }
    if strncmp(
        z,
        b"/dev/\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        return z
            .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize))
    } else {
        return z
    };
}
