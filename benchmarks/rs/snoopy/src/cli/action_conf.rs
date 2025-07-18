use ::libc;
extern "C" {
    fn libsnoopySo_load();
    fn libsnoopySo_dlsym(_: *const libc::c_char) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_configfile_option_t {
    pub name: *const libc::c_char,
    pub data: snoopy_configfile_optionData_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_configfile_optionData_t {
    pub type_0: libc::c_int,
    pub valueParserPtr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut snoopy_configuration_t,
        ) -> libc::c_int,
    >,
    pub getValueAsStringPtr: Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
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
pub unsafe extern "C" fn snoopy_cli_action_conf() -> libc::c_int {
    let mut snoopy_entrypoint_cli_init_ptr: Option::<unsafe extern "C" fn() -> ()> = None;
    let mut snoopy_entrypoint_cli_exit_ptr: Option::<unsafe extern "C" fn() -> ()> = None;
    let mut snoopy_configuration_get_ptr: Option::<
        unsafe extern "C" fn() -> *mut snoopy_configuration_t,
    > = None;
    let mut snoopy_configfile_optionRegistry_getAll_ptr: Option::<
        unsafe extern "C" fn() -> *mut snoopy_configfile_option_t,
    > = None;
    let mut snoopy_configfile_optionRegistry_getOptionValueAsString_ptr: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    > = None;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    let mut optionRegistry: *const snoopy_configfile_option_t = 0
        as *const snoopy_configfile_option_t;
    libsnoopySo_load();
    let ref mut fresh0 = *(&mut snoopy_entrypoint_cli_init_ptr
        as *mut Option::<unsafe extern "C" fn() -> ()> as *mut *mut libc::c_void);
    *fresh0 = libsnoopySo_dlsym(
        b"snoopy_entrypoint_cli_init\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh1 = *(&mut snoopy_entrypoint_cli_exit_ptr
        as *mut Option::<unsafe extern "C" fn() -> ()> as *mut *mut libc::c_void);
    *fresh1 = libsnoopySo_dlsym(
        b"snoopy_entrypoint_cli_exit\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh2 = *(&mut snoopy_configuration_get_ptr
        as *mut Option::<unsafe extern "C" fn() -> *mut snoopy_configuration_t>
        as *mut *mut libc::c_void);
    *fresh2 = libsnoopySo_dlsym(
        b"snoopy_configuration_get\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh3 = *(&mut snoopy_configfile_optionRegistry_getAll_ptr
        as *mut Option::<unsafe extern "C" fn() -> *mut snoopy_configfile_option_t>
        as *mut *mut libc::c_void);
    *fresh3 = libsnoopySo_dlsym(
        b"snoopy_configfile_optionRegistry_getAll\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh4 = *(&mut snoopy_configfile_optionRegistry_getOptionValueAsString_ptr
        as *mut Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char>
        as *mut *mut libc::c_void);
    *fresh4 = libsnoopySo_dlsym(
        b"snoopy_configfile_optionRegistry_getOptionValueAsString\0" as *const u8
            as *const libc::c_char,
    );
    ::std::mem::transmute::<_, fn()>(snoopy_entrypoint_cli_init_ptr.unwrap())();
    CFG = ::std::mem::transmute::<
        _,
        fn() -> *mut snoopy_configuration_t,
    >(snoopy_configuration_get_ptr.unwrap())();
    printf(
        b"; Options from config file (or defaults): %s\n\0" as *const u8
            as *const libc::c_char,
        (*CFG).configfile_path,
    );
    printf(b"[snoopy]\n\0" as *const u8 as *const libc::c_char);
    optionRegistry = ::std::mem::transmute::<
        _,
        fn() -> *mut snoopy_configfile_option_t,
    >(snoopy_configfile_optionRegistry_getAll_ptr.unwrap())();
    let mut i: libc::c_int = 0 as libc::c_int;
    while 0 as libc::c_int
        != strcmp(
            (*optionRegistry.offset(i as isize)).name,
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        let mut optionValue: *mut libc::c_char = snoopy_configfile_optionRegistry_getOptionValueAsString_ptr
            .unwrap()((*optionRegistry.offset(i as isize)).name);
        printf(
            b"%s = %s\n\0" as *const u8 as *const libc::c_char,
            (*optionRegistry.offset(i as isize)).name,
            optionValue,
        );
        free(optionValue as *mut libc::c_void);
        i += 1;
        i;
    }
    ::std::mem::transmute::<_, fn()>(snoopy_entrypoint_cli_exit_ptr.unwrap())();
    return 0 as libc::c_int;
}
