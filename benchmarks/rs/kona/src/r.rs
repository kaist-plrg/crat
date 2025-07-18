use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    static mut kreci: I;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    static mut d_: S;
    fn Ks(x: S) -> K;
    fn Kf(x: F) -> K;
    fn Ki(x: I) -> K;
    fn kap(a: *mut K, v: V) -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn cd(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn mstat() -> K;
    fn alloc(sz: size_t) -> V;
    fn genrand64_real2() -> libc::c_double;
    fn KC(a: K, b: K) -> I;
    fn FC(a: F, b: F) -> I;
    static mut NIL: K;
    fn demote(a: K) -> K;
    fn promote(a: K) -> K;
    fn take_reshape(a: K, b: K) -> K;
    fn kerr(s: cS) -> K;
    fn sp(k: S) -> S;
    fn CSK(x: K) -> S;
    fn rrep(v: V, aft: V, b: *mut I, y: I, s: I) -> K;
    fn floor_ceil(a: K, g: Option::<unsafe extern "C" fn(F) -> F>) -> K;
    fn wrep(x: K, v: V, y: I) -> I;
    fn rep(x: K, y: I) -> I;
    fn X(s: S) -> K;
    fn vf_ex(q: V, g: K) -> K;
    static mut KFIXED: K;
    static mut KONA_ARGS: K;
    static mut KONA_WHO: K;
    static mut KONA_PORT: K;
    static mut KONA_GSET: K;
    static mut KONA_IDX: K;
    static mut KONA_CLIENT: K;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type socklen_t = __socklen_t;
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
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
pub type UC = libc::c_uchar;
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
pub static mut gtime_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _gtime(mut x: K) -> K {
    let mut a: I = kreci;
    if gtime_KVAR.is_null() {
        gtime_KVAR = X(
            b"{(_dj _ x % 86400; 100 _sv 24 60 60 _vsx x ! 86400)}\0" as *const u8
                as *const libc::c_char as S,
        );
        if gtime_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut gtime_KVAR as *mut K as V);
        cd(gtime_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 1 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh0 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh0 = x;
    let mut z: K = vf_ex(&mut gtime_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh1 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh1 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub unsafe extern "C" fn _inv(mut x: K) -> K {
    let mut a: I = kreci;
    if inv_KVAR.is_null() {
        inv_KVAR = X(
            b"{((2##*x)#1,&#*x)_lsq x}\0" as *const u8 as *const libc::c_char as S,
        );
        if inv_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut inv_KVAR as *mut K as V);
        cd(inv_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 1 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh2 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh2 = x;
    let mut z: K = vf_ex(&mut inv_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh3 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh3 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut inv_KVAR: K = 0 as *const k0 as *mut k0;
pub static mut binl_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _binl(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if binl_KVAR.is_null() {
        binl_KVAR = X(b"{x _bin/: y}\0" as *const u8 as *const libc::c_char as S);
        if binl_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut binl_KVAR as *mut K as V);
        cd(binl_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh4 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh4 = x;
    let ref mut fresh5 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh5 = y;
    let mut z: K = vf_ex(&mut binl_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh6 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh6 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut dvl_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _dvl(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if dvl_KVAR.is_null() {
        dvl_KVAR = X(b"{x@&(#y)=y?/:x}\0" as *const u8 as *const libc::c_char as S);
        if dvl_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut dvl_KVAR as *mut K as V);
        cd(dvl_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh7 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh7 = x;
    let ref mut fresh8 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh8 = y;
    let mut z: K = vf_ex(&mut dvl_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh9 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh9 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut di_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _di(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if di_KVAR.is_null() {
        di_KVAR = X(
            b"{r::[@x;_n;(#x)#1];:[@x;. _f[. x;(!x)?/:y];x@&@[r;y;:;0]]}\0" as *const u8
                as *const libc::c_char as S,
        );
        if di_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut di_KVAR as *mut K as V);
        cd(di_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh10 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh10 = x;
    let ref mut fresh11 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh11 = y;
    let mut z: K = vf_ex(&mut di_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh12 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh12 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub unsafe extern "C" fn _dv(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if dv_KVAR.is_null() {
        dv_KVAR = X(b"{x _dvl ,y}\0" as *const u8 as *const libc::c_char as S);
        if dv_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut dv_KVAR as *mut K as V);
        cd(dv_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh13 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh13 = x;
    let ref mut fresh14 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh14 = y;
    let mut z: K = vf_ex(&mut dv_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh15 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh15 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut dv_KVAR: K = 0 as *const k0 as *mut k0;
pub static mut in_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _in(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if in_KVAR.is_null() {
        in_KVAR = X(
            b"{:[@y;x~y;:[~-2=4:y;1;~x~0n;1;0n=+/y];(#y)>y?x;0]}\0" as *const u8
                as *const libc::c_char as S,
        );
        if in_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut in_KVAR as *mut K as V);
        cd(in_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh16 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh16 = x;
    let ref mut fresh17 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh17 = y;
    let mut z: K = vf_ex(&mut in_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh18 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh18 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut lin_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _lin(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if lin_KVAR.is_null() {
        lin_KVAR = X(
            b"{{:[@y;x~y;:[~-2=4:y;1;~x~0n;1;0n=+/y];(#y)>y?x;0]}[;y]'x}\0" as *const u8
                as *const libc::c_char as S,
        );
        if lin_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut lin_KVAR as *mut K as V);
        cd(lin_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh19 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh19 = x;
    let ref mut fresh20 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh20 = y;
    let mut z: K = vf_ex(&mut lin_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh21 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh21 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut mul_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _mul(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if mul_KVAR.is_null() {
        mul_KVAR = X(b"{x _dot\\:y}\0" as *const u8 as *const libc::c_char as S);
        if mul_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut mul_KVAR as *mut K as V);
        cd(mul_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh22 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh22 = x;
    let ref mut fresh23 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh23 = y;
    let mut z: K = vf_ex(&mut mul_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh24 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh24 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut sv_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _sv(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if sv_KVAR.is_null() {
        sv_KVAR = X(b"{{z+y*x}/[0;x;y]}\0" as *const u8 as *const libc::c_char as S);
        if sv_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut sv_KVAR as *mut K as V);
        cd(sv_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh25 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh25 = x;
    let ref mut fresh26 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh26 = y;
    let mut z: K = vf_ex(&mut sv_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh27 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh27 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut hat_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _hat(mut x: K, mut y: K) -> K {
    let mut a: I = kreci;
    if hat_KVAR.is_null() {
        hat_KVAR = X(
            b"{:[(1~4:x)|(2~4:x); _f[!x;y];:[@y;_f[x;,y]; x _dvl y]]}\0" as *const u8
                as *const libc::c_char as S,
        );
        if hat_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut hat_KVAR as *mut K as V);
        cd(hat_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh28 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh28 = x;
    let ref mut fresh29 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh29 = y;
    let mut z: K = vf_ex(&mut hat_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh30 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh30 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub static mut ssr_KVAR: K = 0 as *const k0 as *mut k0;
pub unsafe extern "C" fn _ssr(mut x: K, mut y: K, mut w: K) -> K {
    let mut a: I = kreci;
    if ssr_KVAR.is_null() {
        ssr_KVAR = X(
            b"{if[_n~x;:_n];i:1+2*!_.5*#x:(0,/(0,+/~+\\(>':0,\"[\"=y)-<':(\"]\"=y$:),0)+/:x _ss y)_ x;,/ :[7=4:z;@[x;i;z];4:z$:;@[x;i;:[;z]];@[x;i;:;z]]}\0"
                as *const u8 as *const libc::c_char as S,
        );
        if ssr_KVAR.is_null() {
            return 0 as K;
        }
        kap(&mut KFIXED, &mut ssr_KVAR as *mut K as V);
        cd(ssr_KVAR);
    }
    let mut k: K = newK(0 as libc::c_int as I, 3 as libc::c_int as I);
    if k.is_null() {
        return 0 as K;
    }
    let ref mut fresh31 = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh31 = x;
    let ref mut fresh32 = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh32 = y;
    let ref mut fresh33 = *((*k).k).as_mut_ptr().offset(2 as libc::c_int as isize);
    *fresh33 = w;
    let mut z: K = vf_ex(&mut ssr_KVAR as *mut K as V, k);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*k).n;
    while i < _i_0 {
        let ref mut fresh34 = *((*k).k).as_mut_ptr().offset(i as isize);
        *fresh34 = 0 as *mut k0;
        i += 1;
        i;
    }
    cd(k);
    kreci = a + 1 as libc::c_int as libc::c_longlong;
    return z;
}
pub unsafe extern "C" fn kdef(mut n: I) -> K {
    match n {
        98 => {
            return X(
                b"{(_dj _ x % 86400; 100 _sv 24 60 60 _vsx x ! 86400)}\0" as *const u8
                    as *const libc::c_char as S,
            );
        }
        101 => {
            return X(
                b"{((2##*x)#1,&#*x)_lsq x}\0" as *const u8 as *const libc::c_char as S,
            );
        }
        107 => return X(b"{x _bin/: y}\0" as *const u8 as *const libc::c_char as S),
        108 => {
            return X(
                b"{r::[@x;_n;(#x)#1];:[@x;. _f[. x;(!x)?/:y];x@&@[r;y;:;0]]}\0"
                    as *const u8 as *const libc::c_char as S,
            );
        }
        111 => return X(b"{x _dvl ,y}\0" as *const u8 as *const libc::c_char as S),
        112 => return X(b"{x@&(#y)=y?/:x}\0" as *const u8 as *const libc::c_char as S),
        113 => {
            return X(
                b"{:[(1~4:x)|(2~4:x); _f[!x;y];:[@y;_f[x;,y]; x _dvl y]]}\0" as *const u8
                    as *const libc::c_char as S,
            );
        }
        114 => {
            return X(
                b"{:[@y;x~y;:[~-2=4:y;1;~x~0n;1;0n=+/y];(#y)>y?x;0]}\0" as *const u8
                    as *const libc::c_char as S,
            );
        }
        115 => {
            return X(
                b"{{:[@y;x~y;:[~-2=4:y;1;~x~0n;1;0n=+/y];(#y)>y?x;0]}[;y]'x}\0"
                    as *const u8 as *const libc::c_char as S,
            );
        }
        117 => return X(b"{x _dot\\:y}\0" as *const u8 as *const libc::c_char as S),
        121 => return X(b"{{z+y*x}/[0;x;y]}\0" as *const u8 as *const libc::c_char as S),
        123 => {
            return X(
                b"{if[_n~x;:_n];i:1+2*!_.5*#x:(0,/(0,+/~+\\(>':0,\"[\"=y)-<':(\"]\"=y$:),0)+/:x _ss y)_ x;,/ :[7=4:z;@[x;i;z];4:z$:;@[x;i;:[;z]];@[x;i;:;z]]}\0"
                    as *const u8 as *const libc::c_char as S,
            );
        }
        _ => {}
    }
    return 0 as K;
}
pub unsafe extern "C" fn sqr(mut x: F) -> F {
    return pow(x, 2 as libc::c_int as libc::c_double);
}
pub unsafe extern "C" fn math(
    mut f: Option::<unsafe extern "C" fn(F) -> F>,
    mut a: K,
) -> K {
    let mut at: I = (*a).t;
    let mut n: I = (*a).n;
    if (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        > 2 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut t: I = if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        2 as libc::c_int as libc::c_longlong * at
    } else {
        at
    };
    let mut e: K = 0 as *mut k0;
    let mut z: K = newK(t, n);
    if 0 as libc::c_int as libc::c_longlong == at {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            e = *((*a).k).as_mut_ptr().offset(i as isize);
            let ref mut fresh35 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh35 = math(f, e);
            if (*((*z).k).as_mut_ptr().offset(i as isize)).is_null() {
                cd(z);
                return 0 as K;
            }
            i += 1;
            i;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_0 as isize,
                ) = f
                .unwrap()(*(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize) as F);
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_1 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = f.unwrap()(*(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize));
            i_1 += 1;
            i_1;
        }
    }
    return z;
}
pub unsafe extern "C" fn _kona_exit(mut a: K) -> K {
    if 1 as libc::c_int as libc::c_longlong
        != (if (*a).t < 0 as libc::c_int as libc::c_longlong { -(*a).t } else { (*a).t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    exit(*(((*a).k).as_mut_ptr() as *mut I) as libc::c_int);
}
pub unsafe extern "C" fn _tanh(mut a: K) -> K {
    return math(Some(tanh as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _atan(mut a: K) -> K {
    return math(Some(atan as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _ceil(mut a: K) -> K {
    return math(Some(ceil as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _sin(mut a: K) -> K {
    return math(Some(sin as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _asin(mut a: K) -> K {
    return math(Some(asin as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _acos(mut a: K) -> K {
    return math(Some(acos as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _exp(mut a: K) -> K {
    return math(Some(exp as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _sqrt(mut a: K) -> K {
    return math(Some(sqrt as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _log(mut a: K) -> K {
    return math(Some(log as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _cos(mut a: K) -> K {
    return math(Some(cos as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _floor(mut a: K) -> K {
    return math(
        Some(floor as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
        a,
    );
}
pub unsafe extern "C" fn _tan(mut a: K) -> K {
    return math(Some(tan as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _cosh(mut a: K) -> K {
    return math(Some(cosh as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub unsafe extern "C" fn _sqr(mut a: K) -> K {
    return math(Some(sqr as unsafe extern "C" fn(F) -> F), a);
}
pub unsafe extern "C" fn _sinh(mut a: K) -> K {
    return math(Some(sinh as unsafe extern "C" fn(libc::c_double) -> libc::c_double), a);
}
pub static mut n_s: S = b"Tacdfhikmnpstuvw\0" as *const u8 as *const libc::c_char as S;
pub static mut vn_: [V; 17] = unsafe {
    [
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_T),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_a),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_c),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_d),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_f),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_h),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_i),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_k),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_m),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_n),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_p),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_s),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_t),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_u),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_v),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> K>,
            V,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> K,
                    unsafe extern "C" fn() -> K,
                >(_w),
            ),
        ),
        0 as *const libc::c_void as V,
    ]
};
pub unsafe extern "C" fn _abs(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        > 2 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(t, n);
    if 0 as libc::c_int as libc::c_longlong == t {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            let ref mut fresh36 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh36 = _abs(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize)
                < 0 as libc::c_int as libc::c_longlong
            {
                -*(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize)
            } else {
                *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize)
            };
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_1 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                < 0 as libc::c_int as libc::c_double
            {
                -*(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
            } else {
                *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
            };
            i_1 += 1;
            i_1;
        }
    }
    return z;
}
pub unsafe extern "C" fn net(mut x: K) -> I {
    return (::std::mem::size_of::<M1>() as libc::c_ulong as libc::c_ulonglong)
        .wrapping_add(rep(x, 0 as libc::c_int as I) as libc::c_ulonglong) as I;
}
pub unsafe extern "C" fn _bd(mut x: K) -> K {
    let mut s: I = net(x);
    let mut z: K = newK(-(3 as libc::c_int) as I, s);
    if z.is_null() {
        return 0 as K;
    }
    let mut m: *mut M1 = ((*z).k).as_mut_ptr() as V as *mut M1;
    let mut u: I = 1 as libc::c_int as I;
    (*m).a = *(&mut u as *mut I as S);
    (*m)
        .n = (s as libc::c_ulonglong)
        .wrapping_sub(::std::mem::size_of::<M1>() as libc::c_ulong as libc::c_ulonglong)
        as I;
    wrep(
        x,
        (m as V).offset(::std::mem::size_of::<M1>() as libc::c_ulong as isize),
        0 as libc::c_int as I,
    );
    return z;
}
pub unsafe extern "C" fn _ceiling(mut a: K) -> K {
    return floor_ceil(
        a,
        Some(ceil as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
}
pub unsafe extern "C" fn _ci(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        > 1 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(t * 3 as libc::c_int as libc::c_longlong, n);
    if t == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            let ref mut fresh37 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh37 = _ci(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_0 as isize,
                ) = (*(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize)
                % 256 as libc::c_long as libc::c_longlong) as UC as C;
            i_0 += 1;
            i_0;
        }
    }
    return z;
}
pub unsafe extern "C" fn _db(mut x: K) -> K {
    if -(3 as libc::c_int) as libc::c_longlong != (*x).t {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if ((*x).n as libc::c_ulonglong)
        < ::std::mem::size_of::<M1>() as libc::c_ulong as libc::c_ulonglong
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut m: *mut M1 = ((*x).k).as_mut_ptr() as *mut C as V as *mut M1;
    if ((*m).n as libc::c_ulonglong)
        .wrapping_add(::std::mem::size_of::<M1>() as libc::c_ulong as libc::c_ulonglong)
        != (*x).n as libc::c_ulonglong
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut p: V = (m as V)
        .offset(::std::mem::size_of::<M1>() as libc::c_ulong as isize);
    let mut b: I = 0 as libc::c_int as I;
    let mut u: I = 1 as libc::c_int as I;
    let mut a: C = *(&mut u as *mut I as S);
    return rrep(
        p,
        p.offset((*m).n as isize),
        &mut b,
        0 as libc::c_int as I,
        ((*m).a as libc::c_int != a as libc::c_int) as libc::c_int as I,
    );
}
pub unsafe extern "C" fn _dj(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        > 1 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(t, n);
    if t == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            let ref mut fresh38 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh38 = _dj(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = date_from_jdn(
                *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize),
            );
            i_0 += 1;
            i_0;
        }
    }
    return z;
}
pub unsafe extern "C" fn _getenv(mut a: K) -> K {
    let mut u: S = getenv(CSK(a) as *const libc::c_char);
    let mut z: K = 0 as *mut k0;
    let mut c: I = 0;
    if !u.is_null() {
        c = strlen(u as *const libc::c_char) as I;
        z = newK(-(3 as libc::c_int) as I, c);
        if z.is_null() {
            return 0 as K;
        }
        memcpy(
            ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
            u as *const libc::c_void,
            c as libc::c_ulong,
        );
    } else {
        z = _n();
    }
    return z;
}
pub unsafe extern "C" fn _host(mut a: K) -> K {
    let mut t: I = (*a).t;
    if 4 as libc::c_int as libc::c_longlong == t {
        let mut b: addrinfo = addrinfo {
            ai_flags: 0,
            ai_family: 0,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: 0 as *mut sockaddr,
            ai_canonname: 0 as *mut libc::c_char,
            ai_next: 0 as *mut addrinfo,
        };
        let mut c: *mut addrinfo = 0 as *mut addrinfo;
        memset(
            &mut b as *mut addrinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        b.ai_family = 2 as libc::c_int;
        b.ai_socktype = SOCK_STREAM as libc::c_int;
        if getaddrinfo(
            *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char,
            0 as *const libc::c_char,
            &mut b,
            &mut c,
        ) == 0
        {
            let mut q: I = ntohl((*((*c).ai_addr as *mut sockaddr_in)).sin_addr.s_addr)
                as I;
            freeaddrinfo(c);
            *__errno_location() = 0 as libc::c_int;
            return Ki(q);
        }
    } else if 1 as libc::c_int as libc::c_longlong == t {
        let mut s: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        memset(
            &mut s as *mut sockaddr_in as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        let mut host: [C; 1024] = [0; 1024];
        s.sin_family = 2 as libc::c_int as sa_family_t;
        s.sin_addr.s_addr = htonl(*(((*a).k).as_mut_ptr() as *mut I) as uint32_t);
        if getnameinfo(
            &mut s as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
            host.as_mut_ptr(),
            ::std::mem::size_of::<[C; 1024]>() as libc::c_ulong as socklen_t,
            0 as *mut libc::c_char,
            0 as libc::c_int as socklen_t,
            0 as libc::c_int,
        ) == 0
        {
            *__errno_location() = 0 as libc::c_int;
            return Ks(sp(host.as_mut_ptr()));
        }
    } else {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    *__errno_location() = 0 as libc::c_int;
    return kerr(b"value\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn _ic(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if t != 0
        && 3 as libc::c_int as libc::c_longlong
            != (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(t / 3 as libc::c_int as libc::c_longlong, n);
    if t == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            let ref mut fresh39 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh39 = _ic(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = *(((*a).k).as_mut_ptr() as *mut C).offset(i_0 as isize) as UC as I;
            i_0 += 1;
            i_0;
        }
    }
    return z;
}
pub unsafe extern "C" fn _jd(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    let mut x: I = 0;
    if (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        > 1 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(t, n);
    if t == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            let ref mut fresh40 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh40 = _jd(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            x = *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = jdn_from_date(
                x / 10000 as libc::c_int as libc::c_longlong,
                x / 100 as libc::c_int as libc::c_longlong
                    % 100 as libc::c_int as libc::c_longlong,
                x % 100 as libc::c_int as libc::c_longlong,
            );
            i_0 += 1;
            i_0;
        }
    }
    return z;
}
pub unsafe extern "C" fn _lt(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if (1 as libc::c_int as libc::c_longlong)
        < (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let b: time_t = 0 as libc::c_int as time_t;
    let mut c: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    localtime_r(&b, &mut c);
    let mut d: I = c.tm_gmtoff as I;
    let mut z: K = newK(t, n);
    if t == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i < _i_0 {
            let ref mut fresh41 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh41 = _lt(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize) + d;
            i_0 += 1;
            i_0;
        }
    }
    return z;
}
pub unsafe extern "C" fn _ltime(mut a: K) -> K {
    return _gtime(_lt(a));
}
pub unsafe extern "C" fn stat_sz(mut u: S, mut n: *mut I) -> I {
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(u as *const libc::c_char, &mut s) != 0 {
        return -(1 as libc::c_int) as I;
    }
    *n = s.st_size as I;
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn _size(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = 0 as libc::c_int as I;
    if 4 as libc::c_int as libc::c_longlong != t
        && 3 as libc::c_int as libc::c_longlong
            != (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if stat_sz(CSK(a), &mut n) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    return Kf(n as F);
}
pub unsafe extern "C" fn _bin(mut x: K, mut y: K) -> K {
    if (*x).t > 0 as libc::c_int as libc::c_longlong {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    return Ki(
        binr(x, 0 as libc::c_int as I, (*x).n - 1 as libc::c_int as libc::c_longlong, y),
    );
}
pub unsafe extern "C" fn _draw(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut y: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    let mut c: I = *(((*b).k).as_mut_ptr() as *mut I);
    let mut n: I = 1 as libc::c_int as I;
    let mut j: I = 0 as libc::c_int as I;
    let mut k: I = 0;
    let mut s: I = 0;
    if 1 as libc::c_int as libc::c_longlong
        != (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        || 1 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = an;
    while i < _i_0 {
        n *= *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize);
        if n < 0 as libc::c_int as libc::c_longlong {
            return kerr(b"int\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    if c < 0 as libc::c_int as libc::c_longlong && n > -c {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    y = newK((if c != 0 { -(1 as libc::c_int) } else { -(2 as libc::c_int) }) as I, n);
    if c == 0 {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_0 < _i_1 {
            *(((*y).k).as_mut_ptr() as *mut F).offset(i_0 as isize) = RF();
            i_0 += 1;
            i_0;
        }
    } else if c > 0 as libc::c_int as libc::c_longlong {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_1 < _i_2 {
            *(((*y).k).as_mut_ptr() as *mut I)
                .offset(i_1 as isize) = (c as libc::c_double * RF()) as I;
            i_1 += 1;
            i_1;
        }
    } else if c < 0 as libc::c_int as libc::c_longlong {
        let mut d: I = -c;
        vitter(((*y).k).as_mut_ptr() as *mut I, (*y).n, d);
        j = n - 1 as libc::c_int as libc::c_longlong;
        while j > 0 as libc::c_int as libc::c_longlong {
            k = ((1 as libc::c_int as libc::c_longlong + j) as libc::c_double * RF())
                as I;
            s = *(((*y).k).as_mut_ptr() as *mut I).offset(j as isize);
            *(((*y).k).as_mut_ptr() as *mut I)
                .offset(
                    j as isize,
                ) = *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize);
            *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize) = s;
            j -= 1;
            j;
        }
    }
    z = take_reshape(a, y);
    cd(y);
    return z;
}
unsafe extern "C" fn vitter_a(mut a: *mut I, mut n: I, mut N: I, mut j: I) {
    let mut S: I = 0;
    let mut i: I = 0 as libc::c_int as I;
    let mut top: F = (N - n) as F;
    let mut Nreal: F = N as F;
    let mut V: F = 0.;
    let mut quot: F = 0.;
    while n >= 2 as libc::c_int as libc::c_longlong {
        V = RF();
        S = 0 as libc::c_int as I;
        quot = top / Nreal;
        while quot > V {
            S += 1;
            S;
            top -= 1.;
            top;
            Nreal -= 1.;
            Nreal;
            quot = quot * top / Nreal;
        }
        j += S + 1 as libc::c_int as libc::c_longlong;
        let fresh42 = i;
        i = i + 1;
        *a.offset(fresh42 as isize) = j;
        Nreal -= 1.;
        Nreal;
        n -= 1;
        n;
    }
    S = floor(round(Nreal) * RF()) as I;
    j += S + 1 as libc::c_int as libc::c_longlong;
    let fresh43 = i;
    i = i + 1;
    *a.offset(fresh43 as isize) = j;
}
pub unsafe extern "C" fn vitter(mut a: *mut I, mut n: I, mut N: I) {
    let mut i: I = 0 as libc::c_int as I;
    let mut j: I = -(1 as libc::c_int) as I;
    let mut t: I = 0;
    let mut qu1: I = -n + 1 as libc::c_int as libc::c_longlong + N;
    let mut S: I = 0;
    let mut negalphainv: I = -(13 as libc::c_int) as I;
    let mut threshold: I = -negalphainv * n;
    let mut nreal: F = n as F;
    let mut Nreal: F = N as F;
    let mut ninv: F = 1.0f64 / n as libc::c_double;
    let mut nmin1inv: F = 1.0f64
        / (n - 1 as libc::c_int as libc::c_longlong) as libc::c_double;
    let mut Vprime: F = exp(log(RF()) * ninv);
    let mut qu1real: F = -nreal + 1.0f64 + Nreal;
    let mut negSreal: F = 0.;
    let mut U: F = 0.;
    let mut X_0: F = 0.;
    let mut y1: F = 0.;
    let mut y2: F = 0.;
    let mut top: F = 0.;
    let mut bottom: F = 0.;
    let mut limit: F = 0.;
    while n > 1 as libc::c_int as libc::c_longlong && threshold < N {
        nmin1inv = 1.0f64 / (-1.0f64 + nreal);
        loop {
            loop {
                X_0 = Nreal * (-Vprime + 1.0f64);
                S = floor(X_0) as I;
                if S < qu1 {
                    break;
                }
                Vprime = exp(log(RF()) * ninv);
            }
            U = RF();
            negSreal = -S as F;
            y1 = exp(log(U * Nreal / qu1real) * nmin1inv);
            Vprime = y1 * (-X_0 / Nreal + 1.0f64) * (qu1real / (negSreal + qu1real));
            if Vprime <= 1.0f64 {
                break;
            }
            y2 = 1.0f64;
            top = -1.0f64 + Nreal;
            if -(1 as libc::c_int) as libc::c_longlong + n > S {
                bottom = -nreal + Nreal;
                limit = (-S + N) as F;
            } else {
                bottom = -1.0f64 + negSreal + Nreal;
                limit = qu1 as F;
            }
            t = N - 1 as libc::c_int as libc::c_longlong;
            while t as libc::c_double >= limit {
                y2 = y2 * top / bottom;
                top -= 1.;
                top;
                bottom -= 1.;
                bottom;
                t -= 1;
                t;
            }
            if Nreal / (-X_0 + Nreal) >= y1 * exp(log(y2) * nmin1inv) {
                Vprime = exp(log(RF()) * nmin1inv);
                break;
            } else {
                Vprime = exp(log(RF()) * ninv);
            }
        }
        j += S + 1 as libc::c_int as libc::c_longlong;
        let fresh44 = i;
        i = i + 1;
        *a.offset(fresh44 as isize) = j;
        N = -S + (-(1 as libc::c_int) as libc::c_longlong + N);
        Nreal = negSreal + (-1.0f64 + Nreal);
        n -= 1;
        n;
        nreal -= 1.;
        nreal;
        ninv = nmin1inv;
        qu1 = -S + qu1;
        qu1real = negSreal + qu1real;
        threshold += negalphainv;
    }
    if n > 1 as libc::c_int as libc::c_longlong {
        vitter_a(a.offset(i as isize), n, N, j);
    } else {
        S = floor(N as libc::c_double * Vprime) as I;
        j += S + 1 as libc::c_int as libc::c_longlong;
        let fresh45 = i;
        i = i + 1;
        *a.offset(fresh45 as isize) = j;
    };
}
pub unsafe extern "C" fn _lsq(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut TOL: F = 1.0e-6f64;
    let mut s: F = 0.;
    if at > 0 as libc::c_int as libc::c_longlong
        || at < -(2 as libc::c_int) as libc::c_longlong || bt != 0
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if an == 0 || bn == 0 {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut x: K = 0 as *mut k0;
    let mut y: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    let mut r: I = (**((*b).k).as_mut_ptr().offset(0 as libc::c_int as isize)).n;
    if r <= 0 as libc::c_int as libc::c_longlong {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = bn;
    while i < _i_0 {
        y = *((*b).k).as_mut_ptr().offset(i as isize);
        if (*y).t != -(1 as libc::c_int) as libc::c_longlong
            && (*y).t != -(2 as libc::c_int) as libc::c_longlong
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        if r != (*y).n {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    if at == 0 {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = an;
        while i_0 < _i_1 {
            y = *((*a).k).as_mut_ptr().offset(i_0 as isize);
            if (*y).t != -(1 as libc::c_int) as libc::c_longlong
                && (*y).t != -(2 as libc::c_int) as libc::c_longlong
            {
                return kerr(b"type\0" as *const u8 as *const libc::c_char);
            }
            if r != (*y).n {
                return kerr(b"length\0" as *const u8 as *const libc::c_char);
            }
            i_0 += 1;
            i_0;
        }
    } else if r != an {
        return kerr(b"length\0" as *const u8 as *const libc::c_char)
    }
    let mut n: I = bn;
    let mut m: I = if r > n { r } else { n };
    let mut u: *mut *mut F = alloc(
        (m as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut F>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut *mut F;
    let ref mut fresh46 = *u.offset(0 as libc::c_int as isize);
    *fresh46 = alloc(
        ((n * m) as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut F;
    let mut w: *mut F = alloc(
        (n as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut F;
    let mut v: *mut *mut F = alloc(
        (n as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut F>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut *mut F;
    let ref mut fresh47 = *v.offset(0 as libc::c_int as isize);
    *fresh47 = alloc(
        ((n * n) as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut F;
    let mut t: *mut F = alloc(
        (n as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut F;
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_2: I = m;
    while i_1 < _i_2 {
        let ref mut fresh48 = *u.offset(i_1 as isize);
        *fresh48 = (*u.offset(0 as libc::c_int as isize)).offset((n * i_1) as isize);
        i_1 += 1;
        i_1;
    }
    let mut i_2: I = 0 as libc::c_int as I;
    let mut _i_3: I = n;
    while i_2 < _i_3 {
        let ref mut fresh49 = *v.offset(i_2 as isize);
        *fresh49 = (*v.offset(0 as libc::c_int as isize)).offset((n * i_2) as isize);
        i_2 += 1;
        i_2;
    }
    let mut i_3: I = 0 as libc::c_int as I;
    let mut _i_4: I = n * m;
    while i_3 < _i_4 {
        *(*u.offset(0 as libc::c_int as isize))
            .offset(i_3 as isize) = 0 as libc::c_int as F;
        i_3 += 1;
        i_3;
    }
    let mut i_4: I = 0 as libc::c_int as I;
    let mut _i_5: I = r;
    while i_4 < _i_5 {
        let mut j: I = 0 as libc::c_int as I;
        let mut _j: I = n;
        while j < _j {
            y = *((*b).k).as_mut_ptr().offset(j as isize);
            *(*u.offset(i_4 as isize))
                .offset(
                    j as isize,
                ) = if -(2 as libc::c_int) as libc::c_longlong == (*y).t {
                *(((*y).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
            } else {
                *(((*y).k).as_mut_ptr() as *mut I).offset(i_4 as isize) as libc::c_double
            };
            j += 1;
            j;
        }
        i_4 += 1;
        i_4;
    }
    svdcmp(u, m, n, w, v, t);
    let mut wmax: F = 0.0f64;
    let mut i_5: I = 0 as libc::c_int as I;
    let mut _i_6: I = n;
    while i_5 < _i_6 {
        if *w.offset(i_5 as isize) > wmax {
            wmax = *w.offset(i_5 as isize);
        }
        i_5 += 1;
        i_5;
    }
    let mut thresh: F = TOL * wmax;
    let mut i_6: I = 0 as libc::c_int as I;
    let mut _i_7: I = n;
    while i_6 < _i_7 {
        if *w.offset(i_6 as isize) < thresh {
            *w.offset(i_6 as isize) = 0.0f64;
        }
        i_6 += 1;
        i_6;
    }
    if at == 0 {
        z = newK(0 as libc::c_int as I, an);
        let mut i_7: I = 0 as libc::c_int as I;
        let mut _i_8: I = an;
        while i_7 < _i_8 {
            let ref mut fresh50 = *((*z).k).as_mut_ptr().offset(i_7 as isize);
            *fresh50 = newK(-(2 as libc::c_int) as I, n);
            i_7 += 1;
            i_7;
        }
    } else {
        z = newK(-(2 as libc::c_int) as I, n);
    }
    let mut k: I = 0 as libc::c_int as I;
    let mut _k_0: I = if at != 0 { 1 as libc::c_int as libc::c_longlong } else { an };
    while k < _k_0 {
        y = if at != 0 { a } else { *((*a).k).as_mut_ptr().offset(k as isize) };
        x = if at != 0 { z } else { *((*z).k).as_mut_ptr().offset(k as isize) };
        let mut i_8: I = 0 as libc::c_int as I;
        let mut _i_9: I = n;
        while i_8 < _i_9 {
            s = 0.0f64;
            if *w.offset(i_8 as isize) != 0. {
                let mut j_0: I = 0 as libc::c_int as I;
                let mut _j_0: I = m;
                while j_0 < _j_0 {
                    s
                        += *(*u.offset(j_0 as isize)).offset(i_8 as isize)
                            * (if -(2 as libc::c_int) as libc::c_longlong == (*y).t {
                                *(((*y).k).as_mut_ptr() as *mut F).offset(j_0 as isize)
                            } else {
                                *(((*y).k).as_mut_ptr() as *mut I).offset(j_0 as isize)
                                    as libc::c_double
                            });
                    j_0 += 1;
                    j_0;
                }
                s /= *w.offset(i_8 as isize);
            }
            *t.offset(i_8 as isize) = s;
            i_8 += 1;
            i_8;
        }
        let mut i_9: I = 0 as libc::c_int as I;
        let mut _i_10: I = n;
        while i_9 < _i_10 {
            s = 0.0f64;
            let mut j_1: I = 0 as libc::c_int as I;
            let mut _j_1: I = n;
            while j_1 < _j_1 {
                s
                    += *(*v.offset(i_9 as isize)).offset(j_1 as isize)
                        * *t.offset(j_1 as isize);
                *(((*x).k).as_mut_ptr() as *mut F).offset(i_9 as isize) = s;
                j_1 += 1;
                j_1;
            }
            i_9 += 1;
            i_9;
        }
        k += 1;
        k;
    }
    free(*u.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(u as *mut libc::c_void);
    free(w as *mut libc::c_void);
    free(*v.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(v as *mut libc::c_void);
    free(t as *mut libc::c_void);
    return z;
}
unsafe extern "C" fn radius(mut u: F, mut v: F) -> F {
    let mut Au: F = 0.;
    let mut Av: F = 0.;
    let mut Aw: F = 0.;
    Au = if u < 0 as libc::c_int as libc::c_double { -u } else { u };
    Av = if v < 0 as libc::c_int as libc::c_double { -v } else { v };
    if Au > Av {
        Aw = Av / Au;
        return Au * sqrt(1.0f64 + Aw * Aw);
    }
    if Av != 0.0f64 {
        Aw = Au / Av;
        return Av * sqrt(1.0f64 + Aw * Aw);
    }
    return 0.0f64;
}
unsafe extern "C" fn svdcmp(
    mut a: *mut *mut F,
    mut m: I,
    mut n: I,
    mut w: *mut F,
    mut v: *mut *mut F,
    mut t: *mut F,
) {
    let mut flag: I = 0;
    let mut i: I = 0;
    let mut its: I = 0;
    let mut j: I = 0;
    let mut jj: I = 0;
    let mut k: I = 0;
    let mut l: I = 0;
    let mut nm: I = 0;
    let mut nm1: I = n - 1 as libc::c_int as libc::c_longlong;
    let mut mm1: I = m - 1 as libc::c_int as libc::c_longlong;
    let mut c: F = 0.;
    let mut f: F = 0.;
    let mut h: F = 0.;
    let mut s: F = 0.;
    let mut x: F = 0.;
    let mut y: F = 0.;
    let mut z: F = 0.;
    let mut anorm: F = 0.0f64;
    let mut g: F = 0.0f64;
    let mut scale: F = 0.0f64;
    let mut rv1: *mut F = 0 as *mut F;
    rv1 = t;
    i = 0 as libc::c_int as I;
    while i < n {
        l = i + 1 as libc::c_int as libc::c_longlong;
        *rv1.offset(i as isize) = scale * g;
        scale = 0.0f64;
        s = scale;
        g = s;
        if i < m {
            k = i;
            while k < m {
                scale
                    += if *(*a.offset(k as isize)).offset(i as isize)
                        < 0 as libc::c_int as libc::c_double
                    {
                        -*(*a.offset(k as isize)).offset(i as isize)
                    } else {
                        *(*a.offset(k as isize)).offset(i as isize)
                    };
                k += 1;
                k;
            }
            if scale != 0. {
                k = i;
                while k < m {
                    let ref mut fresh51 = *(*a.offset(k as isize)).offset(i as isize);
                    *fresh51 /= scale;
                    s
                        += *(*a.offset(k as isize)).offset(i as isize)
                            * *(*a.offset(k as isize)).offset(i as isize);
                    k += 1;
                    k;
                }
                f = *(*a.offset(i as isize)).offset(i as isize);
                g = -if f >= 0.0f64 {
                    if sqrt(s) < 0 as libc::c_int as libc::c_double {
                        -sqrt(s)
                    } else {
                        sqrt(s)
                    }
                } else {
                    -if sqrt(s) < 0 as libc::c_int as libc::c_double {
                        -sqrt(s)
                    } else {
                        sqrt(s)
                    }
                };
                h = f * g - s;
                *(*a.offset(i as isize)).offset(i as isize) = f - g;
                if i != nm1 {
                    j = l;
                    while j < n {
                        s = 0.0f64;
                        k = i;
                        while k < m {
                            s
                                += *(*a.offset(k as isize)).offset(i as isize)
                                    * *(*a.offset(k as isize)).offset(j as isize);
                            k += 1;
                            k;
                        }
                        f = s / h;
                        k = i;
                        while k < m {
                            let ref mut fresh52 = *(*a.offset(k as isize))
                                .offset(j as isize);
                            *fresh52 += f * *(*a.offset(k as isize)).offset(i as isize);
                            k += 1;
                            k;
                        }
                        j += 1;
                        j;
                    }
                }
                k = i;
                while k < m {
                    let ref mut fresh53 = *(*a.offset(k as isize)).offset(i as isize);
                    *fresh53 *= scale;
                    k += 1;
                    k;
                }
            }
        }
        *w.offset(i as isize) = scale * g;
        scale = 0.0f64;
        s = scale;
        g = s;
        if i < m && i != nm1 {
            k = l;
            while k < n {
                scale
                    += if *(*a.offset(i as isize)).offset(k as isize)
                        < 0 as libc::c_int as libc::c_double
                    {
                        -*(*a.offset(i as isize)).offset(k as isize)
                    } else {
                        *(*a.offset(i as isize)).offset(k as isize)
                    };
                k += 1;
                k;
            }
            if scale != 0. {
                k = l;
                while k < n {
                    let ref mut fresh54 = *(*a.offset(i as isize)).offset(k as isize);
                    *fresh54 /= scale;
                    s
                        += *(*a.offset(i as isize)).offset(k as isize)
                            * *(*a.offset(i as isize)).offset(k as isize);
                    k += 1;
                    k;
                }
                f = *(*a.offset(i as isize)).offset(l as isize);
                g = -if f >= 0.0f64 {
                    if sqrt(s) < 0 as libc::c_int as libc::c_double {
                        -sqrt(s)
                    } else {
                        sqrt(s)
                    }
                } else {
                    -if sqrt(s) < 0 as libc::c_int as libc::c_double {
                        -sqrt(s)
                    } else {
                        sqrt(s)
                    }
                };
                h = f * g - s;
                *(*a.offset(i as isize)).offset(l as isize) = f - g;
                k = l;
                while k < n {
                    *rv1
                        .offset(
                            k as isize,
                        ) = *(*a.offset(i as isize)).offset(k as isize) / h;
                    k += 1;
                    k;
                }
                if i != mm1 {
                    j = l;
                    while j < m {
                        s = 0.0f64;
                        k = l;
                        while k < n {
                            s
                                += *(*a.offset(j as isize)).offset(k as isize)
                                    * *(*a.offset(i as isize)).offset(k as isize);
                            k += 1;
                            k;
                        }
                        k = l;
                        while k < n {
                            let ref mut fresh55 = *(*a.offset(j as isize))
                                .offset(k as isize);
                            *fresh55 += s * *rv1.offset(k as isize);
                            k += 1;
                            k;
                        }
                        j += 1;
                        j;
                    }
                }
                k = l;
                while k < n {
                    let ref mut fresh56 = *(*a.offset(i as isize)).offset(k as isize);
                    *fresh56 *= scale;
                    k += 1;
                    k;
                }
            }
        }
        anorm = if anorm
            > (if *w.offset(i as isize) < 0 as libc::c_int as libc::c_double {
                -*w.offset(i as isize)
            } else {
                *w.offset(i as isize)
            })
                + (if *rv1.offset(i as isize) < 0 as libc::c_int as libc::c_double {
                    -*rv1.offset(i as isize)
                } else {
                    *rv1.offset(i as isize)
                })
        {
            anorm
        } else {
            (if *w.offset(i as isize) < 0 as libc::c_int as libc::c_double {
                -*w.offset(i as isize)
            } else {
                *w.offset(i as isize)
            })
                + (if *rv1.offset(i as isize) < 0 as libc::c_int as libc::c_double {
                    -*rv1.offset(i as isize)
                } else {
                    *rv1.offset(i as isize)
                })
        };
        i += 1;
        i;
    }
    i = n - 1 as libc::c_int as libc::c_longlong;
    while i >= 0 as libc::c_int as libc::c_longlong {
        if i < nm1 {
            if g != 0. {
                j = l;
                while j < n {
                    *(*v.offset(j as isize))
                        .offset(
                            i as isize,
                        ) = *(*a.offset(i as isize)).offset(j as isize)
                        / *(*a.offset(i as isize)).offset(l as isize) / g;
                    j += 1;
                    j;
                }
                j = l;
                while j < n {
                    s = 0.0f64;
                    k = l;
                    while k < n {
                        s
                            += *(*a.offset(i as isize)).offset(k as isize)
                                * *(*v.offset(k as isize)).offset(j as isize);
                        k += 1;
                        k;
                    }
                    k = l;
                    while k < n {
                        let ref mut fresh57 = *(*v.offset(k as isize))
                            .offset(j as isize);
                        *fresh57 += s * *(*v.offset(k as isize)).offset(i as isize);
                        k += 1;
                        k;
                    }
                    j += 1;
                    j;
                }
            }
            j = l;
            while j < n {
                let ref mut fresh58 = *(*v.offset(j as isize)).offset(i as isize);
                *fresh58 = 0.0f64;
                *(*v.offset(i as isize)).offset(j as isize) = *fresh58;
                j += 1;
                j;
            }
        }
        *(*v.offset(i as isize)).offset(i as isize) = 1.0f64;
        g = *rv1.offset(i as isize);
        l = i;
        i -= 1;
        i;
    }
    i = n - 1 as libc::c_int as libc::c_longlong;
    while i >= 0 as libc::c_int as libc::c_longlong {
        l = i + 1 as libc::c_int as libc::c_longlong;
        g = *w.offset(i as isize);
        if i < nm1 {
            j = l;
            while j < n {
                *(*a.offset(i as isize)).offset(j as isize) = 0.0f64;
                j += 1;
                j;
            }
        }
        if g != 0. {
            g = 1.0f64 / g;
            if i != nm1 {
                j = l;
                while j < n {
                    s = 0.0f64;
                    k = l;
                    while k < m {
                        s
                            += *(*a.offset(k as isize)).offset(i as isize)
                                * *(*a.offset(k as isize)).offset(j as isize);
                        k += 1;
                        k;
                    }
                    f = s / *(*a.offset(i as isize)).offset(i as isize) * g;
                    k = i;
                    while k < m {
                        let ref mut fresh59 = *(*a.offset(k as isize))
                            .offset(j as isize);
                        *fresh59 += f * *(*a.offset(k as isize)).offset(i as isize);
                        k += 1;
                        k;
                    }
                    j += 1;
                    j;
                }
            }
            j = i;
            while j < m {
                let ref mut fresh60 = *(*a.offset(j as isize)).offset(i as isize);
                *fresh60 *= g;
                j += 1;
                j;
            }
        } else {
            j = i;
            while j < m {
                *(*a.offset(j as isize)).offset(i as isize) = 0.0f64;
                j += 1;
                j;
            }
        }
        let ref mut fresh61 = *(*a.offset(i as isize)).offset(i as isize);
        *fresh61 += 1.;
        *fresh61;
        i -= 1;
        i;
    }
    k = n - 1 as libc::c_int as libc::c_longlong;
    while k >= 0 as libc::c_int as libc::c_longlong {
        its = 0 as libc::c_int as I;
        while its < 30 as libc::c_int as libc::c_longlong {
            flag = 1 as libc::c_int as I;
            l = k;
            while l >= 0 as libc::c_int as libc::c_longlong {
                nm = l - 1 as libc::c_int as libc::c_longlong;
                if (if *rv1.offset(l as isize) < 0 as libc::c_int as libc::c_double {
                    -*rv1.offset(l as isize)
                } else {
                    *rv1.offset(l as isize)
                }) + anorm == anorm
                {
                    flag = 0 as libc::c_int as I;
                    break;
                } else {
                    if (if *w.offset(nm as isize) < 0 as libc::c_int as libc::c_double {
                        -*w.offset(nm as isize)
                    } else {
                        *w.offset(nm as isize)
                    }) + anorm == anorm
                    {
                        break;
                    }
                    l -= 1;
                    l;
                }
            }
            if flag != 0 {
                c = 0.0f64;
                s = 1.0f64;
                i = l;
                while i <= k {
                    f = s * *rv1.offset(i as isize);
                    if (if f < 0 as libc::c_int as libc::c_double { -f } else { f })
                        + anorm != anorm
                    {
                        g = *w.offset(i as isize);
                        h = radius(f, g);
                        *w.offset(i as isize) = h;
                        h = 1.0f64 / h;
                        c = g * h;
                        s = -f * h;
                        j = 0 as libc::c_int as I;
                        while j < m {
                            y = *(*a.offset(j as isize)).offset(nm as isize);
                            z = *(*a.offset(j as isize)).offset(i as isize);
                            *(*a.offset(j as isize)).offset(nm as isize) = y * c + z * s;
                            *(*a.offset(j as isize)).offset(i as isize) = z * c - y * s;
                            j += 1;
                            j;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            z = *w.offset(k as isize);
            if l == k {
                if z < 0.0f64 {
                    *w.offset(k as isize) = -z;
                    j = 0 as libc::c_int as I;
                    while j < n {
                        *(*v.offset(j as isize))
                            .offset(
                                k as isize,
                            ) = -*(*v.offset(j as isize)).offset(k as isize);
                        j += 1;
                        j;
                    }
                }
                break;
            } else {
                if its == 30 as libc::c_int as libc::c_longlong {
                    kerr(b"limit\0" as *const u8 as *const libc::c_char);
                    return;
                }
                x = *w.offset(l as isize);
                nm = k - 1 as libc::c_int as libc::c_longlong;
                y = *w.offset(nm as isize);
                g = *rv1.offset(nm as isize);
                h = *rv1.offset(k as isize);
                f = ((y - z) * (y + z) + (g - h) * (g + h)) / (2.0f64 * h * y);
                g = radius(f, 1.0f64);
                f = ((x - z) * (x + z)
                    + h
                        * (y
                            / (f
                                + (if f >= 0.0f64 {
                                    (if g < 0 as libc::c_int as libc::c_double {
                                        -g
                                    } else {
                                        g
                                    })
                                } else {
                                    -(if g < 0 as libc::c_int as libc::c_double {
                                        -g
                                    } else {
                                        g
                                    })
                                })) - h)) / x;
                s = 1.0f64;
                c = s;
                j = l;
                while j <= nm {
                    i = j + 1 as libc::c_int as libc::c_longlong;
                    g = *rv1.offset(i as isize);
                    y = *w.offset(i as isize);
                    h = s * g;
                    g = c * g;
                    z = radius(f, h);
                    *rv1.offset(j as isize) = z;
                    c = f / z;
                    s = h / z;
                    f = x * c + g * s;
                    g = g * c - x * s;
                    h = y * s;
                    y = y * c;
                    jj = 0 as libc::c_int as I;
                    while jj < n {
                        x = *(*v.offset(jj as isize)).offset(j as isize);
                        z = *(*v.offset(jj as isize)).offset(i as isize);
                        *(*v.offset(jj as isize)).offset(j as isize) = x * c + z * s;
                        *(*v.offset(jj as isize)).offset(i as isize) = z * c - x * s;
                        jj += 1;
                        jj;
                    }
                    z = radius(f, h);
                    *w.offset(j as isize) = z;
                    if z != 0. {
                        z = 1.0f64 / z;
                        c = f * z;
                        s = h * z;
                    }
                    f = c * g + s * y;
                    x = c * y - s * g;
                    jj = 0 as libc::c_int as I;
                    while jj < m {
                        y = *(*a.offset(jj as isize)).offset(j as isize);
                        z = *(*a.offset(jj as isize)).offset(i as isize);
                        *(*a.offset(jj as isize)).offset(j as isize) = y * c + z * s;
                        *(*a.offset(jj as isize)).offset(i as isize) = z * c - y * s;
                        jj += 1;
                        jj;
                    }
                    j += 1;
                    j;
                }
                *rv1.offset(l as isize) = 0.0f64;
                *rv1.offset(k as isize) = f;
                *w.offset(k as isize) = x;
                its += 1;
                its;
            }
        }
        k -= 1;
        k;
    }
}
pub unsafe extern "C" fn _setenv(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut bt: I = (*b).t;
    if at != 4 as libc::c_int as libc::c_longlong
        && bt != -(3 as libc::c_int) as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut r: I = setenv(
        *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char,
        CSK(b) as *const libc::c_char,
        1 as libc::c_int,
    ) as I;
    if r != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    return _n();
}
pub unsafe extern "C" fn _sm(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if 0 as libc::c_int as libc::c_longlong != at
        && 3 as libc::c_int as libc::c_longlong
            != (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 4 as libc::c_int as libc::c_longlong
            != (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int as libc::c_longlong != bt
        && 3 as libc::c_int as libc::c_longlong
            != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        && 4 as libc::c_int as libc::c_longlong
            != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut x: I = (at <= 0 as libc::c_int as libc::c_longlong
        && -(3 as libc::c_int) as libc::c_longlong != at) as libc::c_int as I;
    let mut y: I = (bt <= 0 as libc::c_int as libc::c_longlong
        && -(3 as libc::c_int) as libc::c_longlong != bt) as libc::c_int as I;
    if x != 0 && y != 0 && an != bn {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if x != 0 || y != 0 {
        a = if x != 0 { promote(a) } else { a };
        b = if y != 0 { promote(b) } else { b };
        let mut z: K = newK(0 as libc::c_int as I, if x != 0 { (*a).n } else { (*b).n });
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*z).n;
        while i < _i_0 {
            let ref mut fresh62 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh62 = _sm(
                if x != 0 { *((*a).k).as_mut_ptr().offset(i as isize) } else { a },
                if y != 0 { *((*b).k).as_mut_ptr().offset(i as isize) } else { b },
            );
            i += 1;
            i;
        }
        cd(a);
        return demote(z);
    }
    let mut f: I = (if fnmatch(
        CSK(b) as *const libc::c_char,
        CSK(a) as *const libc::c_char,
        (1 as libc::c_int) << 1 as libc::c_int,
    ) != 0
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as I;
    return Ki(f);
}
pub unsafe extern "C" fn _ss(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if 0 as libc::c_int as libc::c_longlong != at
        && 3 as libc::c_int as libc::c_longlong
            != (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 4 as libc::c_int as libc::c_longlong
            != (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int as libc::c_longlong != bt
        && 3 as libc::c_int as libc::c_longlong
            != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        && 4 as libc::c_int as libc::c_longlong
            != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut x: I = (at <= 0 as libc::c_int as libc::c_longlong
        && -(3 as libc::c_int) as libc::c_longlong != at) as libc::c_int as I;
    let mut y: I = (bt <= 0 as libc::c_int as libc::c_longlong
        && -(3 as libc::c_int) as libc::c_longlong != bt) as libc::c_int as I;
    if x != 0 && y != 0 && an != bn {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if (4 as libc::c_int as libc::c_longlong == at
        || 3 as libc::c_int as libc::c_longlong
            == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }))
        && an == 0
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if x != 0 || y != 0 {
        a = if x != 0 { promote(a) } else { ci(a) };
        b = if y != 0 { promote(b) } else { ci(b) };
        if OOM_CD(0 as libc::c_int as I, a, b, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut z: K = newK(0 as libc::c_int as I, if x != 0 { (*a).n } else { (*b).n });
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*z).n;
        while i < _i_0 {
            let ref mut fresh63 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh63 = _ss(
                if x != 0 { *((*a).k).as_mut_ptr().offset(i as isize) } else { a },
                if y != 0 { *((*b).k).as_mut_ptr().offset(i as isize) } else { b },
            );
            if OOM_CD(0 as libc::c_int as I, a, b, z, *fresh63, -(1 as libc::c_int) as V)
                == 0
            {
                return 0 as K;
            }
            i += 1;
            i;
        }
        cd(a);
        cd(b);
        return demote(z);
    }
    let mut t: S = CSK(a);
    let mut p: S = CSK(b);
    let mut lp: I = strlen(p as *const libc::c_char) as I;
    if lp == 0 {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut r: *mut I = alloc(
        (lp as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut I;
    let mut n: I = (if 3 as libc::c_int as libc::c_longlong
        == (if (*a).t < 0 as libc::c_int as libc::c_longlong { -(*a).t } else { (*a).t })
    {
        (*a).n as libc::c_ulonglong
    } else {
        strlen(t as *const libc::c_char) as libc::c_ulonglong
    }) as I;
    let mut m: I = 0 as libc::c_int as I;
    let mut c: C = 0;
    let mut d: C = 0;
    let mut occ: [I; 256] = [0; 256];
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_1: I = 256 as libc::c_int as I;
    while i_0 < _i_1 {
        occ[i_0 as usize] = -(1 as libc::c_int) as I;
        i_0 += 1;
        i_0;
    }
    let mut v: [C; 256] = [0; 256];
    let mut w: [C; 256] = [0; 256];
    let mut q: S = p;
    while *q != 0 {
        *r.offset(m as isize) = q.offset_from(p) as libc::c_long as I;
        if '?' as i32 == *q as libc::c_int {
            q = q.offset(1);
            q;
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_2: I = 256 as libc::c_int as I;
            while i_1 < _i_2 {
                occ[i_1 as usize] = m;
                i_1 += 1;
                i_1;
            }
        } else if '[' as i32 != *q as libc::c_int {
            let fresh64 = q;
            q = q.offset(1);
            occ[*fresh64 as I as usize] = m;
        } else {
            q = rangematch(
                q.offset(1 as libc::c_int as isize),
                0 as libc::c_int as C,
                v.as_mut_ptr(),
            );
            if q.is_null() {
                free(p as *mut libc::c_void);
                return kerr(b"domain\0" as *const u8 as *const libc::c_char);
            }
            let mut any: I = 0 as libc::c_int as I;
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_3: I = 256 as libc::c_int as I;
            while i_2 < _i_3 {
                if v[i_2 as usize] != 0 {
                    occ[i_2 as usize] = m;
                    any = 1 as libc::c_int as I;
                }
                i_2 += 1;
                i_2;
            }
            if any == 0 {
                free(p as *mut libc::c_void);
                return newK(-(1 as libc::c_int) as I, 0 as libc::c_int as I);
            }
        }
        m += 1;
        m;
    }
    let mut z_0: K = newK(-(1 as libc::c_int) as I, 0 as libc::c_int as I);
    let mut f: *mut I = alloc(
        ((m + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut I;
    let mut s: *mut I = alloc(
        ((m + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
            ) as size_t,
    ) as *mut I;
    let mut i_3: I = 0 as libc::c_int as I;
    let mut _i_4: I = m + 1 as libc::c_int as libc::c_longlong;
    while i_3 < _i_4 {
        let ref mut fresh65 = *s.offset(i_3 as isize);
        *fresh65 = 0 as libc::c_int as I;
        *f.offset(i_3 as isize) = *fresh65;
        i_3 += 1;
        i_3;
    }
    let mut i_4: I = m;
    let mut j: I = m + 1 as libc::c_int as libc::c_longlong;
    *f.offset(i_4 as isize) = j;
    let mut flag: I = 0;
    while i_4 > 0 as libc::c_int as libc::c_longlong {
        while j <= m {
            flag = 0 as libc::c_int as I;
            c = *p
                .offset(
                    *r.offset((i_4 - 1 as libc::c_int as libc::c_longlong) as isize)
                        as isize,
                );
            d = *p
                .offset(
                    *r.offset((j - 1 as libc::c_int as libc::c_longlong) as isize)
                        as isize,
                );
            if '?' as i32 == c as libc::c_int || '?' as i32 == d as libc::c_int {
                break;
            }
            if c as libc::c_int == d as libc::c_int && '[' as i32 != c as libc::c_int
                && '[' as i32 != d as libc::c_int
            {
                break;
            }
            if '[' as i32 != c as libc::c_int && '[' as i32 == d as libc::c_int
                && !(rangematch(
                    p
                        .offset(
                            *r
                                .offset((j - 1 as libc::c_int as libc::c_longlong) as isize)
                                as isize,
                        )
                        .offset(1 as libc::c_int as isize),
                    c,
                    0 as S,
                ))
                    .is_null()
            {
                break;
            }
            if '[' as i32 == c as libc::c_int && '[' as i32 != d as libc::c_int
                && !(rangematch(
                    p
                        .offset(
                            *r
                                .offset(
                                    (i_4 - 1 as libc::c_int as libc::c_longlong) as isize,
                                ) as isize,
                        )
                        .offset(1 as libc::c_int as isize),
                    d,
                    0 as S,
                ))
                    .is_null()
            {
                break;
            }
            if '[' as i32 == c as libc::c_int && '[' as i32 == d as libc::c_int {
                rangematch(
                    p
                        .offset(
                            *r
                                .offset(
                                    (i_4 - 1 as libc::c_int as libc::c_longlong) as isize,
                                ) as isize,
                        )
                        .offset(1 as libc::c_int as isize),
                    0 as libc::c_int as C,
                    v.as_mut_ptr(),
                );
                rangematch(
                    p
                        .offset(
                            *r
                                .offset((j - 1 as libc::c_int as libc::c_longlong) as isize)
                                as isize,
                        )
                        .offset(1 as libc::c_int as isize),
                    0 as libc::c_int as C,
                    w.as_mut_ptr(),
                );
                let mut i_5: I = 0 as libc::c_int as I;
                let mut _i_5: I = 256 as libc::c_int as I;
                while i_5 < _i_5 {
                    if v[i_5 as usize] as libc::c_int != 0
                        && w[i_5 as usize] as libc::c_int != 0
                    {
                        flag = 1 as libc::c_int as I;
                        break;
                    } else {
                        i_5 += 1;
                        i_5;
                    }
                }
                if flag != 0 {
                    break;
                }
            }
            if *s.offset(j as isize) == 0 as libc::c_int as libc::c_longlong {
                *s.offset(j as isize) = j - i_4;
            }
            j = *f.offset(j as isize);
        }
        i_4 -= 1;
        i_4;
        j -= 1;
        j;
        *f.offset(i_4 as isize) = j;
    }
    j = *f.offset(0 as libc::c_int as isize);
    let mut i_6: I = 0 as libc::c_int as I;
    let mut _i_6: I = m;
    while i_6 < _i_6 {
        if *s.offset(i_6 as isize) == 0 as libc::c_int as libc::c_longlong {
            *s.offset(i_6 as isize) = j;
        }
        if i_6 == j {
            j = *f.offset(j as isize);
        }
        i_6 += 1;
        i_6;
    }
    i_4 = 0 as libc::c_int as I;
    while i_4 <= n - m {
        if 4 as libc::c_int as libc::c_longlong == (*b).t {
            while i_4 < n - m
                && (*(*__ctype_b_loc())
                    .offset(*t.offset((i_4 + m) as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || i_4 != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *t
                                    .offset(
                                        (i_4 - 1 as libc::c_int as libc::c_longlong) as isize,
                                    ) as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
            {
                i_4 += 1;
                i_4;
            }
            if i_4 == n - m && i_4 != 0
                && *(*__ctype_b_loc())
                    .offset(
                        *t.offset((i_4 - 1 as libc::c_int as libc::c_longlong) as isize)
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                break;
            }
        }
        j = m - 1 as libc::c_int as libc::c_longlong;
        while j >= 0 as libc::c_int as libc::c_longlong {
            let mut c_0: C = *p.offset(*r.offset(j as isize) as isize);
            if !('?' as i32 == c_0 as libc::c_int) {
                if '[' as i32 != c_0 as libc::c_int
                    && c_0 as libc::c_int != *t.offset((i_4 + j) as isize) as libc::c_int
                {
                    break;
                }
                if '[' as i32 == c_0 as libc::c_int
                    && (rangematch(
                        p
                            .offset(*r.offset(j as isize) as isize)
                            .offset(1 as libc::c_int as isize),
                        *t.offset((i_4 + j) as isize),
                        0 as S,
                    ))
                        .is_null()
                {
                    break;
                }
            }
            j -= 1;
            j;
        }
        if j < 0 as libc::c_int as libc::c_longlong {
            kap(&mut z_0, &mut i_4 as *mut I as V);
            i_4 += m;
        } else {
            i_4
                += if *s.offset((j + 1 as libc::c_int as libc::c_longlong) as isize)
                    > j - occ[*t.offset((i_4 + j) as isize) as I as usize]
                {
                    *s.offset((j + 1 as libc::c_int as libc::c_longlong) as isize)
                } else {
                    j - occ[*t.offset((i_4 + j) as isize) as I as usize]
                };
        }
    }
    free(r as *mut libc::c_void);
    free(f as *mut libc::c_void);
    free(s as *mut libc::c_void);
    return z_0;
}
unsafe extern "C" fn rangematch(mut p: S, mut t: C, mut r: S) -> S {
    let mut n: I = 0;
    let mut k: I = 0 as libc::c_int as I;
    let mut c: C = 0;
    let mut d: C = 0;
    n = ('^' as i32 == *p as libc::c_int) as libc::c_int as I;
    if n != 0 {
        p = p.offset(1);
        p;
    }
    if !r.is_null() {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = 256 as libc::c_int as I;
        while i < _i_0 {
            *r.offset(i as isize) = n as C;
            i += 1;
            i;
        }
    }
    if ']' as i32 == *p as libc::c_int {
        if ']' as i32 == t as libc::c_int {
            k = 1 as libc::c_int as I;
        }
        if !r.is_null() {
            *r.offset(']' as i32 as UC as isize) = (n == 0) as libc::c_int as C;
        }
        p = p.offset(1);
        p;
    }
    loop {
        let fresh66 = p;
        p = p.offset(1);
        c = *fresh66;
        if !(']' as i32 != c as libc::c_int) {
            break;
        }
        if c == 0 {
            return 0 as S;
        }
        if '-' as i32 == *p as libc::c_int
            && {
                d = *p.offset(1 as libc::c_int as isize);
                d as libc::c_int != 0
            } && ']' as i32 != d as libc::c_int
        {
            p = p.offset(2 as libc::c_int as isize);
            if d == 0 {
                return 0 as S;
            }
            if c as UC as libc::c_int <= t as UC as libc::c_int
                && t as UC as libc::c_int <= d as UC as libc::c_int
            {
                k = 1 as libc::c_int as I;
            }
            if !r.is_null() {
                let mut i_0: I = 0 as libc::c_int as I;
                let mut _i_1: I = (1 as libc::c_int + d as UC as libc::c_int
                    - c as UC as libc::c_int) as I;
                while i_0 < _i_1 {
                    *r
                        .offset(
                            (i_0 + c as UC as libc::c_longlong) as isize,
                        ) = (n == 0) as libc::c_int as C;
                    i_0 += 1;
                    i_0;
                }
            }
        } else {
            if c as libc::c_int == t as libc::c_int {
                k = 1 as libc::c_int as I;
            }
            if !r.is_null() {
                *r.offset(c as UC as isize) = (n == 0) as libc::c_int as C;
            }
        }
    }
    return if t as libc::c_int != 0 && k == n { 0 as S } else { p };
}
pub unsafe extern "C" fn Ireverse(mut x: K) {
    let mut i: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*x).n / 2 as libc::c_int as libc::c_longlong;
    while i < _i_0 {
        let mut t: I = *(((*x).k).as_mut_ptr() as *mut I)
            .offset(((*x).n - i - 1 as libc::c_int as libc::c_longlong) as isize);
        *(((*x).k).as_mut_ptr() as *mut I)
            .offset(
                ((*x).n - i - 1 as libc::c_int as libc::c_longlong) as isize,
            ) = *(((*x).k).as_mut_ptr() as *mut I).offset(i as isize);
        *(((*x).k).as_mut_ptr() as *mut I).offset(i as isize) = t;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _vsx(mut x: K, mut y: K) -> K {
    if (1 as libc::c_int as libc::c_longlong)
        < (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
        || 1 as libc::c_int as libc::c_longlong
            != (if (*x).t < 0 as libc::c_int as libc::c_longlong {
                -(*x).t
            } else {
                (*x).t
            })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = 0 as K;
    if 0 as libc::c_int as libc::c_longlong == (*y).t {
        z = newK(0 as libc::c_int as I, (*y).n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*y).n;
        while i < _i_0 {
            let ref mut fresh67 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh67 = _vsx(x, *((*y).k).as_mut_ptr().offset(i as isize));
            if OOM_CD(0 as libc::c_int as I, z, *fresh67, -(1 as libc::c_int) as V) == 0
            {
                return 0 as K;
            }
            i += 1;
            i;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == (*y).t {
        z = newK(0 as libc::c_int as I, (*y).n);
        let mut k: K = Ki(0 as libc::c_int as I);
        if OOM_CD(0 as libc::c_int as I, k, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = (*y).n;
        while i_0 < _i_1 {
            *(((*k).k).as_mut_ptr()
                as *mut I) = *(((*y).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            let ref mut fresh68 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh68 = _vsx(x, k);
            if OOM_CD(0 as libc::c_int as I, z, k, *fresh68, -(1 as libc::c_int) as V)
                == 0
            {
                return 0 as K;
            }
            i_0 += 1;
            i_0;
        }
        cd(k);
        z = demote(z);
    } else if 1 as libc::c_int as libc::c_longlong == (*x).t {
        if *(((*x).k).as_mut_ptr() as *mut I) < 2 as libc::c_int as libc::c_longlong {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        z = newK(-(1 as libc::c_int) as I, 0 as libc::c_int as I);
        if z.is_null() {
            return 0 as K;
        }
        let mut a: I = *(((*x).k).as_mut_ptr() as *mut I);
        let mut b: I = *(((*y).k).as_mut_ptr() as *mut I);
        let mut c: I = b / a;
        while (*z).n == 0 || b != c {
            kap(&mut z, &mut b as *mut I as V);
            b = c;
            c = b / a;
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_2: I = (*z).n;
        while i_1 < _i_2 {
            let ref mut fresh69 = *(((*z).k).as_mut_ptr() as *mut I)
                .offset(i_1 as isize);
            *fresh69 %= a;
            i_1 += 1;
            i_1;
        }
        Ireverse(z);
    } else if -(1 as libc::c_int) as libc::c_longlong == (*x).t {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_3: I = (*x).n;
        while i_2 < _i_3 {
            if *(((*x).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                < 1 as libc::c_int as libc::c_longlong
            {
                return kerr(b"domain\0" as *const u8 as *const libc::c_char);
            }
            i_2 += 1;
            i_2;
        }
        z = newK(-(1 as libc::c_int) as I, (*x).n);
        if z.is_null() {
            return 0 as K;
        }
        let mut a_0: I = *(((*y).k).as_mut_ptr() as *mut I);
        let mut n: I = (*z).n;
        if a_0 < 0 as libc::c_int as libc::c_longlong {
            let mut s: I = 1 as libc::c_int as I;
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_4: I = (*x).n;
            while i_3 < _i_4 {
                s *= *(((*x).k).as_mut_ptr() as *mut I).offset(i_3 as isize);
                i_3 += 1;
                i_3;
            }
            a_0 = s - -a_0 % s;
        }
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_5: I = n;
        while i_4 < _i_5 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_4 as isize,
                ) = *(((*x).k).as_mut_ptr() as *mut I)
                .offset(((*x).n - 1 as libc::c_int as libc::c_longlong - i_4) as isize);
            i_4 += 1;
            i_4;
        }
        if n != 0 {
            *(((*z).k).as_mut_ptr()
                as *mut I) = if *(((*z).k).as_mut_ptr() as *mut I) != 0 {
                a_0 / *(((*z).k).as_mut_ptr() as *mut I)
            } else {
                0 as libc::c_int as libc::c_longlong
            };
        }
        let mut i_5: I = 0 as libc::c_int as I;
        let mut _i_6: I = n - 1 as libc::c_int as libc::c_longlong;
        while i_5 < _i_6 {
            let mut d: I = *(((*z).k).as_mut_ptr() as *mut I)
                .offset((i_5 + 1 as libc::c_int as libc::c_longlong) as isize);
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    (i_5 + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = if d != 0 {
                *(((*z).k).as_mut_ptr() as *mut I).offset(i_5 as isize) / d
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            i_5 += 1;
            i_5;
        }
        let mut i_6: I = 0 as libc::c_int as I;
        let mut _i_7: I = n - 1 as libc::c_int as libc::c_longlong;
        while i_6 < _i_7 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    (n - 1 as libc::c_int as libc::c_longlong - i_6) as isize,
                ) = *(((*z).k).as_mut_ptr() as *mut I)
                .offset((n - 2 as libc::c_int as libc::c_longlong - i_6) as isize)
                - *(((*z).k).as_mut_ptr() as *mut I)
                    .offset((n - 1 as libc::c_int as libc::c_longlong - i_6) as isize)
                    * *(((*x).k).as_mut_ptr() as *mut I).offset(i_6 as isize);
            i_6 += 1;
            i_6;
        }
        if n != 0 {
            *(((*z).k).as_mut_ptr()
                as *mut I) = a_0
                - *(((*z).k).as_mut_ptr() as *mut I)
                    * *(((*x).k).as_mut_ptr() as *mut I)
                        .offset(
                            ((*x).n - 1 as libc::c_int as libc::c_longlong) as isize,
                        );
        }
        Ireverse(z);
        return z;
    }
    return z;
}
pub unsafe extern "C" fn _vs(mut x: K, mut y: K) -> K {
    if (1 as libc::c_int as libc::c_longlong)
        < (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
        || 1 as libc::c_int as libc::c_longlong
            != (if (*x).t < 0 as libc::c_int as libc::c_longlong {
                -(*x).t
            } else {
                (*x).t
            })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = 0 as K;
    if 0 as libc::c_int as libc::c_longlong == (*y).t {
        z = newK(0 as libc::c_int as I, (*y).n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*y).n;
        while i < _i_0 {
            let ref mut fresh70 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh70 = _vs(x, *((*y).k).as_mut_ptr().offset(i as isize));
            if OOM_CD(0 as libc::c_int as I, z, *fresh70, -(1 as libc::c_int) as V) == 0
            {
                return 0 as K;
            }
            i += 1;
            i;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == (*y).t {
        z = newK(0 as libc::c_int as I, (*y).n);
        let mut k: K = Ki(0 as libc::c_int as I);
        if OOM_CD(0 as libc::c_int as I, k, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_1: I = (*y).n;
        while i_0 < _i_1 {
            *(((*k).k).as_mut_ptr()
                as *mut I) = *(((*y).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            let ref mut fresh71 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh71 = _vs(x, k);
            if OOM_CD(0 as libc::c_int as I, z, k, *fresh71, -(1 as libc::c_int) as V)
                == 0
            {
                return 0 as K;
            }
            i_0 += 1;
            i_0;
        }
        cd(k);
        z = demote(z);
    } else if 1 as libc::c_int as libc::c_longlong == (*x).t {
        if *(((*x).k).as_mut_ptr() as *mut I) < 2 as libc::c_int as libc::c_longlong {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        if *(((*y).k).as_mut_ptr() as *mut I) == 0 as libc::c_int as libc::c_longlong {
            return X(b"!0\0" as *const u8 as *const libc::c_char as S);
        }
        z = newK(-(1 as libc::c_int) as I, 0 as libc::c_int as I);
        if z.is_null() {
            return 0 as K;
        }
        let mut a: I = *(((*x).k).as_mut_ptr() as *mut I);
        let mut b: I = *(((*y).k).as_mut_ptr() as *mut I);
        let mut c: I = b / a;
        while (*z).n == 0 || b != c {
            kap(&mut z, &mut b as *mut I as V);
            b = c;
            c = b / a;
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_2: I = (*z).n;
        while i_1 < _i_2 {
            let ref mut fresh72 = *(((*z).k).as_mut_ptr() as *mut I)
                .offset(i_1 as isize);
            *fresh72 %= a;
            i_1 += 1;
            i_1;
        }
        Ireverse(z);
    } else if -(1 as libc::c_int) as libc::c_longlong == (*x).t {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_3: I = (*x).n;
        while i_2 < _i_3 {
            if !(i_2 == 0 && *(((*x).k).as_mut_ptr() as *mut I) == 0) {
                if *(((*x).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                    < 1 as libc::c_int as libc::c_longlong
                {
                    return kerr(b"domain\0" as *const u8 as *const libc::c_char);
                }
            }
            i_2 += 1;
            i_2;
        }
        z = newK(-(1 as libc::c_int) as I, (*x).n);
        if z.is_null() {
            return 0 as K;
        }
        let mut a_0: I = *(((*y).k).as_mut_ptr() as *mut I);
        let mut n: I = (*z).n;
        if a_0 < 0 as libc::c_int as libc::c_longlong {
            let mut s: I = 1 as libc::c_int as I;
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_4: I = (*x).n;
            while i_3 < _i_4 {
                s
                    *= if *(((*x).k).as_mut_ptr() as *mut I).offset(i_3 as isize) != 0 {
                        *(((*x).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                    } else {
                        -(1 as libc::c_int) as libc::c_longlong
                    };
                i_3 += 1;
                i_3;
            }
            a_0 = s - -a_0 % s;
        }
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_5: I = n;
        while i_4 < _i_5 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_4 as isize,
                ) = *(((*x).k).as_mut_ptr() as *mut I)
                .offset(((*x).n - 1 as libc::c_int as libc::c_longlong - i_4) as isize);
            i_4 += 1;
            i_4;
        }
        if n != 0 {
            *(((*z).k).as_mut_ptr()
                as *mut I) = if *(((*z).k).as_mut_ptr() as *mut I) != 0 {
                a_0 / *(((*z).k).as_mut_ptr() as *mut I)
            } else {
                0 as libc::c_int as libc::c_longlong
            };
        }
        let mut i_5: I = 0 as libc::c_int as I;
        let mut _i_6: I = n - 1 as libc::c_int as libc::c_longlong;
        while i_5 < _i_6 {
            let mut d: I = *(((*z).k).as_mut_ptr() as *mut I)
                .offset((i_5 + 1 as libc::c_int as libc::c_longlong) as isize);
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    (i_5 + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = if d != 0 {
                *(((*z).k).as_mut_ptr() as *mut I).offset(i_5 as isize) / d
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            i_5 += 1;
            i_5;
        }
        let mut i_6: I = 0 as libc::c_int as I;
        let mut _i_7: I = n - 1 as libc::c_int as libc::c_longlong;
        while i_6 < _i_7 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    (n - 1 as libc::c_int as libc::c_longlong - i_6) as isize,
                ) = *(((*z).k).as_mut_ptr() as *mut I)
                .offset((n - 2 as libc::c_int as libc::c_longlong - i_6) as isize)
                - *(((*z).k).as_mut_ptr() as *mut I)
                    .offset((n - 1 as libc::c_int as libc::c_longlong - i_6) as isize)
                    * *(((*x).k).as_mut_ptr() as *mut I).offset(i_6 as isize);
            i_6 += 1;
            i_6;
        }
        if n != 0 {
            *(((*z).k).as_mut_ptr()
                as *mut I) = a_0
                - *(((*z).k).as_mut_ptr() as *mut I)
                    * *(((*x).k).as_mut_ptr() as *mut I)
                        .offset(
                            ((*x).n - 1 as libc::c_int as libc::c_longlong) as isize,
                        );
        }
        Ireverse(z);
        return z;
    }
    return z;
}
pub unsafe extern "C" fn _t() -> K {
    return Ki(time(0 as *mut time_t) as libc::c_longlong + k_epoch_offset);
}
pub unsafe extern "C" fn _T() -> K {
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tr: time_t = 0;
    let mut u: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    gettimeofday(&mut t, 0 as *mut libc::c_void);
    tr = t.tv_sec;
    gmtime_r(&mut tr, &mut u);
    return Kf(
        jdn_from_date(
            (1900 as libc::c_int + u.tm_year) as I,
            (1 as libc::c_int + u.tm_mon) as I,
            u.tm_mday as I,
        ) as libc::c_double
            + ((u.tm_hour * 60 as libc::c_int * 60 as libc::c_int
                + u.tm_min * 60 as libc::c_int + u.tm_sec) as libc::c_double
                + t.tv_usec as libc::c_double / 1.0e6f64) / 86400.0f64,
    );
}
pub unsafe extern "C" fn _n() -> K {
    return ci(NIL);
}
pub unsafe extern "C" fn _h() -> K {
    let mut c: [C; 256] = [0; 256];
    if gethostname(c.as_mut_ptr(), 256 as libc::c_int as size_t) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    return Ks(sp(c.as_mut_ptr()));
}
pub unsafe extern "C" fn _d() -> K {
    return Ks(d_);
}
pub unsafe extern "C" fn _v() -> K {
    return ci(KONA_GSET);
}
pub unsafe extern "C" fn _i() -> K {
    return ci(KONA_IDX);
}
pub unsafe extern "C" fn _f() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn _s() -> K {
    return mstat();
}
pub unsafe extern "C" fn _p() -> K {
    return ci(KONA_PORT);
}
pub unsafe extern "C" fn _w() -> K {
    return ci(KONA_WHO);
}
pub unsafe extern "C" fn _u() -> K {
    return kerr(b"nyi\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn _a() -> K {
    return ci(KONA_ARGS);
}
pub unsafe extern "C" fn _k() -> K {
    static mut x: K = 0 as *const k0 as K;
    if x.is_null() {
        static mut d: S = b"2025-07-11\0" as *const u8 as *const libc::c_char as S;
        x = newK(-(3 as libc::c_int) as I, strlen(d as *const libc::c_char) as I);
        if OOM_CD(0 as libc::c_int as I, x, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        strcpy(((*x).k).as_mut_ptr() as *mut C, d as *const libc::c_char);
    }
    return ci(x);
}
pub unsafe extern "C" fn _m() -> K {
    return kerr(b"nyi\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn _c() -> K {
    return ci(KONA_CLIENT);
}
unsafe extern "C" fn CIX(mut a: K, mut i: I, mut x: K) -> I {
    let mut at: I = (*a).t;
    let mut t: I = (*x).t;
    let mut r: I = 0 as libc::c_int as I;
    let mut k: K = 0 as K;
    if at == 0 {
        k = ci(*((*a).k).as_mut_ptr().offset(i as isize));
    } else {
        k = newK(-at, 1 as libc::c_int as I);
    }
    match (*k).t {
        1 => {
            *(((*k).k).as_mut_ptr()
                as *mut I) = *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize);
        }
        2 => {
            *(((*k).k).as_mut_ptr()
                as *mut F) = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize);
        }
        3 => {
            *(((*k).k).as_mut_ptr()
                as *mut C) = *(((*a).k).as_mut_ptr() as *mut C).offset(i as isize);
        }
        4 => {
            let ref mut fresh73 = *(((*k).k).as_mut_ptr() as *mut S);
            *fresh73 = *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize);
        }
        _ => {}
    }
    if 1 as libc::c_int as libc::c_longlong == (*k).t
        && 2 as libc::c_int as libc::c_longlong == t
    {
        r = FC(
            if 9223372036854775807 as libc::c_longlong
                == *(((*k).k).as_mut_ptr() as *mut I)
            {
                1 as libc::c_int as libc::c_double / 0.0f64
            } else if -(9223372036854775807 as libc::c_longlong)
                == *(((*k).k).as_mut_ptr() as *mut I)
            {
                -(1 as libc::c_int as libc::c_double / 0.0f64)
            } else if -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                == *(((*k).k).as_mut_ptr() as *mut I)
            {
                0 as libc::c_int as libc::c_double / 0.0f64
            } else {
                *(((*k).k).as_mut_ptr() as *mut I) as libc::c_double
            },
            *(((*x).k).as_mut_ptr() as *mut F),
        );
    } else if 2 as libc::c_int as libc::c_longlong == (*k).t
        && 1 as libc::c_int as libc::c_longlong == t
    {
        r = FC(
            *(((*k).k).as_mut_ptr() as *mut F),
            if 9223372036854775807 as libc::c_longlong
                == *(((*x).k).as_mut_ptr() as *mut I)
            {
                1 as libc::c_int as libc::c_double / 0.0f64
            } else if -(9223372036854775807 as libc::c_longlong)
                == *(((*x).k).as_mut_ptr() as *mut I)
            {
                -(1 as libc::c_int as libc::c_double / 0.0f64)
            } else if -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                == *(((*x).k).as_mut_ptr() as *mut I)
            {
                0 as libc::c_int as libc::c_double / 0.0f64
            } else {
                *(((*x).k).as_mut_ptr() as *mut I) as libc::c_double
            },
        );
    } else {
        r = KC(k, x);
    }
    if !k.is_null() {
        cd(k);
    }
    return r;
}
unsafe extern "C" fn binr(mut a: K, mut b: I, mut c: I, mut x: K) -> I {
    let mut i: I = b + (c - b) / 2 as libc::c_int as libc::c_longlong;
    let mut r: I = CIX(a, i, x);
    if 0 as libc::c_int as libc::c_longlong == r {
        if i > 0 as libc::c_int as libc::c_longlong
            && CIX(a, i - 1 as libc::c_int as libc::c_longlong, x) == 0
        {
            r = 1 as libc::c_int as I;
        } else {
            return i
        }
    }
    if b >= c {
        return if -(1 as libc::c_int) as libc::c_longlong == r {
            1 as libc::c_int as libc::c_longlong + i
        } else {
            i
        };
    }
    return if (0 as libc::c_int as libc::c_longlong) < r {
        binr(a, b, i - 1 as libc::c_int as libc::c_longlong, x)
    } else {
        binr(a, i + 1 as libc::c_int as libc::c_longlong, c, x)
    };
}
pub unsafe extern "C" fn RF() -> F {
    return genrand64_real2();
}
pub static mut k_epoch_offset: I = -(2051222400 as libc::c_long) as I;
pub unsafe extern "C" fn _dot_t() -> K {
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut t, 0 as *mut libc::c_void);
    return Kf(
        k_epoch_offset as libc::c_double + t.tv_sec as F + t.tv_usec as F / 1.0e6f64,
    );
}
unsafe extern "C" fn jdn_from_date(mut year: I, mut month: I, mut day: I) -> I {
    let mut a: I = (14 as libc::c_int as libc::c_longlong - month)
        / 12 as libc::c_int as libc::c_longlong;
    let mut y: I = year + 4800 as libc::c_int as libc::c_longlong - a;
    let mut m: I = month + 12 as libc::c_int as libc::c_longlong * a
        - 3 as libc::c_int as libc::c_longlong;
    return day
        + (153 as libc::c_int as libc::c_longlong * m
            + 2 as libc::c_int as libc::c_longlong)
            / 5 as libc::c_int as libc::c_longlong
        + y * 365 as libc::c_int as libc::c_longlong
        + y / 4 as libc::c_int as libc::c_longlong
        - y / 100 as libc::c_int as libc::c_longlong
        + y / 400 as libc::c_int as libc::c_longlong
        - 32045 as libc::c_int as libc::c_longlong
        - 2464329 as libc::c_int as libc::c_longlong;
}
unsafe extern "C" fn date_from_jdn(mut j: I) -> I {
    let mut b: I = 0;
    let mut c: I = 0;
    let mut d: I = 0;
    let mut e: I = 0;
    let mut m: I = 0;
    let mut year: I = 0;
    let mut month: I = 0;
    let mut day: I = 0;
    let mut a: I = j + 32044 as libc::c_int as libc::c_longlong
        + 2464329 as libc::c_int as libc::c_longlong;
    b = (4 as libc::c_int as libc::c_longlong * a + 3 as libc::c_int as libc::c_longlong)
        / 146097 as libc::c_int as libc::c_longlong;
    c = a
        - b * 146097 as libc::c_int as libc::c_longlong
            / 4 as libc::c_int as libc::c_longlong;
    d = (4 as libc::c_int as libc::c_longlong * c + 3 as libc::c_int as libc::c_longlong)
        / 1461 as libc::c_int as libc::c_longlong;
    e = c
        - 1461 as libc::c_int as libc::c_longlong * d
            / 4 as libc::c_int as libc::c_longlong;
    m = (5 as libc::c_int as libc::c_longlong * e + 2 as libc::c_int as libc::c_longlong)
        / 153 as libc::c_int as libc::c_longlong;
    day = e
        - (153 as libc::c_int as libc::c_longlong * m
            + 2 as libc::c_int as libc::c_longlong)
            / 5 as libc::c_int as libc::c_longlong
        + 1 as libc::c_int as libc::c_longlong;
    month = m + 3 as libc::c_int as libc::c_longlong
        - 12 as libc::c_int as libc::c_longlong
            * (m / 10 as libc::c_int as libc::c_longlong);
    year = b * 100 as libc::c_int as libc::c_longlong + d
        - 4800 as libc::c_int as libc::c_longlong
        + m / 10 as libc::c_int as libc::c_longlong;
    return year * 10000 as libc::c_int as libc::c_longlong
        + month * 100 as libc::c_int as libc::c_longlong + day;
}
