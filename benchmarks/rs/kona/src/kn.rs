use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn abort() -> !;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn cd(a: K) -> K;
    fn show(a: K) -> K;
    static mut offsetDot: V;
    fn at(x: K, y: K) -> K;
    fn CSK(x: K) -> S;
    fn KX(x: K) -> K;
    fn X(s: S) -> K;
    fn maX(a: I, b: I) -> I;
    static mut HTTP_PORT: S;
    fn kerr(s: cS) -> K;
    fn Ki(x: I) -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn denameS(dir_string: S, t: S, create: I) -> *mut K;
    fn vf_ex(q: V, g: K) -> K;
    fn ksender(sockfd: I, y: K, t: I) -> I;
    fn _db(x: K) -> K;
    static mut master: fd_set;
    static mut execute_mutex: pthread_mutex_t;
    static mut errmsg: [C; 256];
    fn bswapI(n: I) -> I;
    fn oerr() -> I;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m1 {
    pub a: libc::c_char,
    pub b: libc::c_char,
    pub c: [libc::c_char; 5],
    pub d: libc::c_char,
    pub n: I,
}
pub type M1 = m1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m0 {
    pub m1: M1,
    pub r: I,
    pub k: K,
    pub a: I,
}
pub type M0 = m0;
pub type TYPE_SEVEN_MEMBERS = libc::c_uint;
pub const TYPE_SEVEN_SIZE: TYPE_SEVEN_MEMBERS = 8;
pub const CACHE_TREE: TYPE_SEVEN_MEMBERS = 7;
pub const CACHE_WD: TYPE_SEVEN_MEMBERS = 6;
pub const CONJ: TYPE_SEVEN_MEMBERS = 5;
pub const PARAMS: TYPE_SEVEN_MEMBERS = 4;
pub const LOCALS: TYPE_SEVEN_MEMBERS = 3;
pub const CODE: TYPE_SEVEN_MEMBERS = 2;
pub const DEPTH: TYPE_SEVEN_MEMBERS = 1;
pub const CONTeXT: TYPE_SEVEN_MEMBERS = 0;
pub static mut CP: [M0; 1025] = [M0 {
    m1: M1 {
        a: 0,
        b: 0,
        c: [0; 5],
        d: 0,
        n: 0,
    },
    r: 0,
    k: 0 as *const k0 as *mut k0,
    a: 0,
}; 1025];
pub static mut KONA_WHO: K = 0 as *const k0 as *mut k0;
pub static mut KONA_PORT: K = 0 as *const k0 as *mut k0;
pub static mut KONA_CLIENT: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn nfinish() {}
pub unsafe extern "C" fn ninit() -> I {
    static mut _done: I = 0 as libc::c_int as I;
    if _done == 0 {
        _done = 1 as libc::c_int as I;
    }
    return _done;
}
pub unsafe extern "C" fn get_in_addr(mut sa: *mut sockaddr) -> *mut libc::c_void {
    if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
        return &mut (*(sa as *mut sockaddr_in)).sin_addr as *mut in_addr
            as *mut libc::c_void;
    }
    return &mut (*(sa as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
        as *mut libc::c_void;
}
static mut _oldw: I = 0;
static mut _oldc: I = 0;
unsafe extern "C" fn mhbegin(mut i: I) {
    _oldw = *(((*KONA_WHO).k).as_mut_ptr() as *mut I);
    _oldc = *(((*KONA_CLIENT).k).as_mut_ptr() as *mut I);
    *(((*KONA_WHO).k).as_mut_ptr() as *mut I) = i;
    *(((*KONA_CLIENT).k).as_mut_ptr() as *mut I) = CP[i as usize].a;
}
unsafe extern "C" fn mhend() {
    *(((*KONA_WHO).k).as_mut_ptr() as *mut I) = _oldw;
    *(((*KONA_CLIENT).k).as_mut_ptr() as *mut I) = _oldc;
}
pub unsafe extern "C" fn wipe_tape(mut i: I) -> I {
    let mut a: I = CP[i as usize].a;
    if !(CP[i as usize].k).is_null() {
        cd(CP[i as usize].k);
    }
    memset(
        &mut *CP.as_mut_ptr().offset(i as isize) as *mut M0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<M0>() as libc::c_ulong,
    );
    CP[i as usize].a = a;
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn close_tape(mut i: I, mut sockfd: I) -> I {
    mhbegin(i);
    wipe_tape(i);
    CP[i as usize].a = 0 as libc::c_int as I;
    let mut r: I = close(sockfd as libc::c_int) as I;
    if r != 0 {
        show(kerr(b"file\0" as *const u8 as *const libc::c_char));
        r = 0 as libc::c_int as I;
    }
    master
        .__fds_bits[(sockfd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)
            as libc::c_longlong) as usize]
        &= !(((1 as libc::c_ulong)
            << sockfd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)
                    as libc::c_longlong) as __fd_mask);
    let mut x: K = *denameS(
        b".\0" as *const u8 as *const libc::c_char as S,
        b".m.c\0" as *const u8 as *const libc::c_char as S,
        0 as libc::c_int as I,
    );
    if 6 as libc::c_int as libc::c_longlong == (*x).t {
        r = 0 as libc::c_int as I;
    } else if 3 as libc::c_int as libc::c_longlong
        != (if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t })
    {
        r = 1 as libc::c_int as I;
        printf(b"type error\0" as *const u8 as *const libc::c_char);
    } else {
        KX(x);
    }
    mhend();
    return r;
}
pub static mut bx: [C; 128] = [
    0 as libc::c_int as C,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub static mut by: [C; 128] = [
    0 as libc::c_int as C,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn modified_execute(mut x: K) -> K {
    if pthread_mutex_lock(&mut execute_mutex) != 0 {
        perror(b"Lock mutex in mod_ex()\0" as *const u8 as *const libc::c_char);
        abort();
    }
    let mut a: K = -(1 as libc::c_int) as K;
    if 4 as libc::c_int as libc::c_longlong == (*x).t
        || 3 as libc::c_int as libc::c_longlong
            == (if (*x).t < 0 as libc::c_int as libc::c_longlong {
                -(*x).t
            } else {
                (*x).t
            })
    {
        a = X(CSK(x));
    }
    if (*x).t == 0 && (*x).n > 0 as libc::c_int as libc::c_longlong {
        a = vf_ex(offsetDot, x);
    }
    if -(1 as libc::c_int) as K != a {
        if pthread_mutex_unlock(&mut execute_mutex) != 0 {
            perror(b"Unlock mutex in mod_ex()\0" as *const u8 as *const libc::c_char);
            abort();
        }
        return a;
    }
    return ci(x);
}
pub unsafe extern "C" fn read_tape(mut i: I, mut j: I, mut type_0: I) -> K {
    let mut u: I = 0;
    let mut a: I = 0;
    let mut c_2: I = 0;
    let mut m: I = 0;
    let mut g: I = 0;
    let mut z: K = 0 as *mut k0;
    let mut b: S = 0 as *mut C;
    let mut current_block: u64;
    let mut nbytes: I = 0 as libc::c_int as I;
    let mut n: I = 0 as libc::c_int as I;
    let mut bz: [C; 128] = [
        0 as libc::c_int as C,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut bn: [C; 1] = [0 as libc::c_int as C];
    if !HTTP_PORT.is_null() {
        if bx[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            nbytes = recv(
                j as libc::c_int,
                bx.as_mut_ptr() as *mut libc::c_void,
                128 as libc::c_int as size_t,
                0 as libc::c_int,
            ) as I;
        } else if by[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            nbytes = recv(
                j as libc::c_int,
                by.as_mut_ptr() as *mut libc::c_void,
                128 as libc::c_int as size_t,
                0 as libc::c_int,
            ) as I;
        } else {
            nbytes = recv(
                j as libc::c_int,
                bz.as_mut_ptr() as *mut libc::c_void,
                128 as libc::c_int as size_t,
                0 as libc::c_int,
            ) as I;
        }
        if nbytes <= 0 as libc::c_int as libc::c_longlong {
            if nbytes == 0 as libc::c_int as libc::c_longlong {
                printf(
                    b"server: socket %lld hung up\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                );
            } else {
                perror(b"recv\0" as *const u8 as *const libc::c_char);
            }
        } else {
            let mut h: K = *denameS(
                b".\0" as *const u8 as *const libc::c_char as S,
                b".m.h\0" as *const u8 as *const libc::c_char as S,
                0 as libc::c_int as I,
            );
            if 6 as libc::c_int as libc::c_longlong == (*h).t {
                send(
                    j as libc::c_int,
                    bx.as_mut_ptr() as *const libc::c_void,
                    nbytes as size_t,
                    0 as libc::c_int,
                );
                bx[0 as libc::c_int as usize] = '\0' as i32 as C;
                close_tape(i, j);
                return 0 as K;
            } else if 7 as libc::c_int as libc::c_longlong != (*h).t
                && 3 as libc::c_int as libc::c_longlong != (*h).n
            {
                let mut n_0: I = snprintf(
                    bx.as_mut_ptr(),
                    128 as libc::c_int as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b".m.h is not type 7-3\0" as *const u8 as *const libc::c_char,
                ) as I;
                if n_0 >= 128 as libc::c_int as libc::c_longlong {
                    return kerr(b"write\0" as *const u8 as *const libc::c_char);
                }
                send(
                    j as libc::c_int,
                    bx.as_mut_ptr() as *const libc::c_void,
                    strlen(bx.as_mut_ptr()),
                    0 as libc::c_int,
                );
                bx[0 as libc::c_int as usize] = '\0' as i32 as C;
                close_tape(i, j);
                return 0 as K;
            } else {
                let mut f: S = ((*(*((*h).k)
                    .as_mut_ptr()
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut C;
                let mut ax: I = 0 as libc::c_int as I;
                let mut ay: I = 0 as libc::c_int as I;
                let mut az: I = 0 as libc::c_int as I;
                let mut sf: I = strlen(f as *const libc::c_char) as I;
                let mut i_0: I = 0 as libc::c_int as I;
                let mut _i: I = sf;
                while i_0 < _i {
                    if *f.offset(i_0 as isize) as libc::c_int == 'x' as i32 {
                        ax = 1 as libc::c_int as I;
                    } else if *f.offset(i_0 as isize) as libc::c_int == 'y' as i32 {
                        ay = 1 as libc::c_int as I;
                    } else if *f.offset(i_0 as isize) as libc::c_int == 'z' as i32 {
                        az = 1 as libc::c_int as I;
                    }
                    i_0 += 1;
                    i_0;
                }
                let mut na: I = maX(1 as libc::c_int as I, ax + ay + az);
                if na == 3 as libc::c_int as libc::c_longlong {
                    if bz[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                        send(
                            j as libc::c_int,
                            bn.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            0 as libc::c_int,
                        );
                        close_tape(i, j);
                        return 0 as K;
                    } else {
                        n = 0 as libc::c_int as I;
                        while n < 128 as libc::c_int as libc::c_longlong {
                            if bx[n as usize] as libc::c_int == '\r' as i32
                                || bx[n as usize] as libc::c_int == '\0' as i32
                            {
                                break;
                            }
                            n += 1;
                            n;
                        }
                        bx[n as usize] = '\0' as i32 as C;
                        n = 0 as libc::c_int as I;
                        while n < 128 as libc::c_int as libc::c_longlong {
                            if by[n as usize] as libc::c_int == '\r' as i32
                                || by[n as usize] as libc::c_int == '\0' as i32
                            {
                                break;
                            }
                            n += 1;
                            n;
                        }
                        by[n as usize] = '\0' as i32 as C;
                        n = 0 as libc::c_int as I;
                        while n < 128 as libc::c_int as libc::c_longlong {
                            if bz[n as usize] as libc::c_int == '\r' as i32 {
                                break;
                            }
                            n += 1;
                            n;
                        }
                        bz[n as usize] = '\0' as i32 as C;
                        let mut sbx: I = strlen(bx.as_mut_ptr()) as I;
                        let mut sby: I = strlen(by.as_mut_ptr()) as I;
                        let mut sbz: I = strlen(bz.as_mut_ptr()) as I;
                        let vla = (13 as libc::c_int as libc::c_longlong + sf + sbx + sby
                            + sbz) as usize;
                        let mut c: Vec::<C> = ::std::vec::from_elem(0, vla);
                        *c
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) = '{' as i32 as C;
                        *c
                            .as_mut_ptr()
                            .offset(
                                (1 as libc::c_int as libc::c_longlong + sf) as isize,
                            ) = '}' as i32 as C;
                        *c
                            .as_mut_ptr()
                            .offset(
                                (2 as libc::c_int as libc::c_longlong + sf) as isize,
                            ) = '[' as i32 as C;
                        *c
                            .as_mut_ptr()
                            .offset(
                                (11 as libc::c_int as libc::c_longlong + sf + sbx + sby
                                    + sbz) as isize,
                            ) = ']' as i32 as C;
                        let ref mut fresh0 = *c
                            .as_mut_ptr()
                            .offset(
                                (10 as libc::c_int as libc::c_longlong + sf + sbx + sby
                                    + sbz) as isize,
                            );
                        *fresh0 = '"' as i32 as C;
                        let ref mut fresh1 = *c
                            .as_mut_ptr()
                            .offset(
                                (9 as libc::c_int as libc::c_longlong + sf + sbx + sby)
                                    as isize,
                            );
                        *fresh1 = *fresh0;
                        let ref mut fresh2 = *c
                            .as_mut_ptr()
                            .offset(
                                (7 as libc::c_int as libc::c_longlong + sf + sbx + sby)
                                    as isize,
                            );
                        *fresh2 = *fresh1;
                        let ref mut fresh3 = *c
                            .as_mut_ptr()
                            .offset(
                                (6 as libc::c_int as libc::c_longlong + sf + sbx) as isize,
                            );
                        *fresh3 = *fresh2;
                        let ref mut fresh4 = *c
                            .as_mut_ptr()
                            .offset(
                                (4 as libc::c_int as libc::c_longlong + sf + sbx) as isize,
                            );
                        *fresh4 = *fresh3;
                        *c
                            .as_mut_ptr()
                            .offset(
                                (3 as libc::c_int as libc::c_longlong + sf) as isize,
                            ) = *fresh4;
                        let ref mut fresh5 = *c
                            .as_mut_ptr()
                            .offset(
                                (8 as libc::c_int as libc::c_longlong + sf + sbx + sby)
                                    as isize,
                            );
                        *fresh5 = ';' as i32 as C;
                        *c
                            .as_mut_ptr()
                            .offset(
                                (5 as libc::c_int as libc::c_longlong + sf + sbx) as isize,
                            ) = *fresh5;
                        *c
                            .as_mut_ptr()
                            .offset(
                                (12 as libc::c_int as libc::c_longlong + sf + sbx + sby
                                    + sbz) as isize,
                            ) = '\0' as i32 as C;
                        let mut i_1: I = 0 as libc::c_int as I;
                        let mut _i_0: I = sf;
                        while i_1 < _i_0 {
                            *c
                                .as_mut_ptr()
                                .offset(
                                    (1 as libc::c_int as libc::c_longlong + i_1) as isize,
                                ) = *f.offset(i_1 as isize);
                            i_1 += 1;
                            i_1;
                        }
                        let mut i_2: I = 0 as libc::c_int as I;
                        let mut _i_1: I = sbx;
                        while i_2 < _i_1 {
                            *c
                                .as_mut_ptr()
                                .offset(
                                    (4 as libc::c_int as libc::c_longlong + sf + i_2) as isize,
                                ) = bx[i_2 as usize];
                            i_2 += 1;
                            i_2;
                        }
                        let mut i_3: I = 0 as libc::c_int as I;
                        let mut _i_2: I = sby;
                        while i_3 < _i_2 {
                            *c
                                .as_mut_ptr()
                                .offset(
                                    (7 as libc::c_int as libc::c_longlong + sf + sbx + i_3)
                                        as isize,
                                ) = by[i_3 as usize];
                            i_3 += 1;
                            i_3;
                        }
                        let mut i_4: I = 0 as libc::c_int as I;
                        let mut _i_3: I = sbz;
                        while i_4 < _i_3 {
                            *c
                                .as_mut_ptr()
                                .offset(
                                    (10 as libc::c_int as libc::c_longlong + sf + sbx + sby
                                        + i_4) as isize,
                                ) = bz[i_4 as usize];
                            i_4 += 1;
                            i_4;
                        }
                        let mut r: K = X(c.as_mut_ptr());
                        let mut w: I = 128 as libc::c_int as I;
                        let vla_0 = w as usize;
                        let mut bck: Vec::<C> = ::std::vec::from_elem(0, vla_0);
                        match (*r).t {
                            1 => {
                                n = snprintf(
                                    bck.as_mut_ptr(),
                                    w as libc::c_ulong,
                                    b"%lld\0" as *const u8 as *const libc::c_char,
                                    *(((*r).k).as_mut_ptr() as *mut I),
                                ) as I;
                                if n >= w {
                                    let ref mut fresh6 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh6 = '.' as i32 as C;
                                    let ref mut fresh7 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh7 = *fresh6;
                                    *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh7;
                                }
                            }
                            2 => {
                                n = snprintf(
                                    bck.as_mut_ptr(),
                                    w as libc::c_ulong,
                                    b"%f\0" as *const u8 as *const libc::c_char,
                                    *(((*r).k).as_mut_ptr() as *mut F),
                                ) as I;
                                if n >= w {
                                    let ref mut fresh8 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh8 = '.' as i32 as C;
                                    let ref mut fresh9 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh9 = *fresh8;
                                    *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh9;
                                }
                            }
                            3 => {
                                n = snprintf(
                                    bck.as_mut_ptr(),
                                    w as libc::c_ulong,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    ((*r).k).as_mut_ptr() as *mut C,
                                ) as I;
                                if n >= w {
                                    let ref mut fresh10 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh10 = '.' as i32 as C;
                                    let ref mut fresh11 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh11 = *fresh10;
                                    *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh11;
                                }
                            }
                            -3 => {
                                n = snprintf(
                                    bck.as_mut_ptr(),
                                    w as libc::c_ulong,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    ((*r).k).as_mut_ptr() as *mut C,
                                ) as I;
                                if n >= w {
                                    let ref mut fresh12 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh12 = '.' as i32 as C;
                                    let ref mut fresh13 = *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh13 = *fresh12;
                                    *bck
                                        .as_mut_ptr()
                                        .offset(
                                            (w - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh13;
                                }
                            }
                            _ => {
                                n = snprintf(
                                    bck.as_mut_ptr(),
                                    w as libc::c_ulong,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"NYI: .m.h result of that type and count\0" as *const u8
                                        as *const libc::c_char,
                                ) as I;
                                if n >= w {
                                    return kerr(b"write\0" as *const u8 as *const libc::c_char);
                                }
                            }
                        }
                        send(
                            j as libc::c_int,
                            bck.as_mut_ptr() as *const libc::c_void,
                            strlen(bck.as_mut_ptr()),
                            0 as libc::c_int,
                        );
                        bx[0 as libc::c_int as usize] = '\0' as i32 as C;
                        by[0 as libc::c_int as usize] = '\0' as i32 as C;
                        close_tape(i, j);
                        return 0 as K;
                    }
                }
                if na == 2 as libc::c_int as libc::c_longlong {
                    if by[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                        send(
                            j as libc::c_int,
                            bn.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            0 as libc::c_int,
                        );
                        close_tape(i, j);
                        return 0 as K;
                    } else {
                        n = 0 as libc::c_int as I;
                        while n < 128 as libc::c_int as libc::c_longlong {
                            if bx[n as usize] as libc::c_int == '\r' as i32 {
                                break;
                            }
                            n += 1;
                            n;
                        }
                        bx[n as usize] = '\0' as i32 as C;
                        n = 0 as libc::c_int as I;
                        while n < 128 as libc::c_int as libc::c_longlong {
                            if by[n as usize] as libc::c_int == '\r' as i32 {
                                break;
                            }
                            n += 1;
                            n;
                        }
                        by[n as usize] = '\0' as i32 as C;
                        let mut sbx_0: I = strlen(bx.as_mut_ptr()) as I;
                        let mut sby_0: I = strlen(by.as_mut_ptr()) as I;
                        let vla_1 = (10 as libc::c_int as libc::c_longlong + sf + sbx_0
                            + sby_0) as usize;
                        let mut c_0: Vec::<C> = ::std::vec::from_elem(0, vla_1);
                        *c_0
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) = '{' as i32 as C;
                        *c_0
                            .as_mut_ptr()
                            .offset(
                                (1 as libc::c_int as libc::c_longlong + sf) as isize,
                            ) = '}' as i32 as C;
                        *c_0
                            .as_mut_ptr()
                            .offset(
                                (2 as libc::c_int as libc::c_longlong + sf) as isize,
                            ) = '[' as i32 as C;
                        *c_0
                            .as_mut_ptr()
                            .offset(
                                (8 as libc::c_int as libc::c_longlong + sf + sbx_0 + sby_0)
                                    as isize,
                            ) = ']' as i32 as C;
                        let ref mut fresh14 = *c_0
                            .as_mut_ptr()
                            .offset(
                                (7 as libc::c_int as libc::c_longlong + sf + sbx_0 + sby_0)
                                    as isize,
                            );
                        *fresh14 = '"' as i32 as C;
                        let ref mut fresh15 = *c_0
                            .as_mut_ptr()
                            .offset(
                                (6 as libc::c_int as libc::c_longlong + sf + sbx_0) as isize,
                            );
                        *fresh15 = *fresh14;
                        let ref mut fresh16 = *c_0
                            .as_mut_ptr()
                            .offset(
                                (4 as libc::c_int as libc::c_longlong + sf + sbx_0) as isize,
                            );
                        *fresh16 = *fresh15;
                        *c_0
                            .as_mut_ptr()
                            .offset(
                                (3 as libc::c_int as libc::c_longlong + sf) as isize,
                            ) = *fresh16;
                        *c_0
                            .as_mut_ptr()
                            .offset(
                                (5 as libc::c_int as libc::c_longlong + sf + sbx_0) as isize,
                            ) = ';' as i32 as C;
                        *c_0
                            .as_mut_ptr()
                            .offset(
                                (9 as libc::c_int as libc::c_longlong + sf + sbx_0 + sby_0)
                                    as isize,
                            ) = '\0' as i32 as C;
                        let mut i_5: I = 0 as libc::c_int as I;
                        let mut _i_4: I = sf;
                        while i_5 < _i_4 {
                            *c_0
                                .as_mut_ptr()
                                .offset(
                                    (1 as libc::c_int as libc::c_longlong + i_5) as isize,
                                ) = *f.offset(i_5 as isize);
                            i_5 += 1;
                            i_5;
                        }
                        let mut i_6: I = 0 as libc::c_int as I;
                        let mut _i_5: I = sbx_0;
                        while i_6 < _i_5 {
                            *c_0
                                .as_mut_ptr()
                                .offset(
                                    (4 as libc::c_int as libc::c_longlong + sf + i_6) as isize,
                                ) = bx[i_6 as usize];
                            i_6 += 1;
                            i_6;
                        }
                        let mut i_7: I = 0 as libc::c_int as I;
                        let mut _i_6: I = sby_0;
                        while i_7 < _i_6 {
                            *c_0
                                .as_mut_ptr()
                                .offset(
                                    (7 as libc::c_int as libc::c_longlong + sf + sbx_0 + i_7)
                                        as isize,
                                ) = by[i_7 as usize];
                            i_7 += 1;
                            i_7;
                        }
                        let mut r_0: K = X(c_0.as_mut_ptr());
                        let mut w_0: I = 128 as libc::c_int as I;
                        let vla_2 = w_0 as usize;
                        let mut bck_0: Vec::<C> = ::std::vec::from_elem(0, vla_2);
                        match (*r_0).t {
                            1 => {
                                n = snprintf(
                                    bck_0.as_mut_ptr(),
                                    w_0 as libc::c_ulong,
                                    b"%lld\0" as *const u8 as *const libc::c_char,
                                    *(((*r_0).k).as_mut_ptr() as *mut I),
                                ) as I;
                                if n >= w_0 {
                                    let ref mut fresh17 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh17 = '.' as i32 as C;
                                    let ref mut fresh18 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh18 = *fresh17;
                                    *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh18;
                                }
                            }
                            2 => {
                                n = snprintf(
                                    bck_0.as_mut_ptr(),
                                    w_0 as libc::c_ulong,
                                    b"%f\0" as *const u8 as *const libc::c_char,
                                    *(((*r_0).k).as_mut_ptr() as *mut F),
                                ) as I;
                                if n >= w_0 {
                                    let ref mut fresh19 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh19 = '.' as i32 as C;
                                    let ref mut fresh20 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh20 = *fresh19;
                                    *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh20;
                                }
                            }
                            3 => {
                                n = snprintf(
                                    bck_0.as_mut_ptr(),
                                    w_0 as libc::c_ulong,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    ((*r_0).k).as_mut_ptr() as *mut C,
                                ) as I;
                                if n >= w_0 {
                                    let ref mut fresh21 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh21 = '.' as i32 as C;
                                    let ref mut fresh22 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh22 = *fresh21;
                                    *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh22;
                                }
                            }
                            -3 => {
                                n = snprintf(
                                    bck_0.as_mut_ptr(),
                                    w_0 as libc::c_ulong,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    ((*r_0).k).as_mut_ptr() as *mut C,
                                ) as I;
                                if n >= w_0 {
                                    let ref mut fresh23 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 2 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh23 = '.' as i32 as C;
                                    let ref mut fresh24 = *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 3 as libc::c_int as libc::c_longlong) as isize,
                                        );
                                    *fresh24 = *fresh23;
                                    *bck_0
                                        .as_mut_ptr()
                                        .offset(
                                            (w_0 - 4 as libc::c_int as libc::c_longlong) as isize,
                                        ) = *fresh24;
                                }
                            }
                            _ => {
                                n = snprintf(
                                    bck_0.as_mut_ptr(),
                                    w_0 as libc::c_ulong,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    b"NYI: .m.h result of that type and count\0" as *const u8
                                        as *const libc::c_char,
                                ) as I;
                                if n >= w_0 {
                                    return kerr(b"write\0" as *const u8 as *const libc::c_char);
                                }
                            }
                        }
                        send(
                            j as libc::c_int,
                            bck_0.as_mut_ptr() as *const libc::c_void,
                            strlen(bck_0.as_mut_ptr()),
                            0 as libc::c_int,
                        );
                        bx[0 as libc::c_int as usize] = '\0' as i32 as C;
                        by[0 as libc::c_int as usize] = '\0' as i32 as C;
                        close_tape(i, j);
                        return 0 as K;
                    }
                }
                n = 0 as libc::c_int as I;
                while n < 128 as libc::c_int as libc::c_longlong {
                    if bx[n as usize] as libc::c_int == '\r' as i32
                        || bx[n as usize] as libc::c_int == '\0' as i32
                    {
                        break;
                    }
                    n += 1;
                    n;
                }
                bx[n as usize] = '\0' as i32 as C;
                let mut sbx_1: I = strlen(bx.as_mut_ptr()) as I;
                let vla_3 = (7 as libc::c_int as libc::c_longlong + sf + sbx_1) as usize;
                let mut c_1: Vec::<C> = ::std::vec::from_elem(0, vla_3);
                *c_1.as_mut_ptr().offset(0 as libc::c_int as isize) = '{' as i32 as C;
                *c_1
                    .as_mut_ptr()
                    .offset(
                        (1 as libc::c_int as libc::c_longlong + sf) as isize,
                    ) = '}' as i32 as C;
                *c_1
                    .as_mut_ptr()
                    .offset(
                        (2 as libc::c_int as libc::c_longlong + sf) as isize,
                    ) = '[' as i32 as C;
                *c_1
                    .as_mut_ptr()
                    .offset(
                        (5 as libc::c_int as libc::c_longlong + sf + sbx_1) as isize,
                    ) = ']' as i32 as C;
                let ref mut fresh25 = *c_1
                    .as_mut_ptr()
                    .offset(
                        (4 as libc::c_int as libc::c_longlong + sf + sbx_1) as isize,
                    );
                *fresh25 = '"' as i32 as C;
                *c_1
                    .as_mut_ptr()
                    .offset(
                        (3 as libc::c_int as libc::c_longlong + sf) as isize,
                    ) = *fresh25;
                *c_1
                    .as_mut_ptr()
                    .offset(
                        (6 as libc::c_int as libc::c_longlong + sf + sbx_1) as isize,
                    ) = '\0' as i32 as C;
                let mut i_8: I = 0 as libc::c_int as I;
                let mut _i_7: I = sf;
                while i_8 < _i_7 {
                    *c_1
                        .as_mut_ptr()
                        .offset(
                            (1 as libc::c_int as libc::c_longlong + i_8) as isize,
                        ) = *f.offset(i_8 as isize);
                    i_8 += 1;
                    i_8;
                }
                let mut i_9: I = 0 as libc::c_int as I;
                let mut _i_8: I = sbx_1;
                while i_9 < _i_8 {
                    *c_1
                        .as_mut_ptr()
                        .offset(
                            (4 as libc::c_int as libc::c_longlong + sf + i_9) as isize,
                        ) = bx[i_9 as usize];
                    i_9 += 1;
                    i_9;
                }
                let mut r_1: K = X(c_1.as_mut_ptr());
                if strcmp(
                    errmsg.as_mut_ptr(),
                    b"(nil)\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    oerr();
                } else {
                    let mut w_1: I = 128 as libc::c_int as I;
                    let vla_4 = w_1 as usize;
                    let mut bck_1: Vec::<C> = ::std::vec::from_elem(0, vla_4);
                    match (*r_1).t {
                        1 => {
                            n = snprintf(
                                bck_1.as_mut_ptr(),
                                w_1 as libc::c_ulong,
                                b"%lld\0" as *const u8 as *const libc::c_char,
                                *(((*r_1).k).as_mut_ptr() as *mut I),
                            ) as I;
                            if n >= w_1 {
                                let ref mut fresh26 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 2 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh26 = '.' as i32 as C;
                                let ref mut fresh27 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 3 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh27 = *fresh26;
                                *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 4 as libc::c_int as libc::c_longlong) as isize,
                                    ) = *fresh27;
                            }
                        }
                        2 => {
                            n = snprintf(
                                bck_1.as_mut_ptr(),
                                w_1 as libc::c_ulong,
                                b"%f\0" as *const u8 as *const libc::c_char,
                                *(((*r_1).k).as_mut_ptr() as *mut F),
                            ) as I;
                            if n >= w_1 {
                                let ref mut fresh28 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 2 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh28 = '.' as i32 as C;
                                let ref mut fresh29 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 3 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh29 = *fresh28;
                                *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 4 as libc::c_int as libc::c_longlong) as isize,
                                    ) = *fresh29;
                            }
                        }
                        3 => {
                            n = snprintf(
                                bck_1.as_mut_ptr(),
                                w_1 as libc::c_ulong,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                ((*r_1).k).as_mut_ptr() as *mut C,
                            ) as I;
                            if n >= w_1 {
                                let ref mut fresh30 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 2 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh30 = '.' as i32 as C;
                                let ref mut fresh31 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 3 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh31 = *fresh30;
                                *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 4 as libc::c_int as libc::c_longlong) as isize,
                                    ) = *fresh31;
                            }
                        }
                        -3 => {
                            n = snprintf(
                                bck_1.as_mut_ptr(),
                                w_1 as libc::c_ulong,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                ((*r_1).k).as_mut_ptr() as *mut C,
                            ) as I;
                            if n >= w_1 {
                                let ref mut fresh32 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 2 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh32 = '.' as i32 as C;
                                let ref mut fresh33 = *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 3 as libc::c_int as libc::c_longlong) as isize,
                                    );
                                *fresh33 = *fresh32;
                                *bck_1
                                    .as_mut_ptr()
                                    .offset(
                                        (w_1 - 4 as libc::c_int as libc::c_longlong) as isize,
                                    ) = *fresh33;
                            }
                        }
                        _ => {
                            n = snprintf(
                                bck_1.as_mut_ptr(),
                                w_1 as libc::c_ulong,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                b"NYI: .m.h result of that type and count\0" as *const u8
                                    as *const libc::c_char,
                            ) as I;
                            if n >= w_1 {
                                return kerr(b"write\0" as *const u8 as *const libc::c_char);
                            }
                        }
                    }
                    send(
                        j as libc::c_int,
                        bck_1.as_mut_ptr() as *const libc::c_void,
                        strlen(bck_1.as_mut_ptr()),
                        0 as libc::c_int,
                    );
                    bx[0 as libc::c_int as usize] = '\0' as i32 as C;
                    by[0 as libc::c_int as usize] = '\0' as i32 as C;
                    close_tape(i, j);
                    return 0 as K;
                }
            }
        }
    } else {
        u = 1 as libc::c_int as I;
        a = *(&mut u as *mut I as S) as I;
        c_2 = CP[i as usize].r;
        m = ::std::mem::size_of::<M1>() as libc::c_ulong as I;
        g = 0;
        z = 0 as K;
        b = if c_2 < m {
            (&mut (*CP.as_mut_ptr().offset(i as isize)).m1 as *mut M1 as S)
                .offset(c_2 as isize)
        } else {
            (((*CP[i as usize].k).k).as_mut_ptr() as *mut C).offset(c_2 as isize)
        };
        g = if c_2 < m { m - c_2 } else { CP[i as usize].m1.n };
        nbytes = recv(
            j as libc::c_int,
            b as *mut libc::c_void,
            g as size_t,
            0 as libc::c_int,
        ) as I;
        if nbytes <= 0 as libc::c_int as libc::c_longlong {
            if !(nbytes == 0 as libc::c_int as libc::c_longlong) {
                perror(b"recv\0" as *const u8 as *const libc::c_char);
            }
        } else {
            CP[i as usize].r += nbytes;
            if m == CP[i as usize].r {
                if a != CP[i as usize].m1.a as libc::c_longlong {
                    CP[i as usize].m1.n = bswapI(CP[i as usize].m1.n);
                }
                let mut k: K = newK(-(3 as libc::c_int) as I, m + CP[i as usize].m1.n);
                CP[i as usize].k = k;
                if (CP[i as usize].k).is_null() {
                    current_block = 11195527334401045551;
                } else {
                    memcpy(
                        ((*k).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                        &mut (*CP.as_mut_ptr().offset(i as isize)).m1 as *mut M1
                            as *const libc::c_void,
                        m as libc::c_ulong,
                    );
                    current_block = 4290490916767178696;
                }
            } else {
                current_block = 4290490916767178696;
            }
            match current_block {
                11195527334401045551 => {}
                _ => {
                    if CP[i as usize].r == m + CP[i as usize].m1.n {
                        let mut p: *mut M1 = ((*CP[i as usize].k).k).as_mut_ptr()
                            as *mut C as V as *mut M1;
                        let mut msg_type: I = (*p).d as I;
                        let mut h_0: K = _db(CP[i as usize].k);
                        if h_0.is_null() {
                            current_block = 11195527334401045551;
                        } else {
                            wipe_tape(i);
                            if 2 as libc::c_int as libc::c_longlong == msg_type
                                && 1 as libc::c_int as libc::c_longlong == type_0
                            {
                                if (*h_0).t != 0
                                    || 2 as libc::c_int as libc::c_longlong != (*h_0).n
                                {
                                    return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
                                }
                                let mut s: K = *((*h_0).k)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize);
                                let mut r_2: K = *((*h_0).k)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize);
                                if 1 as libc::c_int as libc::c_longlong != (*s).t {
                                    return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
                                }
                                if *(((*s).k).as_mut_ptr() as *mut I) != 0 {
                                    if 3 as libc::c_int as libc::c_longlong
                                        != (if (*r_2).t < 0 as libc::c_int as libc::c_longlong {
                                            -(*r_2).t
                                        } else {
                                            (*r_2).t
                                        })
                                    {
                                        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
                                    }
                                    r_2 = kerr(((*r_2).k).as_mut_ptr() as *mut C as cS);
                                } else {
                                    ci(r_2);
                                }
                                cd(h_0);
                                return r_2;
                            }
                            let mut m_0: K = if 2 as libc::c_int as libc::c_longlong
                                > msg_type
                            {
                                *denameS(
                                    b".\0" as *const u8 as *const libc::c_char as S,
                                    (if msg_type != 0 {
                                        b".m.g\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b".m.s\0" as *const u8 as *const libc::c_char
                                    }) as S,
                                    0 as libc::c_int as I,
                                )
                            } else {
                                0 as K
                            };
                            if m_0.is_null()
                                || 6 as libc::c_int as libc::c_longlong == (*m_0).t
                            {
                                z = modified_execute(h_0);
                            } else {
                                mhbegin(i);
                                z = at(m_0, h_0);
                                mhend();
                            }
                            if msg_type != 0 {
                                let mut u_0: K = newK(
                                    0 as libc::c_int as I,
                                    2 as libc::c_int as I,
                                );
                                let mut s_0: K = Ki(0 as libc::c_int as I);
                                if OOM_CD(
                                    0 as libc::c_int as I,
                                    u_0,
                                    s_0,
                                    -(1 as libc::c_int) as V,
                                ) == 0
                                {
                                    return 0 as K;
                                }
                                if z.is_null() {
                                    *(((*s_0).k).as_mut_ptr()
                                        as *mut I) = 1 as libc::c_int as I;
                                    z = newK(
                                        -(3 as libc::c_int) as I,
                                        strlen(errmsg.as_mut_ptr()) as I,
                                    );
                                    if OOM_CD(
                                        0 as libc::c_int as I,
                                        u_0,
                                        z,
                                        -(1 as libc::c_int) as V,
                                    ) == 0
                                    {
                                        return 0 as K;
                                    }
                                    strcpy(
                                        ((*z).k).as_mut_ptr() as *mut C,
                                        errmsg.as_mut_ptr(),
                                    );
                                    kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
                                }
                                let ref mut fresh34 = *((*u_0).k)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize);
                                *fresh34 = s_0;
                                let ref mut fresh35 = *((*u_0).k)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize);
                                *fresh35 = z;
                                z = u_0;
                            } else if z.is_null() {
                                printf(
                                    b"%s error\n\0" as *const u8 as *const libc::c_char,
                                    errmsg.as_mut_ptr(),
                                );
                                kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
                            }
                            cd(h_0);
                            if !z.is_null() {
                                if 1 as libc::c_int as libc::c_longlong == msg_type
                                    && 0 as libc::c_int as libc::c_longlong == type_0
                                {
                                    ksender(j, z, 2 as libc::c_int as I);
                                }
                            }
                            cd(z);
                            z = 0 as K;
                            current_block = 18261087299847419448;
                        }
                    } else {
                        current_block = 18261087299847419448;
                    }
                    match current_block {
                        11195527334401045551 => {}
                        _ => return z,
                    }
                }
            }
        }
    }
    close_tape(i, j);
    return -(1 as libc::c_int) as K;
}
