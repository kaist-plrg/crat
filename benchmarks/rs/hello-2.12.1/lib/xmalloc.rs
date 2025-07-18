use ::libc;
extern "C" {
    fn xalloc_die();
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub const HAVE_GNU_MALLOC: C2RustUnnamed_1 = 1;
pub const HAVE_GNU_CALLOC: C2RustUnnamed_0 = 1;
pub const HAVE_GNU_REALLOC: C2RustUnnamed_2 = 1;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(n);
    if p.is_null() && (HAVE_GNU_MALLOC as libc::c_int != 0 || n != 0) {
        xalloc_die();
    }
    return p;
}
pub unsafe extern "C" fn xrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    if HAVE_GNU_REALLOC as libc::c_int == 0 && n == 0 && !p.is_null() {
        free(p);
        return 0 as *mut libc::c_void;
    }
    let mut r: *mut libc::c_void = realloc(p, n);
    if r.is_null() && (n != 0 || HAVE_GNU_REALLOC as libc::c_int != 0 && p.is_null()) {
        xalloc_die();
    }
    return r;
}
pub unsafe extern "C" fn x2realloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
) -> *mut libc::c_void {
    return x2nrealloc(p, pn, 1 as libc::c_int as size_t);
}
pub unsafe extern "C" fn xzalloc(mut n: size_t) -> *mut libc::c_void {
    return xcalloc(n, 1 as libc::c_int as size_t);
}
pub unsafe extern "C" fn xcalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(s) < n
        || {
            p = calloc(n, s);
            p.is_null()
                && (HAVE_GNU_CALLOC as libc::c_int != 0
                    || n != 0 as libc::c_int as libc::c_ulong)
        }
    {
        xalloc_die();
    }
    return p;
}
pub unsafe extern "C" fn xmemdup(
    mut p: *const libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    return memcpy(xmalloc(s), p, s);
}
pub unsafe extern "C" fn xstrdup(mut string: *const libc::c_char) -> *mut libc::c_char {
    return xmemdup(
        string as *const libc::c_void,
        (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
