use ::libc;
extern "C" {
    fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lchownat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut owner: uid_t,
    mut group: gid_t,
) -> libc::c_int {
    return fchownat(fd, file, owner, group, 0x100 as libc::c_int);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn chownat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut owner: uid_t,
    mut group: gid_t,
) -> libc::c_int {
    return fchownat(fd, file, owner, group, 0 as libc::c_int);
}
