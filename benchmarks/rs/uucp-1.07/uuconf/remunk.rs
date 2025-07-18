use ::libc;
pub type pointer = *mut libc::c_void;
pub static mut _uuconf_remunk_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: remunk.c,v 1.8 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_remote_unknown(
    mut pglobal: pointer,
    mut pzname: *mut *mut libc::c_char,
) -> libc::c_int {
    return 1 as libc::c_int;
}
