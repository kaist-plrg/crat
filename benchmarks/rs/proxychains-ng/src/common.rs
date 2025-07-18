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
pub static mut proxy_type_strmap: [*const libc::c_char; 3] = [
    b"http\0" as *const u8 as *const libc::c_char,
    b"socks4\0" as *const u8 as *const libc::c_char,
    b"socks5\0" as *const u8 as *const libc::c_char,
];
pub static mut chain_type_strmap: [*const libc::c_char; 4] = [
    b"dynamic_chain\0" as *const u8 as *const libc::c_char,
    b"strict_chain\0" as *const u8 as *const libc::c_char,
    b"random_chain\0" as *const u8 as *const libc::c_char,
    b"round_robin_chain\0" as *const u8 as *const libc::c_char,
];
pub static mut proxy_state_strmap: [*const libc::c_char; 4] = [
    b"play\0" as *const u8 as *const libc::c_char,
    b"down\0" as *const u8 as *const libc::c_char,
    b"blocked\0" as *const u8 as *const libc::c_char,
    b"busy\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn pc_isnumericipv4(
    mut ipstring: *const libc::c_char,
) -> libc::c_int {
    let mut x: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut d: size_t = 0 as libc::c_int as size_t;
    let mut wasdot: libc::c_int = 0 as libc::c_int;
    loop {
        match *ipstring.offset(x as isize) as libc::c_int {
            0 => {
                break;
            }
            46 => {
                if n == 0 || wasdot != 0 {
                    return 0 as libc::c_int;
                }
                d = d.wrapping_add(1);
                d;
                wasdot = 1 as libc::c_int;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                n = n.wrapping_add(1);
                n;
                wasdot = 0 as libc::c_int;
            }
            _ => return 0 as libc::c_int,
        }
        x = x.wrapping_add(1);
        x;
    }
    if d == 3 as libc::c_int as libc::c_ulong && n >= 4 as libc::c_int as libc::c_ulong
        && n <= 12 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pc_stringfromipv4(
    mut ip_buf_4_bytes: *mut libc::c_uchar,
    mut outbuf_16_bytes: *mut libc::c_char,
) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut o: *mut libc::c_char = outbuf_16_bytes;
    let mut n: libc::c_uchar = 0;
    p = ip_buf_4_bytes;
    while p < ip_buf_4_bytes.offset(4 as libc::c_int as isize) {
        n = *p;
        if *p as libc::c_int >= 100 as libc::c_int {
            if *p as libc::c_int >= 200 as libc::c_int {
                let fresh0 = o;
                o = o.offset(1);
                *fresh0 = '2' as i32 as libc::c_char;
            } else {
                let fresh1 = o;
                o = o.offset(1);
                *fresh1 = '1' as i32 as libc::c_char;
            }
            n = (n as libc::c_int % 100 as libc::c_int) as libc::c_uchar;
        }
        if *p as libc::c_int >= 10 as libc::c_int {
            let fresh2 = o;
            o = o.offset(1);
            *fresh2 = (n as libc::c_int / 10 as libc::c_int + '0' as i32)
                as libc::c_char;
            n = (n as libc::c_int % 10 as libc::c_int) as libc::c_uchar;
        }
        let fresh3 = o;
        o = o.offset(1);
        *fresh3 = (n as libc::c_int + '0' as i32) as libc::c_char;
        let fresh4 = o;
        o = o.offset(1);
        *fresh4 = '.' as i32 as libc::c_char;
        p = p.offset(1);
        p;
    }
    *o.offset(-(1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
}
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
    if !(check_path(path) != 0) {
        path = getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char);
        if !(check_path(path) != 0) {
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
            if !(check_path(path) != 0) {
                path = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
                snprintf(
                    pbuf,
                    bufsize,
                    b"%s/.proxychains/%s\0" as *const u8 as *const libc::c_char,
                    path,
                    b"proxychains.conf\0" as *const u8 as *const libc::c_char,
                );
                path = pbuf;
                if !(check_path(path) != 0) {
                    path = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
                    snprintf(
                        pbuf,
                        bufsize,
                        b"%s/config/settings/%s\0" as *const u8 as *const libc::c_char,
                        path,
                        b"proxychains.conf\0" as *const u8 as *const libc::c_char,
                    );
                    path = pbuf;
                    if !(check_path(path) != 0) {
                        path = b"/usr/local/etc/proxychains.conf\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        if !(check_path(path) != 0) {
                            path = b"/etc/proxychains.conf\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            if !(check_path(path) != 0) {
                                perror(
                                    b"couldnt find configuration file\0" as *const u8
                                        as *const libc::c_char,
                                );
                                exit(1 as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
    }
    return path;
}
