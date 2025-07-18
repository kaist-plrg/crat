use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn endpwent();
    fn getpwent() -> *mut passwd;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut passwd = 0 as *mut passwd;
    loop {
        p = getpwent();
        if p.is_null() {
            break;
        }
        printf(
            b"%s:%s:%ld:%ld:%s:%s:%s\n\0" as *const u8 as *const libc::c_char,
            (*p).pw_name,
            (*p).pw_passwd,
            (*p).pw_uid as libc::c_long,
            (*p).pw_gid as libc::c_long,
            (*p).pw_gecos,
            (*p).pw_dir,
            (*p).pw_shell,
        );
    }
    endpwent();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
