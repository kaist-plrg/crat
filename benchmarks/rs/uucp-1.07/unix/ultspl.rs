use ::libc;
extern "C" {
    fn ubuffree(z: *mut libc::c_char);
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
}
pub type boolean = libc::c_int;
pub unsafe extern "C" fn fsultrix_has_spool(
    mut zsystem: *const libc::c_char,
) -> boolean {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    z = zsysdep_in_dir(b"sys\0" as *const u8 as *const libc::c_char, zsystem);
    fret = fsysdep_directory(z);
    ubuffree(z);
    return fret;
}
