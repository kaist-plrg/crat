use ::libc;
extern "C" {
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn ftw(
        __dir: *const libc::c_char,
        __func: __ftw_func_t,
        __descriptors: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdirlist {
    pub qnext: *mut sdirlist,
    pub zdir: *mut libc::c_char,
}
pub const FTW_DNR: C2RustUnnamed = 2;
pub const FTW_D: C2RustUnnamed = 1;
pub type __ftw_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, *const stat, libc::c_int) -> libc::c_int,
>;
pub type C2RustUnnamed = libc::c_uint;
pub const FTW_SL: C2RustUnnamed = 4;
pub const FTW_NS: C2RustUnnamed = 3;
pub const FTW_F: C2RustUnnamed = 0;
static mut qSdirlist: *mut sdirlist = 0 as *const sdirlist as *mut sdirlist;
pub unsafe extern "C" fn fsysdep_rmdir(mut zdir: *const libc::c_char) -> boolean {
    let mut fret: boolean = 0;
    let mut q: *mut sdirlist = 0 as *mut sdirlist;
    qSdirlist = 0 as *mut sdirlist;
    fret = 1 as libc::c_int;
    if ftw(
        zdir as *mut libc::c_char,
        Some(
            isremove_dir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const stat,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        5 as libc::c_int,
    ) != 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"ftw: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fret = 0 as libc::c_int;
    }
    q = qSdirlist;
    while !q.is_null() {
        let mut qnext: *mut sdirlist = 0 as *mut sdirlist;
        if rmdir((*q).zdir) != 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"rmdir (%s): %s\0" as *const u8 as *const libc::c_char,
                (*q).zdir,
                strerror(*__errno_location()),
            );
            fret = 0 as libc::c_int;
        }
        ubuffree((*q).zdir);
        qnext = (*q).qnext;
        xfree(q as pointer);
        q = qnext;
    }
    return fret;
}
unsafe extern "C" fn isremove_dir(
    mut zfile: *const libc::c_char,
    mut qstat: *const stat,
    mut iflag: libc::c_int,
) -> libc::c_int {
    if iflag == FTW_D as libc::c_int || iflag == FTW_DNR as libc::c_int {
        let mut q: *mut sdirlist = 0 as *mut sdirlist;
        q = xmalloc(::std::mem::size_of::<sdirlist>() as libc::c_ulong) as *mut sdirlist;
        (*q).qnext = qSdirlist;
        (*q).zdir = zbufcpy(zfile);
        qSdirlist = q;
    } else if remove(zfile) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
            zfile,
            strerror(*__errno_location()),
        );
    }
    return 0 as libc::c_int;
}
