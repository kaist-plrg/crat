use ::libc;
extern "C" {
    pub type __spawn_action;
    fn posix_spawn_file_actions_addchdir_np(
        __actions: *mut posix_spawn_file_actions_t,
        __path: *const libc::c_char,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
pub unsafe extern "C" fn posix_spawn_file_actions_addchdir(
    mut file_actions: *mut posix_spawn_file_actions_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    return posix_spawn_file_actions_addchdir_np(file_actions, path);
}
