use ::libc;
extern "C" {
    pub type _constraint;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn constraint_init(value: value_t) -> *mut constraint_t;
    fn constraint_set(
        con: *mut constraint_t,
        prefix: uint32_t,
        len: libc::c_int,
        value: value_t,
    );
    fn constraint_lookup_ip(con: *mut constraint_t, address: uint32_t) -> value_t;
    fn constraint_count_ips(con: *mut constraint_t, value: value_t) -> uint64_t;
    fn constraint_lookup_index(
        con: *mut constraint_t,
        index: uint64_t,
        value: value_t,
    ) -> uint32_t;
    fn constraint_paint_value(con: *mut constraint_t, value: value_t);
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bl_cidr_node {
    pub ip_address: uint32_t,
    pub prefix_len: libc::c_int,
    pub next: *mut bl_cidr_node,
}
pub type bl_cidr_node_t = bl_cidr_node;
pub type value_t = uint32_t;
pub type constraint_t = _constraint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type bl_ll_t = bl_linked_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bl_linked_list {
    pub first: *mut bl_cidr_node_t,
    pub last: *mut bl_cidr_node_t,
    pub len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
pub type in_port_t = uint16_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type socklen_t = __socklen_t;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut constraint: *mut constraint_t = 0 as *const constraint_t as *mut constraint_t;
static mut blocklisted_cidrs: *mut bl_ll_t = 0 as *const bl_ll_t as *mut bl_ll_t;
static mut allowlisted_cidrs: *mut bl_ll_t = 0 as *const bl_ll_t as *mut bl_ll_t;
pub unsafe extern "C" fn bl_ll_add(
    mut l: *mut bl_ll_t,
    mut addr: in_addr,
    mut p: uint16_t,
) {
    if !l.is_null() {} else {
        __assert_fail(
            b"l\0" as *const u8 as *const libc::c_char,
            b"blocklist.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void bl_ll_add(bl_ll_t *, struct in_addr, uint16_t)\0"))
                .as_ptr(),
        );
    }
    'c_1455: {
        if !l.is_null() {} else {
            __assert_fail(
                b"l\0" as *const u8 as *const libc::c_char,
                b"blocklist.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void bl_ll_add(bl_ll_t *, struct in_addr, uint16_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut new: *mut bl_cidr_node_t = xmalloc(
        ::std::mem::size_of::<bl_cidr_node_t>() as libc::c_ulong,
    ) as *mut bl_cidr_node_t;
    (*new).next = 0 as *mut bl_cidr_node;
    (*new).ip_address = addr.s_addr;
    (*new).prefix_len = p as libc::c_int;
    if ((*l).first).is_null() {
        (*l).first = new;
    } else {
        (*(*l).last).next = new;
    }
    (*l).last = new;
    (*l).len = ((*l).len).wrapping_add(1);
    (*l).len;
}
pub unsafe extern "C" fn get_blocklisted_cidrs() -> *mut bl_cidr_node_t {
    return (*blocklisted_cidrs).first;
}
pub unsafe extern "C" fn get_allowlisted_cidrs() -> *mut bl_cidr_node_t {
    return (*allowlisted_cidrs).first;
}
pub unsafe extern "C" fn blocklist_lookup_index(mut index: uint64_t) -> uint32_t {
    return __bswap_32(
        constraint_lookup_index(constraint, index, 1 as libc::c_int as value_t),
    );
}
pub unsafe extern "C" fn blocklist_is_allowed(mut s_addr: uint32_t) -> libc::c_int {
    return (constraint_lookup_ip(constraint, __bswap_32(s_addr))
        == 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn _add_constraint(
    mut addr: in_addr,
    mut prefix_len: libc::c_int,
    mut value: libc::c_int,
) {
    constraint_set(constraint, __bswap_32(addr.s_addr), prefix_len, value as value_t);
    if value == 1 as libc::c_int {
        bl_ll_add(allowlisted_cidrs, addr, prefix_len as uint16_t);
    } else if value == 0 as libc::c_int {
        bl_ll_add(blocklisted_cidrs, addr, prefix_len as uint16_t);
    } else {
        log_fatal(
            b"blocklist\0" as *const u8 as *const libc::c_char,
            b"unknown type of blocklist operation specified\0" as *const u8
                as *const libc::c_char,
        );
    };
}
pub unsafe extern "C" fn blocklist_prefix(
    mut ip: *mut libc::c_char,
    mut prefix_len: libc::c_int,
) {
    let mut addr: in_addr = in_addr { s_addr: 0 };
    addr.s_addr = inet_addr(ip);
    _add_constraint(addr, prefix_len, 0 as libc::c_int);
}
pub unsafe extern "C" fn allowlist_prefix(
    mut ip: *mut libc::c_char,
    mut prefix_len: libc::c_int,
) {
    let mut addr: in_addr = in_addr { s_addr: 0 };
    addr.s_addr = inet_addr(ip);
    _add_constraint(addr, prefix_len, 1 as libc::c_int);
}
unsafe extern "C" fn init_from_string(
    mut ip: *mut libc::c_char,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut prefix_len: libc::c_int = 32 as libc::c_int;
    let mut slash: *mut libc::c_char = strchr(ip, '/' as i32);
    if !slash.is_null() {
        *slash = '\0' as i32 as libc::c_char;
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: *mut libc::c_char = slash.offset(1 as libc::c_int as isize);
        *__errno_location() = 0 as libc::c_int;
        prefix_len = strtol(len, &mut end, 10 as libc::c_int) as libc::c_int;
        if end == len || *__errno_location() != 0 as libc::c_int
            || prefix_len < 0 as libc::c_int || prefix_len > 32 as libc::c_int
        {
            log_fatal(
                b"constraint\0" as *const u8 as *const libc::c_char,
                b"'%s' is not a valid prefix length\0" as *const u8
                    as *const libc::c_char,
                len,
            );
        }
    }
    let mut addr: in_addr = in_addr { s_addr: 0 };
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if inet_aton(ip, &mut addr) == 0 as libc::c_int {
        let mut hint: addrinfo = addrinfo {
            ai_flags: 0,
            ai_family: 0,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: 0 as *mut sockaddr,
            ai_canonname: 0 as *mut libc::c_char,
            ai_next: 0 as *mut addrinfo,
        };
        let mut res: *mut addrinfo = 0 as *mut addrinfo;
        memset(
            &mut hint as *mut addrinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        hint.ai_family = 2 as libc::c_int;
        let mut r: libc::c_int = getaddrinfo(
            ip,
            0 as *const libc::c_char,
            &mut hint,
            &mut res,
        );
        if r != 0 {
            log_error(
                b"constraint\0" as *const u8 as *const libc::c_char,
                b"'%s' is not a valid IP address or hostname\0" as *const u8
                    as *const libc::c_char,
                ip,
            );
            return -(1 as libc::c_int);
        }
        let mut aip: *mut addrinfo = res;
        while !aip.is_null() {
            if !((*aip).ai_family != 2 as libc::c_int) {
                let mut sa: *mut sockaddr_in = (*aip).ai_addr as *mut sockaddr_in;
                memcpy(
                    &mut addr as *mut in_addr as *mut libc::c_void,
                    &mut (*sa).sin_addr as *mut in_addr as *const libc::c_void,
                    ::std::mem::size_of::<in_addr>() as libc::c_ulong,
                );
                log_debug(
                    b"constraint\0" as *const u8 as *const libc::c_char,
                    b"%s retrieved by hostname\0" as *const u8 as *const libc::c_char,
                    inet_ntoa(addr),
                );
                ret = 0 as libc::c_int;
                _add_constraint(addr, prefix_len, value);
            }
            aip = (*aip).ai_next;
        }
    } else {
        _add_constraint(addr, prefix_len, value);
        return 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn init_from_file(
    mut file: *mut libc::c_char,
    mut name: *const libc::c_char,
    mut value: libc::c_int,
    mut ignore_invalid_hosts: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 1000] = [0; 1000];
    fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        log_fatal(
            name,
            b"unable to open %s file: %s: %s\0" as *const u8 as *const libc::c_char,
            name,
            file,
            strerror(*__errno_location()),
        );
    }
    while !(fgets(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        let mut comment: *mut libc::c_char = strchr(line.as_mut_ptr(), '#' as i32);
        if !comment.is_null() {
            *comment = '\0' as i32 as libc::c_char;
        }
        let mut ip: [libc::c_char; 256] = [0; 256];
        if sscanf(
            line.as_mut_ptr(),
            b"%256s\0" as *const u8 as *const libc::c_char,
            ip.as_mut_ptr(),
        ) == -(1 as libc::c_int)
        {
            continue;
        }
        if init_from_string(ip.as_mut_ptr(), value) != 0 {
            if ignore_invalid_hosts == 0 {
                log_fatal(
                    name,
                    b"unable to parse %s file: %s\0" as *const u8 as *const libc::c_char,
                    name,
                    file,
                );
            }
        }
    }
    fclose(fp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_from_array(
    mut cidrs: *mut *mut libc::c_char,
    mut len: size_t,
    mut value: libc::c_int,
    mut ignore_invalid_hosts: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len as libc::c_int {
        let mut ret: libc::c_int = init_from_string(*cidrs.offset(i as isize), value);
        if ret != 0 && ignore_invalid_hosts == 0 {
            log_fatal(
                b"constraint\0" as *const u8 as *const libc::c_char,
                b"Unable to init from CIDR list\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn blocklist_count_allowed() -> uint64_t {
    if !constraint.is_null() {} else {
        __assert_fail(
            b"constraint\0" as *const u8 as *const libc::c_char,
            b"blocklist.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"uint64_t blocklist_count_allowed(void)\0"))
                .as_ptr(),
        );
    }
    'c_1652: {
        if !constraint.is_null() {} else {
            __assert_fail(
                b"constraint\0" as *const u8 as *const libc::c_char,
                b"blocklist.c\0" as *const u8 as *const libc::c_char,
                205 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"uint64_t blocklist_count_allowed(void)\0"))
                    .as_ptr(),
            );
        }
    };
    return constraint_count_ips(constraint, 1 as libc::c_int as value_t);
}
pub unsafe extern "C" fn blocklist_count_not_allowed() -> uint64_t {
    if !constraint.is_null() {} else {
        __assert_fail(
            b"constraint\0" as *const u8 as *const libc::c_char,
            b"blocklist.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"uint64_t blocklist_count_not_allowed(void)\0"))
                .as_ptr(),
        );
    }
    'c_2543: {
        if !constraint.is_null() {} else {
            __assert_fail(
                b"constraint\0" as *const u8 as *const libc::c_char,
                b"blocklist.c\0" as *const u8 as *const libc::c_char,
                211 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"uint64_t blocklist_count_not_allowed(void)\0"))
                    .as_ptr(),
            );
        }
    };
    return constraint_count_ips(constraint, 0 as libc::c_int as value_t);
}
pub unsafe extern "C" fn blocklist_ip_to_index(mut ip: uint32_t) -> uint32_t {
    if !constraint.is_null() {} else {
        __assert_fail(
            b"constraint\0" as *const u8 as *const libc::c_char,
            b"blocklist.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"uint32_t blocklist_ip_to_index(uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_2595: {
        if !constraint.is_null() {} else {
            __assert_fail(
                b"constraint\0" as *const u8 as *const libc::c_char,
                b"blocklist.c\0" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"uint32_t blocklist_ip_to_index(uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut ip_hostorder: uint32_t = __bswap_32(ip);
    return constraint_lookup_ip(constraint, ip_hostorder);
}
pub unsafe extern "C" fn blocklist_init(
    mut allowlist_filename: *mut libc::c_char,
    mut blocklist_filename: *mut libc::c_char,
    mut allowlist_entries: *mut *mut libc::c_char,
    mut allowlist_entries_len: size_t,
    mut blocklist_entries: *mut *mut libc::c_char,
    mut blocklist_entries_len: size_t,
    mut ignore_invalid_hosts: libc::c_int,
) -> libc::c_int {
    if constraint.is_null() {} else {
        __assert_fail(
            b"!constraint\0" as *const u8 as *const libc::c_char,
            b"blocklist.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int blocklist_init(char *, char *, char **, size_t, char **, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2497: {
        if constraint.is_null() {} else {
            __assert_fail(
                b"!constraint\0" as *const u8 as *const libc::c_char,
                b"blocklist.c\0" as *const u8 as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"int blocklist_init(char *, char *, char **, size_t, char **, size_t, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    blocklisted_cidrs = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<bl_ll_t>() as libc::c_ulong,
    ) as *mut bl_ll_t;
    allowlisted_cidrs = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<bl_ll_t>() as libc::c_ulong,
    ) as *mut bl_ll_t;
    if !allowlist_filename.is_null() && !allowlist_entries.is_null() {
        log_warn(
            b"allowlist\0" as *const u8 as *const libc::c_char,
            b"both a allowlist file and destination addresses were specified. The union of these two sources will be utilized.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !allowlist_filename.is_null()
        || allowlist_entries_len > 0 as libc::c_int as libc::c_ulong
    {
        constraint = constraint_init(0 as libc::c_int as value_t);
        log_debug(
            b"constraint\0" as *const u8 as *const libc::c_char,
            b"blocklisting 0.0.0.0/0\0" as *const u8 as *const libc::c_char,
        );
        if !allowlist_filename.is_null() {
            init_from_file(
                allowlist_filename,
                b"allowlist\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                ignore_invalid_hosts,
            );
        }
        if !allowlist_entries.is_null() {
            init_from_array(
                allowlist_entries,
                allowlist_entries_len,
                1 as libc::c_int,
                ignore_invalid_hosts,
            );
        }
    } else {
        log_debug(
            b"blocklist\0" as *const u8 as *const libc::c_char,
            b"no allowlist file or allowlist entries provided\0" as *const u8
                as *const libc::c_char,
        );
        constraint = constraint_init(1 as libc::c_int as value_t);
    }
    if !blocklist_filename.is_null() {
        init_from_file(
            blocklist_filename,
            b"blocklist\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            ignore_invalid_hosts,
        );
    }
    if !blocklist_entries.is_null() {
        init_from_array(
            blocklist_entries,
            blocklist_entries_len,
            0 as libc::c_int,
            ignore_invalid_hosts,
        );
    }
    init_from_string(
        strdup(b"0.0.0.0\0" as *const u8 as *const libc::c_char),
        0 as libc::c_int,
    );
    constraint_paint_value(constraint, 1 as libc::c_int as value_t);
    let mut allowed: uint64_t = blocklist_count_allowed();
    log_debug(
        b"constraint\0" as *const u8 as *const libc::c_char,
        b"%lu addresses (%0.0f%% of address space) can be scanned\0" as *const u8
            as *const libc::c_char,
        allowed,
        allowed as libc::c_double * 100.0f64
            / ((1 as libc::c_int as libc::c_longlong) << 32 as libc::c_int)
                as libc::c_double,
    );
    if allowed == 0 {
        log_error(
            b"blocklist\0" as *const u8 as *const libc::c_char,
            b"no addresses are eligible to be scanned in the current configuration. This may be because the blocklist being used by ZMap (%s) prevents any addresses from receiving probe packets.\0"
                as *const u8 as *const libc::c_char,
            blocklist_filename,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
