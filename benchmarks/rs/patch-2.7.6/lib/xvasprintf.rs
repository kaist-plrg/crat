use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as libc::c_ulong };
}
unsafe extern "C" fn xstrcat(
    mut argcount: size_t,
    mut args: ::std::ffi::VaList,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut totalsize: size_t = 0;
    let mut i: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    totalsize = 0 as libc::c_int as size_t;
    ap = args.clone();
    i = argcount;
    while i > 0 as libc::c_int as libc::c_ulong {
        let mut next: *const libc::c_char = ap.arg::<*const libc::c_char>();
        totalsize = xsum(totalsize, strlen(next));
        i = i.wrapping_sub(1);
        i;
    }
    if totalsize == 18446744073709551615 as libc::c_ulong
        || totalsize > 2147483647 as libc::c_int as libc::c_ulong
    {
        *__errno_location() = 75 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    result = (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xmalloc(totalsize.wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        xnmalloc(
            totalsize.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    p = result;
    i = argcount;
    while i > 0 as libc::c_int as libc::c_ulong {
        let mut next_0: *const libc::c_char = args.arg::<*const libc::c_char>();
        let mut len: size_t = strlen(next_0);
        memcpy(p as *mut libc::c_void, next_0 as *const libc::c_void, len);
        p = p.offset(len as isize);
        i = i.wrapping_sub(1);
        i;
    }
    *p = '\0' as i32 as libc::c_char;
    return result;
}
pub unsafe extern "C" fn xvasprintf(
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argcount: size_t = 0 as libc::c_int as size_t;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    f = format;
    loop {
        if *f as libc::c_int == '\0' as i32 {
            return xstrcat(argcount, args.as_va_list());
        }
        if *f as libc::c_int != '%' as i32 {
            break;
        }
        f = f.offset(1);
        f;
        if *f as libc::c_int != 's' as i32 {
            break;
        }
        f = f.offset(1);
        f;
        argcount = argcount.wrapping_add(1);
        argcount;
    }
    if vasprintf(&mut result, format, args.as_va_list()) < 0 as libc::c_int {
        if *__errno_location() == 12 as libc::c_int {
            xalloc_die();
        }
        return 0 as *mut libc::c_char;
    }
    return result;
}
