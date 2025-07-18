use ::libc;
extern "C" {
    pub type __dirstream;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn rpl_dirfd(_: *mut DIR) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type DIR = __dirstream;
pub unsafe extern "C" fn opendir_safer(mut name: *const libc::c_char) -> *mut DIR {
    let mut dp: *mut DIR = opendir(name);
    if !dp.is_null() {
        let mut fd: libc::c_int = rpl_dirfd(dp);
        if 0 as libc::c_int <= fd && fd <= 2 as libc::c_int {
            let mut newdp: *mut DIR = 0 as *mut DIR;
            let mut e: libc::c_int = 0;
            let mut f: libc::c_int = rpl_fcntl(
                fd,
                1030 as libc::c_int,
                2 as libc::c_int + 1 as libc::c_int,
            );
            if f < 0 as libc::c_int {
                e = *__errno_location();
                newdp = 0 as *mut DIR;
            } else {
                newdp = fdopendir(f);
                e = *__errno_location();
                if newdp.is_null() {
                    close(f);
                }
            }
            closedir(dp);
            *__errno_location() = e;
            dp = newdp;
        }
    }
    return dp;
}
