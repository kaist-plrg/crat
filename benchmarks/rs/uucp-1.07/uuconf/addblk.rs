use ::libc;
extern "C" {
    fn uuconf_malloc(
        uuconf_pblock: *mut libc::c_void,
        uuconf_cbytes: UUCONF_SIZE_T,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
pub type UUCONF_SIZE_T = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sadded {
    pub qnext: *mut sadded,
    pub padded: pointer,
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
pub union C2RustUnnamed {
    pub ab: [libc::c_char; 1008],
    pub l: libc::c_double,
}
pub static mut _uuconf_addblk_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: addblk.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_add_block(
    mut pblock: pointer,
    mut padd: pointer,
) -> libc::c_int {
    let mut q: *mut sblock = pblock as *mut sblock;
    let mut qnew: *mut sadded = 0 as *mut sadded;
    qnew = uuconf_malloc(pblock, ::std::mem::size_of::<sadded>() as libc::c_ulong)
        as *mut sadded;
    if qnew.is_null() {
        return 1 as libc::c_int;
    }
    (*qnew).qnext = (*q).qadded;
    (*qnew).padded = padd;
    (*q).qadded = qnew;
    return 0 as libc::c_int;
}
