use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn out_of_memory();
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut libc::c_uchar,
    pub end: libc::c_int,
    pub size: libc::c_int,
}
pub unsafe extern "C" fn xbuf_init(mut xbuf: *mut xbuffer) {
    (*xbuf).data = 0 as *mut libc::c_uchar;
    (*xbuf).end = 0 as libc::c_int;
    (*xbuf).size = (*xbuf).end;
}
pub unsafe extern "C" fn xbuf_deinit(mut xbuf: *mut xbuffer) {
    if !((*xbuf).data).is_null() {
        free((*xbuf).data as *mut libc::c_void);
    }
    xbuf_init(xbuf);
}
pub unsafe extern "C" fn xbuf_reset(mut xbuf: *mut xbuffer) {
    (*xbuf).end = 0 as libc::c_int;
}
pub unsafe extern "C" fn xbuf_add_byte(mut xbuf: *mut xbuffer, mut b: libc::c_uchar) {
    if (*xbuf).end >= (*xbuf).size {
        let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if help_ckd_add(
            &mut (*xbuf).size as *mut libc::c_int as *mut libc::c_void,
            (*xbuf).size as uintmax,
            (if (*xbuf).size != 0 { (*xbuf).size } else { 16 as libc::c_int })
                as uintmax,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { (*xbuf).size })
                - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
        ) != 0
        {
            out_of_memory();
        }
        data = ecalloc(
            (*xbuf).size,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as libc::c_uint,
        ) as *mut libc::c_uchar;
        if !((*xbuf).data).is_null() {
            memcpy(
                data as *mut libc::c_void,
                (*xbuf).data as *const libc::c_void,
                (*xbuf).end as libc::c_ulong,
            );
            free((*xbuf).data as *mut libc::c_void);
        }
        (*xbuf).data = data;
    }
    let fresh0 = (*xbuf).end;
    (*xbuf).end = (*xbuf).end + 1;
    *((*xbuf).data).offset(fresh0 as isize) = b;
}
pub unsafe extern "C" fn xbuf_add_data(
    mut xbuf: *mut xbuffer,
    mut data: *mut libc::c_uchar,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        xbuf_add_byte(xbuf, *data.offset(i as isize));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn xbuf_pop(mut buf: *mut xbuffer) -> libc::c_int {
    if (*buf).end == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*buf).end -= 1;
    return *((*buf).data).offset((*buf).end as isize) as libc::c_int;
}
pub unsafe extern "C" fn xbuf_set(mut dst: *mut xbuffer, mut src: *mut xbuffer) {
    xbuf_reset(dst);
    xbuf_add_data(dst, (*src).data, (*src).end);
}
pub unsafe extern "C" fn xbuf_char_data(mut xbuf: *mut xbuffer) -> *mut libc::c_char {
    return (*xbuf).data as *mut libc::c_char;
}
unsafe extern "C" fn help_fixup(
    mut r: *mut libc::c_void,
    mut val: uintmax,
    mut rsize: libc::c_int,
    mut rsigned: libc::c_int,
) -> libc::c_int {
    if rsigned != 0 {
        if rsize as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        {
            let mut pr: *mut libc::c_int = r as *mut libc::c_int;
            if (2147483647 as libc::c_int as libc::c_ulong) < val {
                return 1 as libc::c_int;
            }
            *pr = val as libc::c_int;
        } else if rsize as libc::c_ulong
            == ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
        {
            let mut pr_0: *mut libc::c_longlong = r as *mut libc::c_longlong;
            if (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                < val as libc::c_ulonglong
            {
                return 1 as libc::c_int;
            }
            *pr_0 = val as libc::c_longlong;
        } else if rsize as libc::c_ulong
            == ::std::mem::size_of::<intmax_t>() as libc::c_ulong
        {
            let mut pr_1: *mut intmax_t = r as *mut intmax_t;
            if (9223372036854775807 as libc::c_long as libc::c_ulong) < val {
                return 1 as libc::c_int;
            }
            *pr_1 = val as intmax_t;
        } else {
            let mut pr_2: *mut libc::c_long = r as *mut libc::c_long;
            if (9223372036854775807 as libc::c_long as libc::c_ulong) < val {
                return 1 as libc::c_int;
            }
            *pr_2 = val as libc::c_long;
        }
    } else if rsize as libc::c_ulong
        == ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
    {
        let mut pr_3: *mut libc::c_uint = r as *mut libc::c_uint;
        if ((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong) < val
        {
            return 1 as libc::c_int;
        }
        *pr_3 = val as libc::c_uint;
    } else if rsize as libc::c_ulong
        == ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
    {
        let mut pr_4: *mut libc::c_ulong = r as *mut libc::c_ulong;
        if (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong) < val
        {
            return 1 as libc::c_int;
        }
        *pr_4 = val;
    } else if rsize as libc::c_ulong
        == ::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong
    {
        let mut pr_5: *mut libc::c_longlong = r as *mut libc::c_longlong;
        if (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong) < val as libc::c_ulonglong
        {
            return 1 as libc::c_int;
        }
        *pr_5 = val as libc::c_longlong;
    } else {
        let mut pr_6: *mut uintmax = r as *mut uintmax;
        *pr_6 = val;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn help_ckd_add(
    mut r: *mut libc::c_void,
    mut a: uintmax,
    mut b: uintmax,
    mut rsize: libc::c_int,
    mut rsigned: libc::c_int,
) -> libc::c_int {
    let mut sum: uintmax = a.wrapping_add(b);
    return (sum < a || help_fixup(r, sum, rsize, rsigned) != 0) as libc::c_int;
}
pub unsafe extern "C" fn help_ckd_mul(
    mut r: *mut libc::c_void,
    mut a: uintmax,
    mut b: uintmax,
    mut rsize: libc::c_int,
    mut rsigned: libc::c_int,
) -> libc::c_int {
    let mut product: uintmax = a.wrapping_mul(b);
    return (b != 0 as libc::c_int as libc::c_ulong && a != product.wrapping_div(b)
        || help_fixup(r, product, rsize, rsigned) != 0) as libc::c_int;
}
