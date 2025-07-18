use ::libc;
extern "C" {
    fn getuid() -> __uid_t;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub unsafe extern "C" fn get_env_value(
    mut varname: *mut libc::c_char,
) -> *mut libc::c_char {
    return getenv(varname);
}
pub unsafe extern "C" fn get_home_dir() -> *mut libc::c_char {
    let mut home_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut entry: *mut passwd = 0 as *mut passwd;
    home_dir = 0 as *mut libc::c_void as *mut libc::c_char;
    entry = getpwuid(getuid());
    if !entry.is_null() {
        home_dir = (*entry).pw_dir;
    }
    return home_dir;
}
