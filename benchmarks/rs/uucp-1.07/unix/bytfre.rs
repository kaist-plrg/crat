use ::libc;
extern "C" {
    fn get_fs_usage(
        path: *mut libc::c_char,
        disk: *mut libc::c_char,
        fsp: *mut fs_usage,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_usage {
    pub fsu_blocks: libc::c_long,
    pub fsu_bfree: libc::c_long,
    pub fsu_bavail: libc::c_long,
    pub fsu_files: libc::c_long,
    pub fsu_ffree: libc::c_long,
}
pub unsafe extern "C" fn csysdep_bytes_free(
    mut zfile: *const libc::c_char,
) -> libc::c_long {
    let mut s: fs_usage = fs_usage {
        fsu_blocks: 0,
        fsu_bfree: 0,
        fsu_bavail: 0,
        fsu_files: 0,
        fsu_ffree: 0,
    };
    if get_fs_usage(
        zfile as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        &mut s,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if s.fsu_bavail
        >= 9223372036854775807 as libc::c_long / 512 as libc::c_int as libc::c_long
    {
        return 9223372036854775807 as libc::c_long;
    }
    return s.fsu_bavail * 512 as libc::c_int as libc::c_long;
}
