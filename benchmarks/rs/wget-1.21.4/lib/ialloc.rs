use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn ireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    if n == 0 as libc::c_int as libc::c_long || s == 0 as libc::c_int as libc::c_long {
        s = 1 as libc::c_int as idx_t;
        n = s;
    }
    return if n as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
        && s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
    {
        reallocarray(p, n as size_t, s as size_t)
    } else {
        _gl_alloc_nomem()
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn icalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    if (18446744073709551615 as libc::c_ulong) < n as libc::c_ulong {
        if s != 0 as libc::c_int as libc::c_long {
            return _gl_alloc_nomem();
        }
        n = 0 as libc::c_int as idx_t;
    }
    if (18446744073709551615 as libc::c_ulong) < s as libc::c_ulong {
        if n != 0 as libc::c_int as libc::c_long {
            return _gl_alloc_nomem();
        }
        s = 0 as libc::c_int as idx_t;
    }
    return calloc(n as libc::c_ulong, s as libc::c_ulong);
}
#[cold]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_void;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn imalloc(mut s: idx_t) -> *mut libc::c_void {
    return if s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong {
        malloc(s as libc::c_ulong)
    } else {
        _gl_alloc_nomem()
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn irealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return if s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong {
        realloc(p, (s | (s == 0) as libc::c_int as libc::c_long) as libc::c_ulong)
    } else {
        _gl_alloc_nomem()
    };
}
