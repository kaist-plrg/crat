use ::libc;
extern "C" {
    fn uuconf_grade_cmp(uuconf_b1: libc::c_int, uuconf_b2: libc::c_int) -> libc::c_int;
}
pub static mut _uuconf_tgcmp_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: tgcmp.c,v 1.6 2002/03/05 19:10:43 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_itime_grade_cmp(
    mut i1: libc::c_long,
    mut i2: libc::c_long,
) -> libc::c_int {
    return uuconf_grade_cmp(i1 as libc::c_int, i2 as libc::c_int);
}
