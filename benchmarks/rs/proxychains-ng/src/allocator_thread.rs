use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn dalias_hash(s0: *mut libc::c_char) -> uint32_t;
    static mut remote_dns_subnet: libc::c_uint;
}
pub type __int16_t = libc::c_short;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type ssize_t = __ssize_t;
pub type int16_t = __int16_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type4 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct at_msg {
    pub h: at_msghdr,
    pub m: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub host: [libc::c_char; 260],
    pub ip: ip_type4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct at_msghdr {
    pub msgtype: libc::c_uchar,
    pub reserved: libc::c_char,
    pub datalen: libc::c_ushort,
}
pub type at_direction = libc::c_uint;
pub const ATD_MAX: at_direction = 2;
pub const ATD_CLIENT: at_direction = 1;
pub const ATD_SERVER: at_direction = 0;
pub const ATM_EXIT: at_msgtype = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_hash_tuple {
    pub hash: uint32_t,
    pub string: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_ip_lookup_table {
    pub counter: uint32_t,
    pub capa: uint32_t,
    pub list: *mut *mut string_hash_tuple,
}
pub const ATM_GETNAME: at_msgtype = 1;
pub const ATM_GETIP: at_msgtype = 0;
pub type at_msgtype = libc::c_uint;
pub const ATM_FAIL: at_msgtype = 2;
unsafe extern "C" fn dumpstring(
    mut s: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = malloc(len) as *mut libc::c_char;
    if !p.is_null() {
        memcpy(p as *mut libc::c_void, s as *const libc::c_void, len);
    }
    return p as *mut libc::c_void;
}
static mut internal_ips_lock: *mut pthread_mutex_t = 0 as *const pthread_mutex_t
    as *mut pthread_mutex_t;
static mut internal_ips: *mut internal_ip_lookup_table = 0
    as *const internal_ip_lookup_table as *mut internal_ip_lookup_table;
pub unsafe extern "C" fn index_from_internal_ip(mut internalip: ip_type4) -> uint32_t {
    let mut tmp: ip_type4 = internalip;
    let mut ret: uint32_t = 0;
    ret = (tmp.octet[3 as libc::c_int as usize] as libc::c_int
        + ((tmp.octet[2 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
        + ((tmp.octet[1 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int))
        as uint32_t;
    ret = (ret as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    return ret;
}
pub unsafe extern "C" fn string_from_internal_ip(
    mut internalip: ip_type4,
) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: uint32_t = index_from_internal_ip(internalip);
    if index < (*internal_ips).counter {
        res = (**((*internal_ips).list).offset(index as isize)).string;
    }
    return res;
}
pub unsafe extern "C" fn make_internal_ip(mut index: uint32_t) -> ip_type4 {
    let mut ret: ip_type4 = ip_type4 { octet: [0; 4] };
    index = index.wrapping_add(1);
    index;
    if index > 0xffffff as libc::c_int as libc::c_uint {
        return ip_type4 {
            as_int: -(1 as libc::c_int) as uint32_t,
        };
    }
    ret
        .octet[0 as libc::c_int
        as usize] = (remote_dns_subnet & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    ret
        .octet[1 as libc::c_int
        as usize] = ((index & 0xff0000 as libc::c_int as libc::c_uint)
        >> 16 as libc::c_int) as libc::c_uchar;
    ret
        .octet[2 as libc::c_int
        as usize] = ((index & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
        as libc::c_uchar;
    ret
        .octet[3 as libc::c_int
        as usize] = (index & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    return ret;
}
unsafe extern "C" fn ip_from_internal_list(
    mut name: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut tmp: string_hash_tuple = string_hash_tuple {
        hash: 0,
        string: 0 as *mut libc::c_char,
    };
    let mut current_block: u64;
    let mut hash: uint32_t = dalias_hash(name);
    let mut i: size_t = 0;
    let mut res: ip_type4 = ip_type4 { octet: [0; 4] };
    let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*internal_ips).counter != 0 {
        i = 0 as libc::c_int as size_t;
        loop {
            if !(i < (*internal_ips).counter as libc::c_ulong) {
                current_block = 13183875560443969876;
                break;
            }
            if (**((*internal_ips).list).offset(i as isize)).hash == hash
                && strcmp(name, (**((*internal_ips).list).offset(i as isize)).string)
                    == 0
            {
                res = make_internal_ip(i as uint32_t);
                current_block = 8601410800669747136;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    } else {
        current_block = 13183875560443969876;
    }
    match current_block {
        13183875560443969876 => {
            if (*internal_ips).capa
                < ((*internal_ips).counter)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
            {
                new_mem = realloc(
                    (*internal_ips).list as *mut libc::c_void,
                    (((*internal_ips).capa)
                        .wrapping_add(16 as libc::c_int as libc::c_uint)
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                );
                if !new_mem.is_null() {
                    (*internal_ips)
                        .capa = ((*internal_ips).capa as libc::c_uint)
                        .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    (*internal_ips).list = new_mem as *mut *mut string_hash_tuple;
                    current_block = 7175849428784450219;
                } else {
                    current_block = 17500079516916021833;
                }
            } else {
                current_block = 7175849428784450219;
            }
            match current_block {
                7175849428784450219 => {
                    res = make_internal_ip((*internal_ips).counter);
                    if (res.as_int
                        == ip_type4 {
                            as_int: -(1 as libc::c_int) as uint32_t,
                        }
                            .as_int)
                    {
                        current_block = 17500079516916021833;
                    } else {
                        tmp = {
                            let mut init = string_hash_tuple {
                                hash: 0 as libc::c_int as uint32_t,
                                string: 0 as *mut libc::c_char,
                            };
                            init
                        };
                        new_mem = dumpstring(
                            &mut tmp as *mut string_hash_tuple as *mut libc::c_char,
                            ::std::mem::size_of::<string_hash_tuple>() as libc::c_ulong,
                        );
                        if new_mem.is_null() {
                            current_block = 17500079516916021833;
                        } else {
                            let ref mut fresh0 = *((*internal_ips).list)
                                .offset((*internal_ips).counter as isize);
                            *fresh0 = new_mem as *mut string_hash_tuple;
                            (**((*internal_ips).list)
                                .offset((*internal_ips).counter as isize))
                                .hash = hash;
                            new_mem = dumpstring(name, len);
                            if new_mem.is_null() {
                                let ref mut fresh1 = *((*internal_ips).list)
                                    .offset((*internal_ips).counter as isize);
                                *fresh1 = 0 as *mut string_hash_tuple;
                                current_block = 17500079516916021833;
                            } else {
                                let ref mut fresh2 = (**((*internal_ips).list)
                                    .offset((*internal_ips).counter as isize))
                                    .string;
                                *fresh2 = new_mem as *mut libc::c_char;
                                (*internal_ips)
                                    .counter = ((*internal_ips).counter as libc::c_uint)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t
                                    as uint32_t;
                                current_block = 8601410800669747136;
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                8601410800669747136 => {}
                _ => {
                    return ip_type4 {
                        as_int: -(1 as libc::c_int) as uint32_t,
                    };
                }
            }
        }
        _ => {}
    }
    return res;
}
static mut allocator_thread: pthread_t = 0;
pub static mut req_pipefd: [libc::c_int; 2] = [0; 2];
pub static mut resp_pipefd: [libc::c_int; 2] = [0; 2];
unsafe extern "C" fn wait_data(mut readfd_: libc::c_int) -> libc::c_int {
    let mut fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh3 = &mut __d0;
    let fresh4;
    let fresh5 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh6 = &mut __d1;
    let fresh7;
    let fresh8 = &mut *(fds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    fds
        .fds_bits[(readfd_
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << readfd_
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    let mut ret: libc::c_int = 0;
    loop {
        ret = select(
            readfd_ + 1 as libc::c_int,
            &mut fds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if !(ret <= 0 as libc::c_int) {
            break;
        }
        if !(ret < 0 as libc::c_int) {
            continue;
        }
        let mut e: libc::c_int = *__errno_location();
        if e == 4 as libc::c_int {
            continue;
        }
        let mut emsg: [libc::c_char; 1024] = [0; 1024];
        let mut x: *mut libc::c_char = strerror_r(
            *__errno_location(),
            emsg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        dprintf(
            2 as libc::c_int,
            b"select2: %s\n\0" as *const u8 as *const libc::c_char,
            x,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn trywrite(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut bytes: size_t,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    let mut out: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    loop {
        ret = write(fd, out as *const libc::c_void, bytes);
        match ret {
            -1 => {
                if !(*__errno_location() == 4 as libc::c_int) {
                    break;
                }
            }
            0 => {
                break;
            }
            _ => {
                if ret as libc::c_ulong == bytes || bytes == 0 {
                    return 1 as libc::c_int;
                }
                out = out.offset(ret as isize);
                bytes = (bytes as libc::c_ulong).wrapping_sub(ret as libc::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    return 0 as libc::c_int;
}
static mut destfd: [*mut libc::c_int; 2] = [0 as *const libc::c_int
    as *mut libc::c_int; 2];
unsafe extern "C" fn sendmessage(
    mut dir: at_direction,
    mut msg: *mut at_msg,
) -> libc::c_int {
    if (*msg).h.datalen as libc::c_int <= 256 as libc::c_int {} else {
        __assert_fail(
            b"msg->h.datalen <= MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"int sendmessage(enum at_direction, struct at_msg *)\0"))
                .as_ptr(),
        );
    };
    let mut ret: libc::c_int = trywrite(
        *destfd[dir as usize],
        msg as *mut libc::c_void,
        (::std::mem::size_of::<at_msghdr>() as libc::c_ulong)
            .wrapping_add((*msg).h.datalen as libc::c_ulong),
    );
    if (*msg).h.datalen as libc::c_int <= 256 as libc::c_int {} else {
        __assert_fail(
            b"msg->h.datalen <= MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"int sendmessage(enum at_direction, struct at_msg *)\0"))
                .as_ptr(),
        );
    };
    return ret;
}
unsafe extern "C" fn tryread(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut bytes: size_t,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    let mut out: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    loop {
        ret = read(fd, out as *mut libc::c_void, bytes);
        match ret {
            -1 => {
                if !(*__errno_location() == 4 as libc::c_int) {
                    break;
                }
            }
            0 => {
                break;
            }
            _ => {
                if ret as libc::c_ulong == bytes || bytes == 0 {
                    return 1 as libc::c_int;
                }
                out = out.offset(ret as isize);
                bytes = (bytes as libc::c_ulong).wrapping_sub(ret as libc::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn readmsg(mut fd: libc::c_int, mut msg: *mut at_msg) -> libc::c_int {
    let mut ret: libc::c_int = tryread(
        fd,
        msg as *mut libc::c_void,
        ::std::mem::size_of::<at_msghdr>() as libc::c_ulong,
    );
    if ret != 1 as libc::c_int {
        return ret;
    }
    return tryread(
        fd,
        &mut (*msg).m as *mut C2RustUnnamed as *mut libc::c_void,
        (*msg).h.datalen as size_t,
    );
}
static mut readfd: [*mut libc::c_int; 2] = [0 as *const libc::c_int
    as *mut libc::c_int; 2];
unsafe extern "C" fn getmessage(
    mut dir: at_direction,
    mut msg: *mut at_msg,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    ret = wait_data(*readfd[dir as usize]) as ssize_t;
    if ret != 0 {
        if readmsg(*readfd[dir as usize], msg) == 0 {
            return 0 as libc::c_int;
        }
        if (*msg).h.datalen as libc::c_int <= 256 as libc::c_int {} else {
            __assert_fail(
                b"msg->h.datalen <= MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
                b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int getmessage(enum at_direction, struct at_msg *)\0"))
                    .as_ptr(),
            );
        };
    }
    return ret as libc::c_int;
}
unsafe extern "C" fn threadfunc(mut x: *mut libc::c_void) -> *mut libc::c_void {
    let mut ret: libc::c_int = 0;
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: C2RustUnnamed { host: [0; 260] },
    };
    loop {
        ret = getmessage(ATD_SERVER, &mut msg);
        if !(ret != 0) {
            break;
        }
        match msg.h.msgtype as libc::c_int {
            0 => {
                msg
                    .m
                    .ip = ip_from_internal_list(
                    (msg.m.host).as_mut_ptr(),
                    msg.h.datalen as size_t,
                );
                msg
                    .h
                    .datalen = ::std::mem::size_of::<ip_type4>() as libc::c_ulong
                    as libc::c_ushort;
            }
            1 => {
                let mut host: *mut libc::c_char = string_from_internal_ip(msg.m.ip);
                if !host.is_null() {
                    let mut l: size_t = strlen(host);
                    if l.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        < 256 as libc::c_int as libc::c_ulong
                    {} else {
                        __assert_fail(
                            b"l+1 < MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
                            b"src/allocator_thread.c\0" as *const u8
                                as *const libc::c_char,
                            249 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 25],
                                &[libc::c_char; 25],
                            >(b"void *threadfunc(void *)\0"))
                                .as_ptr(),
                        );
                    };
                    memcpy(
                        (msg.m.host).as_mut_ptr() as *mut libc::c_void,
                        host as *const libc::c_void,
                        l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    msg
                        .h
                        .datalen = l.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_ushort;
                } else {
                    msg.h.datalen = 0 as libc::c_int as libc::c_ushort;
                }
            }
            3 => return 0 as *mut libc::c_void,
            _ => {
                abort();
            }
        }
        ret = sendmessage(ATD_CLIENT, &mut msg);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn at_get_ip_for_host(
    mut host: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: C2RustUnnamed { host: [0; 260] },
    };
    let mut current_block: u64;
    let mut readbuf: ip_type4 = ip_type4 { octet: [0; 4] };
    pthread_mutex_lock(internal_ips_lock);
    if len > 256 as libc::c_int as libc::c_ulong {
        current_block = 13023734125552931536;
    } else {
        msg = {
            let mut init = at_msg {
                h: {
                    let mut init = at_msghdr {
                        msgtype: ATM_GETIP as libc::c_int as libc::c_uchar,
                        reserved: 0,
                        datalen: len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_ushort,
                    };
                    init
                },
                m: C2RustUnnamed { host: [0; 260] },
            };
            init
        };
        memcpy(
            (msg.m.host).as_mut_ptr() as *mut libc::c_void,
            host as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if sendmessage(ATD_SERVER, &mut msg) != 0
            && getmessage(ATD_CLIENT, &mut msg) != 0
        {
            readbuf = msg.m.ip;
            current_block = 11875828834189669668;
        } else {
            current_block = 13023734125552931536;
        }
    }
    match current_block {
        13023734125552931536 => {
            readbuf = ip_type4 {
                as_int: -(1 as libc::c_int) as uint32_t,
            };
        }
        _ => {}
    }
    if msg.h.msgtype as libc::c_int == ATM_GETIP as libc::c_int {} else {
        __assert_fail(
            b"msg.h.msgtype == ATM_GETIP\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"ip_type4 at_get_ip_for_host(char *, size_t)\0"))
                .as_ptr(),
        );
    };
    pthread_mutex_unlock(internal_ips_lock);
    return readbuf;
}
pub unsafe extern "C" fn at_get_host_for_ip(
    mut ip: ip_type4,
    mut readbuf: *mut libc::c_char,
) -> size_t {
    let mut msg: at_msg = {
        let mut init = at_msg {
            h: {
                let mut init = at_msghdr {
                    msgtype: ATM_GETNAME as libc::c_int as libc::c_uchar,
                    reserved: 0,
                    datalen: ::std::mem::size_of::<ip_type4>() as libc::c_ulong
                        as libc::c_ushort,
                };
                init
            },
            m: C2RustUnnamed { ip: ip },
        };
        init
    };
    let mut res: size_t = 0 as libc::c_int as size_t;
    pthread_mutex_lock(internal_ips_lock);
    if sendmessage(ATD_SERVER, &mut msg) != 0 && getmessage(ATD_CLIENT, &mut msg) != 0 {
        if msg.h.datalen as int16_t as libc::c_int <= 0 as libc::c_int {
            res = 0 as libc::c_int as size_t;
        } else {
            memcpy(
                readbuf as *mut libc::c_void,
                (msg.m.host).as_mut_ptr() as *const libc::c_void,
                msg.h.datalen as libc::c_ulong,
            );
            res = (msg.h.datalen as libc::c_int - 1 as libc::c_int) as size_t;
        }
    }
    if msg.h.msgtype as libc::c_int == ATM_GETNAME as libc::c_int {} else {
        __assert_fail(
            b"msg.h.msgtype == ATM_GETNAME\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"size_t at_get_host_for_ip(ip_type4, char *)\0"))
                .as_ptr(),
        );
    };
    pthread_mutex_unlock(internal_ips_lock);
    return res;
}
unsafe extern "C" fn initpipe(mut fds: *mut libc::c_int) {
    let mut retval: libc::c_int = 0;
    retval = pipe2(fds, 0o2000000 as libc::c_int);
    if retval == -(1 as libc::c_int) {
        perror(b"pipe\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn at_init() {
    let mut shm: *mut libc::c_void = mmap(
        0 as *mut libc::c_void,
        4096 as libc::c_int as size_t,
        0x2 as libc::c_int | 0x1 as libc::c_int,
        0x20 as libc::c_int | 0x1 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    );
    if !shm.is_null() {} else {
        __assert_fail(
            b"shm\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"void at_init(void)\0"))
                .as_ptr(),
        );
    };
    internal_ips_lock = shm as *mut pthread_mutex_t;
    internal_ips = (shm as *mut libc::c_char).offset(2048 as libc::c_int as isize)
        as *mut libc::c_void as *mut internal_ip_lookup_table;
    pthread_mutex_init(internal_ips_lock, 0 as *const pthread_mutexattr_t);
    memset(
        internal_ips as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<internal_ip_lookup_table>() as libc::c_ulong,
    );
    initpipe(req_pipefd.as_mut_ptr());
    initpipe(resp_pipefd.as_mut_ptr());
    let mut allocator_thread_attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&mut allocator_thread_attr);
    pthread_attr_setstacksize(
        &mut allocator_thread_attr,
        (if 16 as libc::c_int * 1024 as libc::c_int > 16384 as libc::c_int {
            16 as libc::c_int * 1024 as libc::c_int
        } else {
            16384 as libc::c_int
        }) as size_t,
    );
    pthread_create(
        &mut allocator_thread,
        &mut allocator_thread_attr,
        Some(threadfunc as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pthread_attr_destroy(&mut allocator_thread_attr);
}
pub unsafe extern "C" fn at_close() {
    let msg: libc::c_int = ATM_EXIT as libc::c_int;
    write(
        req_pipefd[1 as libc::c_int as usize],
        &msg as *const libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    pthread_join(allocator_thread, 0 as *mut *mut libc::c_void);
    close(req_pipefd[0 as libc::c_int as usize]);
    close(req_pipefd[1 as libc::c_int as usize]);
    close(resp_pipefd[0 as libc::c_int as usize]);
    close(resp_pipefd[1 as libc::c_int as usize]);
    pthread_mutex_destroy(internal_ips_lock);
}
unsafe extern "C" fn run_static_initializers() {
    destfd = [
        &*(req_pipefd.as_ptr() as *mut i32).offset(1 as libc::c_int as isize)
            as *const libc::c_int as *mut libc::c_int,
        &*(resp_pipefd.as_ptr() as *mut i32).offset(1 as libc::c_int as isize)
            as *const libc::c_int as *mut libc::c_int,
    ];
    readfd = [
        &*(req_pipefd.as_ptr() as *mut i32).offset(0 as libc::c_int as isize)
            as *const libc::c_int as *mut libc::c_int,
        &*(resp_pipefd.as_ptr() as *mut i32).offset(0 as libc::c_int as isize)
            as *const libc::c_int as *mut libc::c_int,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
