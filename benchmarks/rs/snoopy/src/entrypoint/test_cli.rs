use ::libc;
extern "C" {
    fn snoopy_init();
    fn snoopy_cleanup();
    fn snoopy_configuration_preinit_disableConfigFileParsing();
    fn snoopy_configuration_preinit_enableAltConfigFileParsing(
        altConfigFilePath: *mut libc::c_char,
    );
    fn snoopy_inputdatastorage_store_filename(filename: *const libc::c_char);
    fn snoopy_inputdatastorage_store_argv(argv: *const *mut libc::c_char);
    fn snoopy_inputdatastorage_store_envp(envp: *const *mut libc::c_char);
}
pub unsafe extern "C" fn snoopy_entrypoint_test_cli_init(
    mut filename: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    configFilePath: *mut libc::c_char,
) {
    if !configFilePath.is_null() {
        snoopy_configuration_preinit_enableAltConfigFileParsing(configFilePath);
    } else {
        snoopy_configuration_preinit_disableConfigFileParsing();
    }
    snoopy_init();
    snoopy_inputdatastorage_store_filename(filename);
    snoopy_inputdatastorage_store_argv(argv);
    let mut envp: [*mut libc::c_char; 1] = [0 as *mut libc::c_char];
    snoopy_inputdatastorage_store_envp(envp.as_mut_ptr() as *const *mut libc::c_char);
}
pub unsafe extern "C" fn snoopy_entrypoint_test_cli_threads_init() {
    snoopy_init();
}
pub unsafe extern "C" fn snoopy_entrypoint_test_cli_exit() {
    snoopy_cleanup();
}
