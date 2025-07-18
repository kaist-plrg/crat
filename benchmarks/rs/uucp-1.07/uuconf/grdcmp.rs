use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub static mut _uuconf_grdcmp_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: grdcmp.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_grade_cmp(
    mut barg1: libc::c_int,
    mut barg2: libc::c_int,
) -> libc::c_int {
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    b1 = barg1 as libc::c_uchar as libc::c_int;
    b2 = barg2 as libc::c_uchar as libc::c_int;
    if *(*__ctype_b_loc()).offset(b1 as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if *(*__ctype_b_loc()).offset(b2 as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            return b1 - b2
        } else {
            return -(1 as libc::c_int)
        }
    } else if *(*__ctype_b_loc()).offset(b1 as isize) as libc::c_int
        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if *(*__ctype_b_loc()).offset(b2 as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            return 1 as libc::c_int
        } else if *(*__ctype_b_loc()).offset(b2 as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            return b1 - b2
        } else {
            return -(1 as libc::c_int)
        }
    } else if *(*__ctype_b_loc()).offset(b2 as isize) as libc::c_int
        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 1 as libc::c_int
    } else {
        return b1 - b2
    };
}
