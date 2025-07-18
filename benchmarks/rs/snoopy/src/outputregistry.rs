use ::libc;
extern "C" {
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_genericregistry_getCount(regArray: *mut *mut libc::c_char) -> libc::c_int;
    fn snoopy_genericregistry_doesIdExist(
        regArray: *mut *mut libc::c_char,
        itemId: libc::c_int,
    ) -> libc::c_int;
    fn snoopy_genericregistry_doesNameExist(
        regArray: *mut *mut libc::c_char,
        itemName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_genericregistry_getIdFromName(
        regArray: *mut *mut libc::c_char,
        itemName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_genericregistry_getName(
        regArray: *mut *mut libc::c_char,
        itemId: libc::c_int,
    ) -> *mut libc::c_char;
    fn snoopy_output_devlogoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_devnulloutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_devttyoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_fileoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_socketoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_stderroutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_stdoutoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_output_noopoutput(
        logMessage: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_configuration_t {
    pub initialized: libc::c_int,
    pub configfile_enabled: libc::c_int,
    pub configfile_path: *mut libc::c_char,
    pub configfile_found: libc::c_int,
    pub configfile_parsed: libc::c_int,
    pub error_logging_enabled: libc::c_int,
    pub message_format: *mut libc::c_char,
    pub message_format_malloced: libc::c_int,
    pub filtering_enabled: libc::c_int,
    pub filter_chain: *mut libc::c_char,
    pub filter_chain_malloced: libc::c_int,
    pub output: *mut libc::c_char,
    pub output_malloced: libc::c_int,
    pub output_arg: *mut libc::c_char,
    pub output_arg_malloced: libc::c_int,
    pub syslog_facility: libc::c_int,
    pub syslog_level: libc::c_int,
    pub syslog_ident_format_malloced: libc::c_int,
    pub syslog_ident_format: *mut libc::c_char,
}
pub static mut snoopy_outputregistry_names: [*mut libc::c_char; 9] = [
    b"devlog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"devnull\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"devtty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"socket\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stderr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"noop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut snoopy_outputregistry_ptrs: [Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
>; 8] = unsafe {
    [
        Some(
            snoopy_output_devlogoutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_devnulloutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_devttyoutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_fileoutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_socketoutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_stderroutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_stdoutoutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            snoopy_output_noopoutput
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    ]
};
pub unsafe extern "C" fn snoopy_outputregistry_getCount() -> libc::c_int {
    return snoopy_genericregistry_getCount(snoopy_outputregistry_names.as_mut_ptr());
}
pub unsafe extern "C" fn snoopy_outputregistry_doesIdExist(
    mut outputId: libc::c_int,
) -> libc::c_int {
    return snoopy_genericregistry_doesIdExist(
        snoopy_outputregistry_names.as_mut_ptr(),
        outputId,
    );
}
pub unsafe extern "C" fn snoopy_outputregistry_doesNameExist(
    outputName: *const libc::c_char,
) -> libc::c_int {
    return snoopy_genericregistry_doesNameExist(
        snoopy_outputregistry_names.as_mut_ptr(),
        outputName,
    );
}
pub unsafe extern "C" fn snoopy_outputregistry_getIdFromName(
    outputName: *const libc::c_char,
) -> libc::c_int {
    return snoopy_genericregistry_getIdFromName(
        snoopy_outputregistry_names.as_mut_ptr(),
        outputName,
    );
}
pub unsafe extern "C" fn snoopy_outputregistry_getName(
    mut outputId: libc::c_int,
) -> *mut libc::c_char {
    return snoopy_genericregistry_getName(
        snoopy_outputregistry_names.as_mut_ptr(),
        outputId,
    );
}
pub unsafe extern "C" fn snoopy_outputregistry_callById(
    mut outputId: libc::c_int,
    logMessage: *const libc::c_char,
    outputArg: *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int == snoopy_outputregistry_doesIdExist(outputId) {
        return -(1 as libc::c_int);
    }
    return (snoopy_outputregistry_ptrs[outputId as usize])
        .unwrap()(logMessage, outputArg);
}
pub unsafe extern "C" fn snoopy_outputregistry_callByName(
    outputName: *const libc::c_char,
    logMessage: *const libc::c_char,
    outputArg: *const libc::c_char,
) -> libc::c_int {
    let mut outputId: libc::c_int = 0;
    outputId = snoopy_outputregistry_getIdFromName(outputName);
    if outputId == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return (snoopy_outputregistry_ptrs[outputId as usize])
        .unwrap()(logMessage, outputArg);
}
pub unsafe extern "C" fn snoopy_outputregistry_dispatch(
    logMessage: *const libc::c_char,
) -> libc::c_int {
    let mut CFG: *const snoopy_configuration_t = 0 as *const snoopy_configuration_t;
    CFG = snoopy_configuration_get();
    return snoopy_outputregistry_callByName(
        (*CFG).output,
        logMessage,
        (*CFG).output_arg,
    );
}
