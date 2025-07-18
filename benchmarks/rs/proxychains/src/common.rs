use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn check_path(mut path: *mut libc::c_char) -> libc::c_int {
    if path.is_null() {
        return 0 as libc::c_int;
    }
    return (access(path, 4 as libc::c_int) != -(1 as libc::c_int)) as libc::c_int;
}
pub unsafe extern "C" fn get_config_path(
    mut default_path: *mut libc::c_char,
    mut pbuf: *mut libc::c_char,
    mut bufsize: size_t,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut path: *mut libc::c_char = default_path;
    if check_path(path) != 0 {
        return path;
    }
    if !pbuf.is_null() {
        path = getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char);
        if check_path(path) != 0 {
            return path;
        }
        path = getcwd(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        snprintf(
            pbuf,
            bufsize,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            path,
            b"proxychains.conf\0" as *const u8 as *const libc::c_char,
        );
        path = pbuf;
        if check_path(path) != 0 {
            return path;
        }
        path = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
        snprintf(
            pbuf,
            bufsize,
            b"%s/.proxychains/%s\0" as *const u8 as *const libc::c_char,
            path,
            b"proxychains.conf\0" as *const u8 as *const libc::c_char,
        );
        path = pbuf;
        if check_path(path) != 0 {
            return path;
        }
        path = b"/usr/local/etc/proxychains.conf\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        if check_path(path) != 0 {
            return path;
        }
        path = b"/etc/proxychains.conf\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        if check_path(path) != 0 {
            return path;
        }
    }
    perror(b"couldnt find configuration file\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
