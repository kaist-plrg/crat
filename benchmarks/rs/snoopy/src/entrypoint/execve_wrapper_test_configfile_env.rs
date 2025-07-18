use ::libc;
extern "C" {
    fn snoopy_init();
    fn snoopy_cleanup();
    fn snoopy_action_log_syscall_exec();
    fn snoopy_configuration_preinit_setConfigFilePathFromEnv();
    fn snoopy_inputdatastorage_store_filename(filename: *const libc::c_char);
    fn snoopy_inputdatastorage_store_argv(argv: *const *mut libc::c_char);
    fn snoopy_inputdatastorage_store_envp(envp: *const *mut libc::c_char);
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
}
pub unsafe extern "C" fn execv(
    mut filename: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    static mut func: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const *mut libc::c_char,
        ) -> libc::c_int,
    > = None;
    func = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const *mut libc::c_char,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"execv\0" as *const u8 as *const libc::c_char,
        ),
    );
    let mut envp: [*mut libc::c_char; 1] = [0 as *mut libc::c_char];
    snoopy_entrypoint_execve_wrapper_test_configfile_env_init(
        filename,
        argv,
        envp.as_mut_ptr() as *const *mut libc::c_char,
    );
    snoopy_action_log_syscall_exec();
    snoopy_entrypoint_execve_wrapper_test_configfile_env_exit();
    return (Some(func.unwrap())).unwrap()(filename, argv);
}
pub unsafe extern "C" fn execve(
    mut filename: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut envp: *const *mut libc::c_char,
) -> libc::c_int {
    static mut func: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
        ) -> libc::c_int,
    > = None;
    func = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const *mut libc::c_char,
                *const *mut libc::c_char,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"execve\0" as *const u8 as *const libc::c_char,
        ),
    );
    snoopy_entrypoint_execve_wrapper_test_configfile_env_init(filename, argv, envp);
    snoopy_action_log_syscall_exec();
    snoopy_entrypoint_execve_wrapper_test_configfile_env_exit();
    return (Some(func.unwrap())).unwrap()(filename, argv, envp);
}
pub unsafe extern "C" fn snoopy_entrypoint_execve_wrapper_test_configfile_env_init(
    mut filename: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut envp: *const *mut libc::c_char,
) {
    snoopy_configuration_preinit_setConfigFilePathFromEnv();
    snoopy_init();
    snoopy_inputdatastorage_store_filename(filename);
    snoopy_inputdatastorage_store_argv(argv);
    snoopy_inputdatastorage_store_envp(envp);
}
pub unsafe extern "C" fn snoopy_entrypoint_execve_wrapper_test_configfile_env_exit() {
    snoopy_cleanup();
}
