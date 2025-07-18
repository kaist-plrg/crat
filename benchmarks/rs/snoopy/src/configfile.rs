use ::libc;
extern "C" {
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_outputregistry_doesNameExist(
        outputName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_string_toUpper(s: *mut libc::c_char);
    fn snoopy_util_syslog_convertFacilityToInt(
        facilityStr: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_syslog_convertFacilityToStr(
        facilityInt: libc::c_int,
    ) -> *const libc::c_char;
    fn snoopy_util_syslog_convertLevelToInt(
        levelStr: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_syslog_convertLevelToStr(
        levelInt: libc::c_int,
    ) -> *const libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn snoopy_ini_parse(
        filename: *const libc::c_char,
        handler: ini_handler,
        user: *mut libc::c_void,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
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
pub struct snoopy_configfile_option_t {
    pub name: *const libc::c_char,
    pub data: snoopy_configfile_optionData_t,
}
pub type ini_handler = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub static mut snoopy_configfile_optionRegistry: [snoopy_configfile_option_t; 8] = unsafe {
    [
        {
            let mut init = snoopy_configfile_option_t {
                name: b"error_logging\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 1 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_error_logging
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_error_logging),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"filter_chain\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 2 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_filter_chain
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_filter_chain),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"message_format\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 2 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_message_format
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_message_format),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"output\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 2 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_output
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_output),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"syslog_facility\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 2 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_syslog_facility
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_syslog_facility),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"syslog_ident\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 2 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_syslog_ident
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_syslog_ident),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"syslog_level\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 2 as libc::c_int,
                        valueParserPtr: Some(
                            snoopy_configfile_parseValue_syslog_level
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut snoopy_configuration_t,
                                ) -> libc::c_int,
                        ),
                        getValueAsStringPtr: Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> *mut libc::c_char,
                                unsafe extern "C" fn() -> *mut libc::c_char,
                            >(snoopy_configfile_getOptionValueAsString_syslog_level),
                        ),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = snoopy_configfile_option_t {
                name: b"\0" as *const u8 as *const libc::c_char,
                data: {
                    let mut init = snoopy_configfile_optionData_t {
                        type_0: 0 as libc::c_int,
                        valueParserPtr: None,
                        getValueAsStringPtr: None,
                    };
                    init
                },
            };
            init
        },
    ]
};
pub unsafe extern "C" fn snoopy_configfile_load(
    mut iniFilePath: *mut libc::c_char,
) -> libc::c_int {
    let mut iniParseStatus: libc::c_int = 0;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    CFG = snoopy_configuration_get();
    (*CFG).configfile_path = iniFilePath;
    iniParseStatus = snoopy_ini_parse(
        iniFilePath,
        Some(
            snoopy_configfile_iniParser_callback
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        CFG as *mut libc::c_void,
    );
    if 0 as libc::c_int != iniParseStatus {
        return -(1 as libc::c_int);
    }
    (*CFG).configfile_found = 1 as libc::c_int;
    (*CFG).configfile_parsed = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_iniParser_callback(
    mut sth: *mut libc::c_void,
    mut section: *const libc::c_char,
    mut name: *const libc::c_char,
    mut confValString: *const libc::c_char,
) -> libc::c_int {
    let mut CFG: *mut snoopy_configuration_t = sth as *mut snoopy_configuration_t;
    if 0 as libc::c_int
        != strcmp(section, b"snoopy\0" as *const u8 as *const libc::c_char)
    {
        return 1 as libc::c_int;
    }
    let mut optionId: libc::c_int = snoopy_configfile_optionRegistry_getIdFromName(name);
    if optionId != -(1 as libc::c_int) {
        return (snoopy_configfile_optionRegistry[optionId as usize].data.valueParserPtr)
            .unwrap()(confValString, CFG);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_error_logging(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    let mut confValInt: libc::c_int = snoopy_configfile_getboolean(
        confValString,
        -(1 as libc::c_int),
    );
    if -(1 as libc::c_int) != confValInt {
        (*CFG).error_logging_enabled = confValInt;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_error_logging() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    if (*CFG).error_logging_enabled == 1 as libc::c_int {
        return strdup(b"yes\0" as *const u8 as *const libc::c_char)
    } else {
        return strdup(b"no\0" as *const u8 as *const libc::c_char)
    };
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_filter_chain(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    (*CFG).filter_chain = strdup(confValString);
    (*CFG).filter_chain_malloced = 1 as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_filter_chain() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    return strdup((*CFG).filter_chain);
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_message_format(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    (*CFG).message_format = strdup(confValString);
    (*CFG).message_format_malloced = 1 as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_message_format() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    return strdup((*CFG).message_format);
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_output(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    let mut confVal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outputName: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputArgFound: libc::c_int = 0 as libc::c_int;
    let mut saveptr1: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    confVal = strdup(confValString);
    if (strchr(confVal, ':' as i32)).is_null() {
        outputName = confVal;
        (*CFG)
            .output_arg = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*CFG).output_arg_malloced = 0 as libc::c_int;
        outputArg = b"\0" as *const u8 as *const libc::c_char;
    } else {
        outputName = strtok_r(
            confVal,
            b":\0" as *const u8 as *const libc::c_char,
            &mut saveptr1,
        );
        outputArg = strtok_r(
            0 as *mut libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
            &mut saveptr1,
        );
        outputArgFound = 1 as libc::c_int;
    }
    if 1 as libc::c_int == snoopy_outputregistry_doesNameExist(outputName) {
        (*CFG).output = strdup(outputName);
        (*CFG).output_malloced = 1 as libc::c_int;
        if 1 as libc::c_int == outputArgFound {
            (*CFG).output_arg = strdup(outputArg);
            (*CFG).output_arg_malloced = 1 as libc::c_int;
        }
    } else {
        (*CFG)
            .output = b"devlog\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*CFG).output_malloced = 0 as libc::c_int;
        (*CFG)
            .output_arg = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*CFG).output_arg_malloced = 0 as libc::c_int;
    }
    free(confVal as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_output() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    let mut outputString: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 as libc::c_int
        == strcmp(b"\0" as *const u8 as *const libc::c_char, (*CFG).output_arg)
    {
        outputString = strdup((*CFG).output);
    } else {
        let mut outputStringBufSize: size_t = (strlen((*CFG).output))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*CFG).output_arg))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        outputString = malloc(outputStringBufSize) as *mut libc::c_char;
        snprintf(
            outputString,
            outputStringBufSize,
            b"%s:%s\0" as *const u8 as *const libc::c_char,
            (*CFG).output,
            (*CFG).output_arg,
        );
        *outputString
            .offset(
                outputStringBufSize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    return outputString;
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_syslog_facility(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    let mut confVal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut confValCleaned: *const libc::c_char = 0 as *const libc::c_char;
    let mut facilityInt: libc::c_int = 0;
    confVal = strdup(confValString);
    confValCleaned = snoopy_configfile_syslog_value_cleanup(confVal);
    facilityInt = snoopy_util_syslog_convertFacilityToInt(confValCleaned);
    if -(1 as libc::c_int) == facilityInt {
        (*CFG).syslog_facility = (10 as libc::c_int) << 3 as libc::c_int;
    } else {
        (*CFG).syslog_facility = facilityInt;
    }
    free(confVal as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_syslog_facility() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    return strdup(snoopy_util_syslog_convertFacilityToStr((*CFG).syslog_facility));
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_syslog_ident(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    (*CFG).syslog_ident_format = strdup(confValString);
    (*CFG).syslog_ident_format_malloced = 1 as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_syslog_ident() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    return strdup((*CFG).syslog_ident_format);
}
pub unsafe extern "C" fn snoopy_configfile_parseValue_syslog_level(
    mut confValString: *const libc::c_char,
    mut CFG: *mut snoopy_configuration_t,
) -> libc::c_int {
    let mut confVal: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut confValCleaned: *const libc::c_char = 0 as *const libc::c_char;
    let mut levelInt: libc::c_int = 0;
    confVal = strdup(confValString);
    confValCleaned = snoopy_configfile_syslog_value_cleanup(confVal);
    levelInt = snoopy_util_syslog_convertLevelToInt(confValCleaned);
    if -(1 as libc::c_int) == levelInt {
        (*CFG).syslog_level = 6 as libc::c_int;
    } else {
        (*CFG).syslog_level = levelInt;
    }
    free(confVal as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_configfile_getOptionValueAsString_syslog_level() -> *mut libc::c_char {
    let mut CFG: *const snoopy_configuration_t = snoopy_configuration_get();
    return strdup(snoopy_util_syslog_convertLevelToStr((*CFG).syslog_level));
}
pub unsafe extern "C" fn snoopy_configfile_syslog_value_cleanup(
    mut confVal: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut confValCleaned: *mut libc::c_char = 0 as *mut libc::c_char;
    snoopy_util_string_toUpper(confVal);
    confValCleaned = snoopy_configfile_syslog_value_remove_prefix(confVal);
    return confValCleaned;
}
pub unsafe extern "C" fn snoopy_configfile_syslog_value_remove_prefix(
    mut confVal: *mut libc::c_char,
) -> *mut libc::c_char {
    if 0 as libc::c_int
        == strncmp(
            confVal,
            b"LOG_\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        )
    {
        return confVal.offset(4 as libc::c_int as isize)
    } else {
        return confVal
    };
}
pub unsafe extern "C" fn snoopy_configfile_getboolean(
    mut c: *const libc::c_char,
    mut notfound: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if *c.offset(0 as libc::c_int as isize) as libc::c_int == 'y' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == 'Y' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == '1' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32
    {
        ret = 1 as libc::c_int;
    } else if *c.offset(0 as libc::c_int as isize) as libc::c_int == 'n' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == 'f' as i32
        || *c.offset(0 as libc::c_int as isize) as libc::c_int == 'F' as i32
    {
        ret = 0 as libc::c_int;
    } else {
        ret = notfound;
    }
    return ret;
}
pub unsafe extern "C" fn snoopy_configfile_optionRegistry_getIdFromName(
    optionName: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while 0 as libc::c_int
        != strcmp(
            snoopy_configfile_optionRegistry[i as usize].name,
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        if strcmp(snoopy_configfile_optionRegistry[i as usize].name, optionName)
            == 0 as libc::c_int
        {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn snoopy_configfile_optionRegistry_getAll() -> *mut snoopy_configfile_option_t {
    return snoopy_configfile_optionRegistry.as_mut_ptr();
}
pub unsafe extern "C" fn snoopy_configfile_optionRegistry_getOptionValueAsString(
    optionName: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    while 0 as libc::c_int
        != strcmp(
            snoopy_configfile_optionRegistry[i as usize].name,
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        if strcmp(snoopy_configfile_optionRegistry[i as usize].name, optionName)
            == 0 as libc::c_int
        {
            return ::std::mem::transmute::<
                _,
                fn() -> *mut libc::c_char,
            >(
                (snoopy_configfile_optionRegistry[i as usize].data.getValueAsStringPtr)
                    .unwrap(),
            )();
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_char;
}
