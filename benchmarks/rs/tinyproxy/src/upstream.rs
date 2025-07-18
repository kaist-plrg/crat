use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn hostspec_match(ip: *const libc::c_char, h: *const hostspec) -> libc::c_int;
    fn hostspec_parse(domain: *mut libc::c_char, h: *mut hostspec) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn basicauth_string(
        user: *const libc::c_char,
        pass: *const libc::c_char,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type hostspec_type = libc::c_uint;
pub const HST_NUMERIC: hostspec_type = 2;
pub const HST_STRING: hostspec_type = 1;
pub const HST_NONE: hostspec_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostspec {
    pub type_0: hostspec_type,
    pub address: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: *mut libc::c_char,
    pub ip: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub network: [libc::c_uchar; 16],
    pub mask: [libc::c_uchar; 16],
}
pub type upstream_build_error = libc::c_uint;
pub const UBE_NETMASK: upstream_build_error = 6;
pub const UBE_INVPARAMS: upstream_build_error = 5;
pub const UBE_INVHOST: upstream_build_error = 4;
pub const UBE_EDOMAIN: upstream_build_error = 3;
pub const UBE_USERLEN: upstream_build_error = 2;
pub const UBE_OOM: upstream_build_error = 1;
pub const UBE_SUCCESS: upstream_build_error = 0;
pub type proxy_type = libc::c_uint;
pub const PT_SOCKS5: proxy_type = 3;
pub const PT_SOCKS4: proxy_type = 2;
pub const PT_HTTP: proxy_type = 1;
pub const PT_NONE: proxy_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct upstream {
    pub next: *mut upstream,
    pub host: *mut libc::c_char,
    pub ua: C2RustUnnamed_1,
    pub pass: *mut libc::c_char,
    pub port: libc::c_int,
    pub target: hostspec,
    pub type_0: proxy_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub user: *mut libc::c_char,
    pub authstr: *mut libc::c_char,
}
pub unsafe extern "C" fn proxy_type_name(mut type_0: proxy_type) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        0 => return b"none\0" as *const u8 as *const libc::c_char,
        1 => return b"http\0" as *const u8 as *const libc::c_char,
        2 => return b"socks4\0" as *const u8 as *const libc::c_char,
        3 => return b"socks5\0" as *const u8 as *const libc::c_char,
        _ => return b"unknown\0" as *const u8 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn upstream_build_error_string(
    mut ube: upstream_build_error,
) -> *const libc::c_char {
    static mut emap: [*const libc::c_char; 7] = [
        b"\0" as *const u8 as *const libc::c_char,
        b"Unable to allocate memory in upstream_build()\0" as *const u8
            as *const libc::c_char,
        b"User / pass in upstream config too long\0" as *const u8 as *const libc::c_char,
        b"Nonsense upstream none rule: empty domain\0" as *const u8
            as *const libc::c_char,
        b"Nonsense upstream rule: invalid host or port\0" as *const u8
            as *const libc::c_char,
        b"Nonsense upstream rule: invalid parameters\0" as *const u8
            as *const libc::c_char,
        b"Nonsense upstream rule: failed to parse netmask\0" as *const u8
            as *const libc::c_char,
    ];
    return emap[ube as usize];
}
unsafe extern "C" fn upstream_build(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut domain: *mut libc::c_char,
    mut user: *const libc::c_char,
    mut pass: *const libc::c_char,
    mut type_0: proxy_type,
    mut ube: *mut upstream_build_error,
) -> *mut upstream {
    let mut current_block: u64;
    let mut up: *mut upstream = 0 as *mut upstream;
    *ube = UBE_SUCCESS;
    up = malloc(::std::mem::size_of::<upstream>() as libc::c_ulong) as *mut upstream;
    if up.is_null() {
        *ube = UBE_OOM;
        return 0 as *mut upstream;
    }
    (*up).type_0 = type_0;
    (*up).target.type_0 = HST_NONE;
    (*up).pass = 0 as *mut libc::c_char;
    (*up).ua.user = (*up).pass;
    (*up).host = (*up).ua.user;
    if !user.is_null() {
        if type_0 as libc::c_uint == PT_HTTP as libc::c_int as libc::c_uint {
            let mut b: [libc::c_char; 345] = [0; 345];
            let mut ret: ssize_t = 0;
            ret = basicauth_string(
                user,
                pass,
                b.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 345]>() as libc::c_ulong,
            );
            if ret == 0 as libc::c_int as libc::c_long {
                *ube = UBE_USERLEN;
                return 0 as *mut upstream;
            }
            (*up).ua.authstr = strdup(b.as_mut_ptr());
        } else {
            (*up).ua.user = strdup(user);
            (*up).pass = strdup(pass);
        }
    }
    if domain.is_null() {
        if type_0 as libc::c_uint == PT_NONE as libc::c_int as libc::c_uint {
            current_block = 15904375183555213903;
        } else if host.is_null() || *host.offset(0 as libc::c_int as isize) == 0
            || port < 1 as libc::c_int
        {
            *ube = UBE_INVHOST;
            current_block = 17113658446281256821;
        } else {
            (*up).host = strdup(host);
            (*up).port = port;
            log_message(
                6 as libc::c_int,
                b"Added upstream %s %s:%d for [default]\0" as *const u8
                    as *const libc::c_char,
                proxy_type_name(type_0),
                host,
                port,
            );
            current_block = 2604890879466389055;
        }
    } else {
        if type_0 as libc::c_uint == PT_NONE as libc::c_int as libc::c_uint {
            if *domain.offset(0 as libc::c_int as isize) == 0 {
                current_block = 15904375183555213903;
            } else {
                current_block = 11459959175219260272;
            }
        } else if host.is_null() || *host.offset(0 as libc::c_int as isize) == 0
            || *domain.offset(0 as libc::c_int as isize) == 0
        {
            *ube = UBE_INVPARAMS;
            current_block = 17113658446281256821;
        } else {
            (*up).host = strdup(host);
            (*up).port = port;
            current_block = 11459959175219260272;
        }
        match current_block {
            17113658446281256821 => {}
            15904375183555213903 => {}
            _ => {
                if hostspec_parse(domain, &mut (*up).target) != 0
                    || (*up).target.type_0 as libc::c_uint
                        == HST_NONE as libc::c_int as libc::c_uint
                {
                    *ube = UBE_NETMASK;
                    current_block = 17113658446281256821;
                } else {
                    if type_0 as libc::c_uint == PT_NONE as libc::c_int as libc::c_uint {
                        log_message(
                            6 as libc::c_int,
                            b"Added upstream none for %s\0" as *const u8
                                as *const libc::c_char,
                            domain,
                        );
                    } else {
                        log_message(
                            6 as libc::c_int,
                            b"Added upstream %s %s:%d for %s\0" as *const u8
                                as *const libc::c_char,
                            proxy_type_name(type_0),
                            host,
                            port,
                            domain,
                        );
                    }
                    current_block = 2604890879466389055;
                }
            }
        }
    }
    match current_block {
        2604890879466389055 => return up,
        15904375183555213903 => {
            *ube = UBE_EDOMAIN;
        }
        _ => {}
    }
    free((*up).ua.user as *mut libc::c_void);
    (*up).ua.user = 0 as *mut libc::c_char;
    free((*up).pass as *mut libc::c_void);
    (*up).pass = 0 as *mut libc::c_char;
    free((*up).host as *mut libc::c_void);
    (*up).host = 0 as *mut libc::c_char;
    if (*up).target.type_0 as libc::c_uint == HST_STRING as libc::c_int as libc::c_uint {
        free((*up).target.address.string as *mut libc::c_void);
        (*up).target.address.string = 0 as *mut libc::c_char;
    }
    free(up as *mut libc::c_void);
    up = 0 as *mut upstream;
    return 0 as *mut upstream;
}
pub unsafe extern "C" fn upstream_add(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut domain: *mut libc::c_char,
    mut user: *const libc::c_char,
    mut pass: *const libc::c_char,
    mut type_0: proxy_type,
    mut upstream_list: *mut *mut upstream,
) -> upstream_build_error {
    let mut current_block: u64;
    let mut up: *mut upstream = 0 as *mut upstream;
    let mut ube: upstream_build_error = UBE_SUCCESS;
    up = upstream_build(host, port, domain, user, pass, type_0, &mut ube);
    if up.is_null() {
        return ube;
    }
    if (*up).target.type_0 as libc::c_uint == HST_NONE as libc::c_int as libc::c_uint {
        let mut tmp: *mut upstream = *upstream_list;
        loop {
            if tmp.is_null() {
                current_block = 1841672684692190573;
                break;
            }
            if (*tmp).target.type_0 as libc::c_uint
                == HST_NONE as libc::c_int as libc::c_uint
            {
                log_message(
                    4 as libc::c_int,
                    b"Duplicate default upstream\0" as *const u8 as *const libc::c_char,
                );
                current_block = 16360901136203613893;
                break;
            } else {
                if ((*tmp).next).is_null() {
                    (*up).next = 0 as *mut upstream;
                    (*tmp).next = up;
                    return ube;
                }
                tmp = (*tmp).next;
            }
        }
        match current_block {
            1841672684692190573 => {}
            _ => {
                free((*up).host as *mut libc::c_void);
                (*up).host = 0 as *mut libc::c_char;
                if (*up).target.type_0 as libc::c_uint
                    == HST_STRING as libc::c_int as libc::c_uint
                {
                    free((*up).target.address.string as *mut libc::c_void);
                    (*up).target.address.string = 0 as *mut libc::c_char;
                }
                free(up as *mut libc::c_void);
                up = 0 as *mut upstream;
                return ube;
            }
        }
    }
    (*up).next = *upstream_list;
    *upstream_list = up;
    return ube;
}
pub unsafe extern "C" fn upstream_get(
    mut host: *mut libc::c_char,
    mut up: *mut upstream,
) -> *mut upstream {
    while !up.is_null() {
        if (*up).target.type_0 as libc::c_uint == HST_NONE as libc::c_int as libc::c_uint
        {
            break;
        }
        if hostspec_match(host, &mut (*up).target) != 0 {
            break;
        }
        up = (*up).next;
    }
    if !up.is_null() && ((*up).host).is_null() {
        up = 0 as *mut upstream;
    }
    if !up.is_null() {
        log_message(
            6 as libc::c_int,
            b"Found upstream proxy %s %s:%d for %s\0" as *const u8
                as *const libc::c_char,
            proxy_type_name((*up).type_0),
            (*up).host,
            (*up).port,
            host,
        );
    } else {
        log_message(
            6 as libc::c_int,
            b"No upstream proxy for %s\0" as *const u8 as *const libc::c_char,
            host,
        );
    }
    return up;
}
pub unsafe extern "C" fn free_upstream_list(mut up: *mut upstream) {
    while !up.is_null() {
        let mut tmp: *mut upstream = up;
        up = (*up).next;
        if (*tmp).target.type_0 as libc::c_uint
            == HST_STRING as libc::c_int as libc::c_uint
        {
            free((*tmp).target.address.string as *mut libc::c_void);
            (*tmp).target.address.string = 0 as *mut libc::c_char;
        }
        free((*tmp).host as *mut libc::c_void);
        (*tmp).host = 0 as *mut libc::c_char;
        free(tmp as *mut libc::c_void);
        tmp = 0 as *mut upstream;
    }
}
