use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ab: [libc::c_char; 4],
    pub bdummy: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbuf {
    pub qnext: *mut sbuf,
    pub c: size_t,
    pub u: C2RustUnnamed,
}
static mut qBlist: *mut sbuf = 0 as *const sbuf as *mut sbuf;
pub unsafe extern "C" fn zbufalc(mut c: size_t) -> *mut libc::c_char {
    let mut q: *mut sbuf = 0 as *mut sbuf;
    if qBlist.is_null() {
        q = xmalloc(
            (::std::mem::size_of::<sbuf>() as libc::c_ulong)
                .wrapping_add(c)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong),
        ) as *mut sbuf;
        (*q).c = c;
    } else {
        q = qBlist;
        qBlist = (*q).qnext;
        if (*q).c < c {
            q = xrealloc(
                q as pointer,
                (::std::mem::size_of::<sbuf>() as libc::c_ulong)
                    .wrapping_add(c)
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong),
            ) as *mut sbuf;
            (*q).c = c;
        }
    }
    return ((*q).u.ab).as_mut_ptr();
}
pub unsafe extern "C" fn ubuffree(mut z: *mut libc::c_char) {
    let mut q: *mut sbuf = 0 as *mut sbuf;
    let mut ioff: libc::c_int = 0;
    if z.is_null() {
        return;
    }
    ioff = 16 as libc::c_ulong as libc::c_int;
    q = z.offset(-(ioff as isize)) as pointer as *mut sbuf;
    (*q).qnext = qBlist;
    qBlist = q;
}
pub unsafe extern "C" fn zbufcpy(mut z: *const libc::c_char) -> *mut libc::c_char {
    let mut csize: size_t = 0;
    let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
    if z.is_null() {
        return 0 as *mut libc::c_char;
    }
    csize = (strlen(z)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    zret = zbufalc(csize);
    memcpy(zret as *mut libc::c_void, z as *const libc::c_void, csize);
    return zret;
}
