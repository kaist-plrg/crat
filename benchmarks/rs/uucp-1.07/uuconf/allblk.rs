use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub static mut _uuconf_allblk_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: allblk.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_malloc_block() -> *mut libc::c_void {
    let mut qret: *mut sblock = 0 as *mut sblock;
    qret = malloc(::std::mem::size_of::<sblock>() as libc::c_ulong) as *mut sblock;
    if qret.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*qret).qnext = 0 as *mut sblock;
    (*qret).ifree = 0 as libc::c_int as size_t;
    (*qret).plast = 0 as *mut libc::c_void;
    (*qret).qadded = 0 as *mut sadded;
    return qret as pointer;
}
