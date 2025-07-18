use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: libc::c_uint,
    pub c_dev_min: libc::c_uint,
    pub c_rdev_maj: libc::c_uint,
    pub c_rdev_min: libc::c_uint,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut libc::c_char,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferment {
    pub next: *mut deferment,
    pub header: cpio_file_stat,
}
pub unsafe extern "C" fn create_deferment(
    mut file_hdr: *mut cpio_file_stat,
) -> *mut deferment {
    let mut d: *mut deferment = 0 as *mut deferment;
    d = xmalloc(::std::mem::size_of::<deferment>() as libc::c_ulong) as *mut deferment;
    (*d).header = *file_hdr;
    (*d)
        .header
        .c_name = xmalloc(
        (strlen((*file_hdr).c_name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy((*d).header.c_name, (*file_hdr).c_name);
    return d;
}
pub unsafe extern "C" fn free_deferment(mut d: *mut deferment) {
    rpl_free((*d).header.c_name as *mut libc::c_void);
    rpl_free(d as *mut libc::c_void);
}
