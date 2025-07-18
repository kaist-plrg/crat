use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ab: [libc::c_char; 1008],
    pub l: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblock {
    pub qnext: *mut sblock,
    pub ifree: size_t,
    pub plast: pointer,
    pub qadded: *mut sadded,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sadded {
    pub qnext: *mut sadded,
    pub padded: pointer,
}
pub static mut _uuconf_free_rcsid: [libc::c_char; 48] = unsafe {
    *::std::mem::transmute::<
        &[u8; 48],
        &[libc::c_char; 48],
    >(b"$Id: free.c,v 1.7 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_free(mut pblock: pointer, mut pbuf: pointer) {
    let mut q: *mut sblock = pblock as *mut sblock;
    if pbuf.is_null() {
        return;
    }
    if q.is_null() {
        free(pbuf);
        return;
    }
    while !q.is_null() {
        if (*q).plast == pbuf {
            (*q)
                .ifree = (pbuf as *mut libc::c_char)
                .offset_from(((*q).u.ab).as_mut_ptr()) as libc::c_long as size_t;
            return;
        }
        q = (*q).qnext;
    }
}
