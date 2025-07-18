use ::libc;
extern "C" {
    pub type __dirstream;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn __errno_location() -> *mut libc::c_int;
    fn openat_safer(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type DIR = __dirstream;
pub unsafe extern "C" fn opendirat(
    mut dir_fd: libc::c_int,
    mut dir: *const libc::c_char,
    mut extra_flags: libc::c_int,
    mut pnew_fd: *mut libc::c_int,
) -> *mut DIR {
    let mut open_flags: libc::c_int = 0 as libc::c_int | 0o2000000 as libc::c_int
        | 0o200000 as libc::c_int | 0o400 as libc::c_int | 0o4000 as libc::c_int
        | extra_flags;
    let mut new_fd: libc::c_int = openat_safer(dir_fd, dir, open_flags);
    if new_fd < 0 as libc::c_int {
        return 0 as *mut DIR;
    }
    let mut dirp: *mut DIR = fdopendir(new_fd);
    if !dirp.is_null() {
        *pnew_fd = new_fd;
    } else {
        let mut fdopendir_errno: libc::c_int = *__errno_location();
        close(new_fd);
        *__errno_location() = fdopendir_errno;
    }
    return dirp;
}
