use ::libc;
extern "C" {
    fn snoopy_init();
    fn snoopy_cleanup();
    fn snoopy_inputdatastorage_store_filename(filename: *const libc::c_char);
    fn snoopy_inputdatastorage_store_argv(argv: *const *mut libc::c_char);
    fn snoopy_inputdatastorage_store_envp(envp: *const *mut libc::c_char);
}
pub unsafe extern "C" fn snoopy_entrypoint_cli_init() {
    snoopy_init();
    snoopy_inputdatastorage_store_filename(
        b"snoopy-cli\0" as *const u8 as *const libc::c_char,
    );
    let mut argv: [*mut libc::c_char; 1] = [0 as *mut libc::c_char];
    snoopy_inputdatastorage_store_argv(argv.as_mut_ptr() as *const *mut libc::c_char);
    let mut envp: [*mut libc::c_char; 1] = [0 as *mut libc::c_char];
    snoopy_inputdatastorage_store_envp(envp.as_mut_ptr() as *const *mut libc::c_char);
}
pub unsafe extern "C" fn snoopy_entrypoint_cli_exit() {
    snoopy_cleanup();
}
