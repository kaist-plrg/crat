use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
}
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
pub static mut _uuconf_freblk_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: freblk.c,v 1.8 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_free_block(mut pblock: pointer) {
    let mut q: *mut sblock = pblock as *mut sblock;
    let mut qloop: *mut sblock = 0 as *mut sblock;
    qloop = q;
    while !qloop.is_null() {
        let mut qadd: *mut sadded = 0 as *mut sadded;
        qadd = (*qloop).qadded;
        while !qadd.is_null() {
            free((*qadd).padded);
            qadd = (*qadd).qnext;
        }
        qloop = (*qloop).qnext;
    }
    while !q.is_null() {
        let mut qnext: *mut sblock = 0 as *mut sblock;
        qnext = (*q).qnext;
        free(q as pointer);
        q = qnext;
    }
}
