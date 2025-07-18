use ::libc;
extern "C" {
    fn abort() -> !;
    fn statvfs(__file: *const libc::c_char, __buf: *mut statvfs) -> libc::c_int;
}
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_usage {
    pub fsu_blocks: libc::c_long,
    pub fsu_bfree: libc::c_long,
    pub fsu_bavail: libc::c_long,
    pub fsu_files: libc::c_long,
    pub fsu_ffree: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
unsafe extern "C" fn adjust_blocks(
    mut blocks: libc::c_long,
    mut fromsize: libc::c_int,
    mut tosize: libc::c_int,
) -> libc::c_long {
    if tosize <= 0 as libc::c_int {
        abort();
    }
    if fromsize <= 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if fromsize == tosize {
        return blocks
    } else if fromsize > tosize {
        return blocks * (fromsize / tosize) as libc::c_long
    } else {
        return (blocks
            + (if blocks < 0 as libc::c_int as libc::c_long {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }) as libc::c_long) / (tosize / fromsize) as libc::c_long
    };
}
pub unsafe extern "C" fn get_fs_usage(
    mut path: *mut libc::c_char,
    mut disk: *mut libc::c_char,
    mut fsp: *mut fs_usage,
) -> libc::c_int {
    let mut fsd: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        __f_spare: [0; 6],
    };
    if statvfs(path, &mut fsd) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*fsp)
        .fsu_blocks = adjust_blocks(
        fsd.f_blocks as libc::c_long,
        (if fsd.f_frsize != 0 { fsd.f_frsize } else { fsd.f_bsize }) as libc::c_int,
        512 as libc::c_int,
    );
    (*fsp)
        .fsu_bfree = adjust_blocks(
        fsd.f_bfree as libc::c_long,
        (if fsd.f_frsize != 0 { fsd.f_frsize } else { fsd.f_bsize }) as libc::c_int,
        512 as libc::c_int,
    );
    (*fsp)
        .fsu_bavail = adjust_blocks(
        fsd.f_bavail as libc::c_long,
        (if fsd.f_frsize != 0 { fsd.f_frsize } else { fsd.f_bsize }) as libc::c_int,
        512 as libc::c_int,
    );
    (*fsp).fsu_files = fsd.f_files as libc::c_long;
    (*fsp).fsu_ffree = fsd.f_ffree as libc::c_long;
    return 0 as libc::c_int;
}
