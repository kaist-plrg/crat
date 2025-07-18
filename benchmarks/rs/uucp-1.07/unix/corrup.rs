use ::libc;
extern "C" {
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
pub unsafe extern "C" fn zsysdep_save_corrupt_file(
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut zslash: *const libc::c_char = 0 as *const libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    zslash = strrchr(zfile, '/' as i32);
    if zslash.is_null() {
        zslash = zfile;
    } else {
        zslash = zslash.offset(1);
        zslash;
    }
    zto = zsappend3(
        zSspooldir,
        b".Corrupt\0" as *const u8 as *const libc::c_char,
        zslash,
    );
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
