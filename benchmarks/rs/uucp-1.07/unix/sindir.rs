use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn zsysdep_in_dir(
    mut zdir: *const libc::c_char,
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut cdir: size_t = 0;
    let mut cfile: size_t = 0;
    let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
    cdir = strlen(zdir);
    cfile = strlen(zfile);
    zret = zbufalc(
        cdir.wrapping_add(cfile).wrapping_add(2 as libc::c_int as libc::c_ulong),
    );
    if cdir == 1 as libc::c_int as libc::c_ulong && *zdir as libc::c_int == '/' as i32 {
        cdir = 0 as libc::c_int as size_t;
    } else {
        memcpy(zret as *mut libc::c_void, zdir as *const libc::c_void, cdir);
    }
    memcpy(
        zret.offset(cdir as isize).offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        zfile as *const libc::c_void,
        cfile,
    );
    *zret.offset(cdir as isize) = '/' as i32 as libc::c_char;
    *zret
        .offset(
            cdir.wrapping_add(cfile).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as isize,
        ) = '\0' as i32 as libc::c_char;
    return zret;
}
