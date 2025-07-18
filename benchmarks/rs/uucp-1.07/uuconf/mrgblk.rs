use ::libc;
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sadded {
    pub qnext: *mut sadded,
    pub padded: pointer,
}
pub static mut _uuconf_mrgblk_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: mrgblk.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_pmalloc_block_merge(
    mut p1: pointer,
    mut p2: pointer,
) -> pointer {
    let mut q1: *mut sblock = p1 as *mut sblock;
    let mut q2: *mut sblock = p2 as *mut sblock;
    let mut pq: *mut *mut sblock = 0 as *mut *mut sblock;
    pq = &mut q1;
    while !(*pq).is_null() {
        pq = &mut (**pq).qnext;
    }
    *pq = q2;
    return q1 as pointer;
}
