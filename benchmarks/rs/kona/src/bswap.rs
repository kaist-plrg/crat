use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong)
        >> 56 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
            >> 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
            >> 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
            >> 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
            << 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
            << 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56 as libc::c_int)
        as __uint64_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
pub unsafe extern "C" fn membswp32(mut d: V, mut s: V, mut n: I) -> V {
    let mut q: *mut uint32_t = d as *mut uint32_t;
    let mut p: *mut uint32_t = s as *mut uint32_t;
    let mut i: I = 0;
    i = 0 as libc::c_int as I;
    while i < n {
        let fresh0 = p;
        p = p.offset(1);
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 = __bswap_32(*fresh0);
        i = i + 4 as libc::c_int as libc::c_longlong;
    }
    return d;
}
pub unsafe extern "C" fn membswp64(mut d: V, mut s: V, mut n: I) -> V {
    let mut q: *mut uint64_t = d as *mut uint64_t;
    let mut p: *mut uint64_t = s as *mut uint64_t;
    let mut i: I = 0;
    i = 0 as libc::c_int as I;
    while i < n {
        let fresh2 = p;
        p = p.offset(1);
        let fresh3 = q;
        q = q.offset(1);
        *fresh3 = __bswap_64(*fresh2);
        i = i + 8 as libc::c_int as libc::c_longlong;
    }
    return d;
}
pub unsafe extern "C" fn bswapI(mut n: I) -> I {
    return __bswap_64(n as __uint64_t) as I;
}
pub unsafe extern "C" fn membswpI(mut d: V, mut s: V, mut n: I, mut x: I) -> V {
    if x != 0 {
        return membswp64(d, s, n);
    }
    return memcpy(d, s as *const libc::c_void, n as libc::c_ulong);
}
pub unsafe extern "C" fn membswpF(mut d: V, mut s: V, mut n: I, mut x: I) -> V {
    return if x != 0 {
        membswp64(d, s, n)
    } else {
        memcpy(d, s as *const libc::c_void, n as libc::c_ulong)
    };
}
