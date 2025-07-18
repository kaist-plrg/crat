use ::libc;
pub type size_t = libc::c_ulong;
pub type UUCONF_POINTER = *mut libc::c_void;
pub type uuconf_cmdtabfn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::c_int,
        *mut *mut libc::c_char,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_cmdtab {
    pub uuconf_zcmd: *const libc::c_char,
    pub uuconf_itype: libc::c_int,
    pub uuconf_pvar: UUCONF_POINTER,
    pub uuconf_pifn: uuconf_cmdtabfn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdtab_offset {
    pub zcmd: *const libc::c_char,
    pub itype: libc::c_int,
    pub ioff: size_t,
    pub pifn: uuconf_cmdtabfn,
}
pub static mut _uuconf_base_rcsid: [libc::c_char; 48] = unsafe {
    *::std::mem::transmute::<
        &[u8; 48],
        &[libc::c_char; 48],
    >(b"$Id: base.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_ucmdtab_base(
    mut qoff: *const cmdtab_offset,
    mut celes: size_t,
    mut pbase: *mut libc::c_char,
    mut qset: *mut uuconf_cmdtab,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < celes {
        (*qset).uuconf_zcmd = (*qoff).zcmd;
        (*qset).uuconf_itype = (*qoff).itype;
        if (*qoff).ioff == -(1 as libc::c_int) as size_t {
            (*qset).uuconf_pvar = 0 as *mut libc::c_void;
        } else {
            (*qset).uuconf_pvar = pbase.offset((*qoff).ioff as isize) as UUCONF_POINTER;
        }
        (*qset).uuconf_pifn = (*qoff).pifn;
        i = i.wrapping_add(1);
        i;
        qoff = qoff.offset(1);
        qoff;
        qset = qset.offset(1);
        qset;
    }
}
