use ::libc;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn nettle_cnd_memcpy(
    mut cnd: libc::c_int,
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: size_t,
) {
    let mut sp: *const libc::c_uchar = src as *const libc::c_uchar;
    let mut dp: *mut libc::c_uchar = dst as *mut libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut m: libc::c_uchar = 0;
    let mut i: size_t = 0;
    ::std::ptr::write_volatile(
        &mut m as *mut libc::c_uchar,
        -(cnd as libc::c_uchar as libc::c_int) as libc::c_uchar,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        ::std::ptr::write_volatile(
            &mut c as *mut libc::c_uchar,
            (*sp.offset(i as isize) as libc::c_int & m as libc::c_int) as libc::c_uchar,
        );
        ::std::ptr::write_volatile(
            &mut c as *mut libc::c_uchar,
            (::std::ptr::read_volatile::<libc::c_uchar>(&c as *const libc::c_uchar)
                as libc::c_int
                | *dp.offset(i as isize) as libc::c_int & !(m as libc::c_int))
                as libc::c_uchar as libc::c_uchar,
        );
        ::std::ptr::write_volatile(dp.offset(i as isize), c);
        i = i.wrapping_add(1);
        i;
    }
}
