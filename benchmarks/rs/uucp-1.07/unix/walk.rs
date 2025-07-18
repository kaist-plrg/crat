use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
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
pub const FTW_F: C2RustUnnamed = 0;
pub type __ftw_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, *const stat, libc::c_int) -> libc::c_int,
>;
pub type C2RustUnnamed = libc::c_uint;
pub const FTW_SL: C2RustUnnamed = 4;
pub const FTW_NS: C2RustUnnamed = 3;
pub const FTW_DNR: C2RustUnnamed = 2;
pub const FTW_D: C2RustUnnamed = 1;
static mut cSlen: size_t = 0;
static mut puSfn: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, pointer) -> (),
> = None;
static mut pSinfo: pointer = 0 as *const libc::c_void as *mut libc::c_void;
pub unsafe extern "C" fn usysdep_walk_tree(
    mut zdir: *const libc::c_char,
    mut pufn: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, pointer) -> (),
    >,
    mut pinfo: pointer,
) -> boolean {
    cSlen = (strlen(zdir)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    puSfn = pufn;
    pSinfo = pinfo;
    return (ftw(
        zdir as *mut libc::c_char,
        Some(
            iswalk_dir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const stat,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        5 as libc::c_int,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn iswalk_dir(
    mut zname: *const libc::c_char,
    mut qstat: *const stat,
    mut iflag: libc::c_int,
) -> libc::c_int {
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    if iflag != FTW_F as libc::c_int {
        return 0 as libc::c_int;
    }
    zcopy = zbufcpy(zname.offset(cSlen as isize));
    (Some(puSfn.unwrap())).unwrap()(zname, zcopy, pSinfo);
    ubuffree(zcopy);
    return 0 as libc::c_int;
}
