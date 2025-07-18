use ::libc;
extern "C" {
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
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
    return xmalloc(n.wrapping_mul(s));
}
#[inline]
unsafe extern "C" fn xcharalloc(mut n: size_t) -> *mut libc::c_char {
    return (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xmalloc(n)
    } else {
        xnmalloc(n, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn xmemdup0(
    mut p: *const libc::c_void,
    mut s: size_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = xcharalloc(
        s.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(result as *mut libc::c_void, p, s);
    *result.offset(s as isize) = 0 as libc::c_int as libc::c_char;
    return result;
}
