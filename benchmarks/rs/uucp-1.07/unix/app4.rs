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
pub unsafe extern "C" fn zsappend4(
    mut zdir1: *const libc::c_char,
    mut zdir2: *const libc::c_char,
    mut zdir3: *const libc::c_char,
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut cdir1: size_t = 0;
    let mut cdir2: size_t = 0;
    let mut cdir3: size_t = 0;
    let mut cfile: size_t = 0;
    let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
    cdir1 = strlen(zdir1);
    cdir2 = strlen(zdir2);
    cdir3 = strlen(zdir3);
    cfile = strlen(zfile);
    zret = zbufalc(
        cdir1
            .wrapping_add(cdir2)
            .wrapping_add(cdir3)
            .wrapping_add(cfile)
            .wrapping_add(4 as libc::c_int as libc::c_ulong),
    );
    if cdir1 == 1 as libc::c_int as libc::c_ulong && *zdir1 as libc::c_int == '/' as i32
    {
        cdir1 = 0 as libc::c_int as size_t;
    } else {
        memcpy(zret as *mut libc::c_void, zdir1 as *const libc::c_void, cdir1);
    }
    memcpy(
        zret.offset(cdir1 as isize).offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        zdir2 as *const libc::c_void,
        cdir2,
    );
    memcpy(
        zret
            .offset(cdir1 as isize)
            .offset(cdir2 as isize)
            .offset(2 as libc::c_int as isize) as *mut libc::c_void,
        zdir3 as *const libc::c_void,
        cdir3,
    );
    memcpy(
        zret
            .offset(cdir1 as isize)
            .offset(cdir2 as isize)
            .offset(cdir3 as isize)
            .offset(3 as libc::c_int as isize) as *mut libc::c_void,
        zfile as *const libc::c_void,
        cfile,
    );
    *zret.offset(cdir1 as isize) = '/' as i32 as libc::c_char;
    *zret
        .offset(
            cdir1.wrapping_add(cdir2).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as isize,
        ) = '/' as i32 as libc::c_char;
    *zret
        .offset(
            cdir1
                .wrapping_add(cdir2)
                .wrapping_add(cdir3)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = '/' as i32 as libc::c_char;
    *zret
        .offset(
            cdir1
                .wrapping_add(cdir2)
                .wrapping_add(cdir3)
                .wrapping_add(cfile)
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    return zret;
}
