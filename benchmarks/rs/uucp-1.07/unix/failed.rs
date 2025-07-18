use ::libc;
extern "C" {
    fn zsappend3(
        zdir1: *const libc::c_char,
        zdir2: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut zSspooldir: *const libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn fsysdep_move_file(
        zorig: *const libc::c_char,
        zto: *const libc::c_char,
        fmkdirs: boolean,
        fpublic: boolean,
        fcheck: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
}
pub type boolean = libc::c_int;
pub unsafe extern "C" fn zsysdep_save_failed_file(
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    zto = zsappend3(zSspooldir, b".Failed\0" as *const u8 as *const libc::c_char, zfile);
    if fsysdep_move_file(
        zfile,
        zto,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
    ) == 0
    {
        ubuffree(zto);
        return 0 as *mut libc::c_char;
    }
    return zto;
}
