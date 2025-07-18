use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut g_argv: *mut *mut libc::c_char;
    fn snoopyTestCli_action_run_datasource_all();
    fn snoopyTestCli_action_run_filter_all();
    fn snoopyTestCli_action_run_output_all();
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_filtering_check_chain(chain: *const libc::c_char) -> libc::c_int;
    fn snoopy_action_log_message_dispatch(
        logMessage: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_message_generateFromFormat(
        logMessage: *mut libc::c_char,
        logMessageBufSize: size_t,
        logMessageFormat: *mut libc::c_char,
    );
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub unsafe extern "C" fn snoopyTestCli_action_run_everything_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Run everything (for Valgrind)\n\nUsage:\n    snoopy-test run everything\n\nResult:\n    Runs as many subsystems as possible, to cover as much code as possible.\n    Useful for Valgrind analysis.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_everything() -> libc::c_int {
    let mut logMessage: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    CFG = snoopy_configuration_get();
    logMessage = malloc(16383 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    *logMessage.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"-----[ Filters ]---------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopyTestCli_action_run_filter_all();
    printf(
        b"-----[ Filtering ]-------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_filtering_check_chain(
        b"exclude_uid:10,11,12;only_uid=0,1,2,3\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Done.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"-----[ Datasources ]-----------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopyTestCli_action_run_datasource_all();
    printf(
        b"-----[ Outputs ]---------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopyTestCli_action_run_output_all();
    printf(
        b"-----[ Message formatting ]----------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_message_generateFromFormat(
        logMessage,
        16383 as libc::c_int as size_t,
        (*CFG).message_format,
    );
    printf(b"Message: %s\n\0" as *const u8 as *const libc::c_char, logMessage);
    printf(
        b"-----[ Dispatching ]-----------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_action_log_message_dispatch(logMessage);
    printf(b"Done.\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nAll done.\n\0" as *const u8 as *const libc::c_char);
    free(logMessage as *mut libc::c_void);
    snoopy_entrypoint_test_cli_exit();
    fclose(stdin);
    fclose(stdout);
    fclose(stderr);
    return 0 as libc::c_int;
}
