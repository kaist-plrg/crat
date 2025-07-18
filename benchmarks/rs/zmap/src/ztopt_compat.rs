use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gengetopt_args_info {
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub help_given: libc::c_uint,
    pub version_given: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_parser_params {
    pub override_0: libc::c_int,
    pub initialize: libc::c_int,
    pub check_required: libc::c_int,
    pub check_ambiguity: libc::c_int,
    pub print_errors: libc::c_int,
}
pub type cmdline_parser_arg_type = libc::c_uint;
pub const ARG_NO: cmdline_parser_arg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_list {
    pub string_arg: *mut libc::c_char,
    pub next: *mut line_list,
}
pub static mut gengetopt_args_info_purpose: *const libc::c_char = b"A tests harness tool for zmap\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_usage: *const libc::c_char = b"Usage: ztests [OPTION]...\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_description: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub static mut gengetopt_args_info_help: [*const libc::c_char; 4] = [
    b"Additional options:\0" as *const u8 as *const libc::c_char,
    b"  -h, --help     Print help and exit\0" as *const u8 as *const libc::c_char,
    b"  -V, --version  Print version and exit\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub unsafe extern "C" fn cmdline_parser(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    return cmdline_parser2(
        argc,
        argv,
        args_info,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
}
pub unsafe extern "C" fn cmdline_parser2(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut override_0: libc::c_int,
    mut initialize: libc::c_int,
    mut check_required: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut params: cmdline_parser_params = cmdline_parser_params {
        override_0: 0,
        initialize: 0,
        check_required: 0,
        check_ambiguity: 0,
        print_errors: 0,
    };
    params.override_0 = override_0;
    params.initialize = initialize;
    params.check_required = check_required;
    params.check_ambiguity = 0 as libc::c_int;
    params.print_errors = 1 as libc::c_int;
    result = cmdline_parser_internal(
        argc,
        argv,
        args_info,
        &mut params,
        0 as *const libc::c_char,
    );
    if result == 1 as libc::c_int {
        cmdline_parser_free(args_info);
        exit(1 as libc::c_int);
    }
    return result;
}
pub unsafe extern "C" fn cmdline_parser_free(mut args_info: *mut gengetopt_args_info) {
    cmdline_parser_release(args_info);
}
unsafe extern "C" fn cmdline_parser_release(mut args_info: *mut gengetopt_args_info) {
    clear_given(args_info);
}
unsafe extern "C" fn clear_given(mut args_info: *mut gengetopt_args_info) {
    (*args_info).help_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).version_given = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn cmdline_parser_internal(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut params: *mut cmdline_parser_params,
    mut additional_error: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut error_occurred: libc::c_int = 0 as libc::c_int;
    let mut local_args_info: gengetopt_args_info = gengetopt_args_info {
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        help_given: 0,
        version_given: 0,
    };
    let mut override_0: libc::c_int = 0;
    let mut initialize: libc::c_int = 0;
    let mut check_required: libc::c_int = 0;
    let mut check_ambiguity: libc::c_int = 0;
    package_name = *argv.offset(0 as libc::c_int as isize);
    override_0 = (*params).override_0;
    initialize = (*params).initialize;
    check_required = (*params).check_required;
    check_ambiguity = (*params).check_ambiguity;
    if initialize != 0 {
        cmdline_parser_init(args_info);
    }
    cmdline_parser_init(&mut local_args_info);
    optarg = 0 as *mut libc::c_char;
    optind = 0 as libc::c_int;
    opterr = (*params).print_errors;
    optopt = '?' as i32;
    loop {
        let mut option_index: libc::c_int = 0 as libc::c_int;
        static mut long_options: [option; 3] = [
            {
                let mut init = option {
                    name: b"help\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'h' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"version\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'V' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: 0 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
        ];
        c = getopt_long(
            argc,
            argv,
            b"hV\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            current_block = 14401909646449704462;
            break;
        }
        match c {
            104 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).help_given,
                    &mut local_args_info.help_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"help\0" as *const u8 as *const libc::c_char,
                    'h' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11082423427765001199;
                    break;
                }
            }
            86 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).version_given,
                    &mut local_args_info.version_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"version\0" as *const u8 as *const libc::c_char,
                    'V' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11082423427765001199;
                    break;
                }
            }
            0 | 63 => {
                current_block = 11082423427765001199;
                break;
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s: option unknown: %c%s\n\0" as *const u8 as *const libc::c_char,
                    b"ztests\0" as *const u8 as *const libc::c_char,
                    c,
                    if !additional_error.is_null() {
                        additional_error
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                abort();
            }
        }
    }
    match current_block {
        11082423427765001199 => {
            cmdline_parser_release(&mut local_args_info);
            return 1 as libc::c_int;
        }
        _ => {
            cmdline_parser_release(&mut local_args_info);
            if error_occurred != 0 {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn update_arg(
    mut field: *mut libc::c_void,
    mut orig_field: *mut *mut libc::c_char,
    mut field_given: *mut libc::c_uint,
    mut prev_given: *mut libc::c_uint,
    mut value: *mut libc::c_char,
    mut possible_values: *mut *const libc::c_char,
    mut default_value: *const libc::c_char,
    mut arg_type: cmdline_parser_arg_type,
    mut check_ambiguity: libc::c_int,
    mut override_0: libc::c_int,
    mut no_free: libc::c_int,
    mut multiple_option: libc::c_int,
    mut long_opt: *const libc::c_char,
    mut short_opt: libc::c_char,
    mut additional_error: *const libc::c_char,
) -> libc::c_int {
    let mut stop_char: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *const libc::c_char = value;
    let mut found: libc::c_int = 0;
    stop_char = 0 as *mut libc::c_char;
    found = 0 as libc::c_int;
    if multiple_option == 0 && !prev_given.is_null()
        && (*prev_given != 0 || check_ambiguity != 0 && *field_given != 0)
    {
        if short_opt as libc::c_int != '-' as i32 {
            fprintf(
                stderr,
                b"%s: `--%s' (`-%c') option given more than once%s\n\0" as *const u8
                    as *const libc::c_char,
                package_name,
                long_opt,
                short_opt as libc::c_int,
                if !additional_error.is_null() {
                    additional_error
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        } else {
            fprintf(
                stderr,
                b"%s: `--%s' option given more than once%s\n\0" as *const u8
                    as *const libc::c_char,
                package_name,
                long_opt,
                if !additional_error.is_null() {
                    additional_error
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        return 1 as libc::c_int;
    }
    if !field_given.is_null() && *field_given != 0 && override_0 == 0 {
        return 0 as libc::c_int;
    }
    if !prev_given.is_null() {
        *prev_given = (*prev_given).wrapping_add(1);
        *prev_given;
    }
    if !field_given.is_null() {
        *field_given = (*field_given).wrapping_add(1);
        *field_given;
    }
    if !possible_values.is_null() {
        val = *possible_values.offset(found as isize);
    }
    match arg_type as libc::c_uint {
        _ => {}
    }
    match arg_type as libc::c_uint {
        0 => {}
        _ => {
            if !value.is_null() && !orig_field.is_null() {
                if no_free != 0 {
                    *orig_field = value;
                } else {
                    if !(*orig_field).is_null() {
                        free(*orig_field as *mut libc::c_void);
                    }
                    *orig_field = gengetopt_strdup(value);
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gengetopt_strdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() {
        return result;
    }
    result = malloc((strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if result.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(result, s);
    return result;
}
static mut package_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn cmdline_parser_init(mut args_info: *mut gengetopt_args_info) {
    clear_given(args_info);
    clear_args(args_info);
    init_args_info(args_info);
}
unsafe extern "C" fn init_args_info(mut args_info: *mut gengetopt_args_info) {
    (*args_info).help_help = gengetopt_args_info_help[1 as libc::c_int as usize];
    (*args_info).version_help = gengetopt_args_info_help[2 as libc::c_int as usize];
}
unsafe extern "C" fn clear_args(mut args_info: *mut gengetopt_args_info) {}
pub unsafe extern "C" fn cmdline_parser_ext(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut params: *mut cmdline_parser_params,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = cmdline_parser_internal(
        argc,
        argv,
        args_info,
        params,
        0 as *const libc::c_char,
    );
    if result == 1 as libc::c_int {
        cmdline_parser_free(args_info);
        exit(1 as libc::c_int);
    }
    return result;
}
pub unsafe extern "C" fn cmdline_parser_dump(
    mut outfile: *mut FILE,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if outfile.is_null() {
        fprintf(
            stderr,
            b"%s: cannot dump options to stream\n\0" as *const u8 as *const libc::c_char,
            b"ztests\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if (*args_info).help_given != 0 {
        write_into_file(
            outfile,
            b"help\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).version_given != 0 {
        write_into_file(
            outfile,
            b"version\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    return i;
}
unsafe extern "C" fn write_into_file(
    mut outfile: *mut FILE,
    mut opt: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut values: *mut *const libc::c_char,
) {
    if !arg.is_null() {
        fprintf(outfile, b"%s=\"%s\"\n\0" as *const u8 as *const libc::c_char, opt, arg);
    } else {
        fprintf(outfile, b"%s\n\0" as *const u8 as *const libc::c_char, opt);
    };
}
pub unsafe extern "C" fn cmdline_parser_file_save(
    mut filename: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    let mut outfile: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0 as libc::c_int;
    outfile = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if outfile.is_null() {
        fprintf(
            stderr,
            b"%s: cannot open file for writing: %s\n\0" as *const u8
                as *const libc::c_char,
            b"ztests\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    i = cmdline_parser_dump(outfile, args_info);
    fclose(outfile);
    return i;
}
pub unsafe extern "C" fn cmdline_parser_print_help() {
    let mut i: libc::c_int = 0 as libc::c_int;
    print_help_common();
    while !(gengetopt_args_info_help[i as usize]).is_null() {
        let fresh0 = i;
        i = i + 1;
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_help[fresh0 as usize],
        );
    }
}
unsafe extern "C" fn print_help_common() {
    let mut len_purpose: size_t = strlen(gengetopt_args_info_purpose);
    let mut len_usage: size_t = strlen(gengetopt_args_info_usage);
    if len_usage > 0 as libc::c_int as libc::c_ulong {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, gengetopt_args_info_usage);
    }
    if len_purpose > 0 as libc::c_int as libc::c_ulong {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_purpose,
        );
    }
    if len_usage != 0 || len_purpose != 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if strlen(gengetopt_args_info_description) > 0 as libc::c_int as libc::c_ulong {
        printf(
            b"%s\n\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_description,
        );
    }
}
pub unsafe extern "C" fn cmdline_parser_print_version() {
    printf(
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        if strlen(b"ztests\0" as *const u8 as *const libc::c_char) != 0 {
            b"ztests\0" as *const u8 as *const libc::c_char
        } else {
            b"ztests\0" as *const u8 as *const libc::c_char
        },
        b"DEVELOPMENT\0" as *const u8 as *const libc::c_char,
    );
    if strlen(gengetopt_args_info_versiontext) > 0 as libc::c_int as libc::c_ulong {
        printf(
            b"\n%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_versiontext,
        );
    }
}
pub static mut gengetopt_args_info_versiontext: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn cmdline_parser_params_init(
    mut params: *mut cmdline_parser_params,
) {
    if !params.is_null() {
        (*params).override_0 = 0 as libc::c_int;
        (*params).initialize = 1 as libc::c_int;
        (*params).check_required = 1 as libc::c_int;
        (*params).check_ambiguity = 0 as libc::c_int;
        (*params).print_errors = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn cmdline_parser_params_create() -> *mut cmdline_parser_params {
    let mut params: *mut cmdline_parser_params = malloc(
        ::std::mem::size_of::<cmdline_parser_params>() as libc::c_ulong,
    ) as *mut cmdline_parser_params;
    cmdline_parser_params_init(params);
    return params;
}
pub unsafe extern "C" fn cmdline_parser_configfile(
    mut filename: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut override_0: libc::c_int,
    mut initialize: libc::c_int,
    mut check_required: libc::c_int,
) -> libc::c_int {
    let mut params: cmdline_parser_params = cmdline_parser_params {
        override_0: 0,
        initialize: 0,
        check_required: 0,
        check_ambiguity: 0,
        print_errors: 0,
    };
    params.override_0 = override_0;
    params.initialize = initialize;
    params.check_required = check_required;
    params.check_ambiguity = 0 as libc::c_int;
    params.print_errors = 1 as libc::c_int;
    return cmdline_parser_config_file(filename, args_info, &mut params);
}
pub unsafe extern "C" fn cmdline_parser_config_file(
    mut filename: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut params: *mut cmdline_parser_params,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut my_argc: libc::c_int = 1 as libc::c_int;
    let mut my_argv_arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut additional_error: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd_line_list_tmp = malloc(::std::mem::size_of::<line_list>() as libc::c_ulong)
        as *mut line_list;
    (*cmd_line_list_tmp).next = cmd_line_list;
    cmd_line_list = cmd_line_list_tmp;
    (*cmd_line_list)
        .string_arg = gengetopt_strdup(b"ztests\0" as *const u8 as *const libc::c_char);
    result = _cmdline_parser_configfile(filename, &mut my_argc);
    if result != 1 as libc::c_int {
        my_argv_arg = malloc(
            ((my_argc + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        cmd_line_list_tmp = cmd_line_list;
        i = my_argc - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let ref mut fresh1 = *my_argv_arg.offset(i as isize);
            *fresh1 = (*cmd_line_list_tmp).string_arg;
            cmd_line_list_tmp = (*cmd_line_list_tmp).next;
            i -= 1;
            i;
        }
        let ref mut fresh2 = *my_argv_arg.offset(my_argc as isize);
        *fresh2 = 0 as *mut libc::c_char;
        additional_error = malloc(
            (strlen(filename))
                .wrapping_add(
                    strlen(
                        b" in configuration file \0" as *const u8 as *const libc::c_char,
                    ),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(
            additional_error,
            b" in configuration file \0" as *const u8 as *const libc::c_char,
        );
        strcat(additional_error, filename);
        result = cmdline_parser_internal(
            my_argc,
            my_argv_arg,
            args_info,
            params,
            additional_error,
        );
        free(additional_error as *mut libc::c_void);
        free(my_argv_arg as *mut libc::c_void);
    }
    free_cmd_list();
    if result == 1 as libc::c_int {
        cmdline_parser_free(args_info);
        exit(1 as libc::c_int);
    }
    return result;
}
unsafe extern "C" fn free_cmd_list() {
    if !cmd_line_list.is_null() {
        while !cmd_line_list.is_null() {
            cmd_line_list_tmp = cmd_line_list;
            cmd_line_list = (*cmd_line_list).next;
            free((*cmd_line_list_tmp).string_arg as *mut libc::c_void);
            free(cmd_line_list_tmp as *mut libc::c_void);
        }
    }
}
static mut cmd_line_list_tmp: *mut line_list = 0 as *const line_list as *mut line_list;
static mut cmd_line_list: *mut line_list = 0 as *const line_list as *mut line_list;
unsafe extern "C" fn _cmdline_parser_configfile(
    mut filename: *const libc::c_char,
    mut my_argc: *mut libc::c_int,
) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut my_argv: [libc::c_char; 2052] = [0; 2052];
    let mut linebuf: [libc::c_char; 2048] = [0; 2048];
    let mut line_num: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut equal: libc::c_int = 0;
    let mut fopt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut farg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_index: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut next_token: size_t = 0;
    let mut delimiter: libc::c_char = 0;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fprintf(
            stderr,
            b"%s: Error opening configuration file '%s'\n\0" as *const u8
                as *const libc::c_char,
            b"ztests\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    while !(fgets(linebuf.as_mut_ptr(), 2048 as libc::c_int, file)).is_null() {
        line_num += 1;
        line_num;
        my_argv[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        len = strlen(linebuf.as_mut_ptr());
        if len
            > (2048 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                as libc::c_ulong
        {
            fprintf(
                stderr,
                b"%s:%s:%d: Line too long in configuration file\n\0" as *const u8
                    as *const libc::c_char,
                b"ztests\0" as *const u8 as *const libc::c_char,
                filename,
                line_num,
            );
            result = 1 as libc::c_int;
            break;
        } else {
            next_token = strspn(
                linebuf.as_mut_ptr(),
                b" \t\r\n\0" as *const u8 as *const libc::c_char,
            );
            str_index = linebuf.as_mut_ptr().offset(next_token as isize);
            if *str_index.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                || *str_index.offset(0 as libc::c_int as isize) as libc::c_int
                    == '#' as i32
            {
                continue;
            }
            fopt = str_index;
            next_token = strcspn(
                fopt,
                b" \t\r\n=\0" as *const u8 as *const libc::c_char,
            );
            if *fopt.offset(next_token as isize) as libc::c_int == '\0' as i32 {
                farg = 0 as *mut libc::c_char;
                equal = 0 as libc::c_int;
            } else {
                equal = (*fopt.offset(next_token as isize) as libc::c_int == '=' as i32)
                    as libc::c_int;
                let fresh3 = next_token;
                next_token = next_token.wrapping_add(1);
                *fopt.offset(fresh3 as isize) = '\0' as i32 as libc::c_char;
                next_token = (next_token as libc::c_ulong)
                    .wrapping_add(
                        strspn(
                            fopt.offset(next_token as isize),
                            b" \t\r\n\0" as *const u8 as *const libc::c_char,
                        ),
                    ) as size_t as size_t;
                if equal == 0 {
                    equal = (*fopt.offset(next_token as isize) as libc::c_int
                        == '=' as i32) as libc::c_int;
                    if equal != 0 {
                        next_token = next_token.wrapping_add(1);
                        next_token;
                        next_token = (next_token as libc::c_ulong)
                            .wrapping_add(
                                strspn(
                                    fopt.offset(next_token as isize),
                                    b" \t\r\n\0" as *const u8 as *const libc::c_char,
                                ),
                            ) as size_t as size_t;
                    }
                }
                str_index = str_index.offset(next_token as isize);
                farg = str_index;
                if *farg.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
                    || *farg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\'' as i32
                {
                    farg = farg.offset(1);
                    str_index = strchr(
                        farg,
                        *str_index.offset(0 as libc::c_int as isize) as libc::c_int,
                    );
                    if str_index.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%s:%d: unterminated string in configuration file\n\0"
                                as *const u8 as *const libc::c_char,
                            b"ztests\0" as *const u8 as *const libc::c_char,
                            filename,
                            line_num,
                        );
                        result = 1 as libc::c_int;
                        break;
                    }
                } else {
                    next_token = strcspn(
                        farg,
                        b" \t\r\n#'\"\0" as *const u8 as *const libc::c_char,
                    );
                    str_index = str_index.offset(next_token as isize);
                }
                delimiter = *str_index;
                let fresh4 = str_index;
                str_index = str_index.offset(1);
                *fresh4 = '\0' as i32 as libc::c_char;
                if delimiter as libc::c_int != '\0' as i32
                    && delimiter as libc::c_int != '#' as i32
                {
                    str_index = str_index
                        .offset(
                            strspn(
                                str_index,
                                b" \t\r\n\0" as *const u8 as *const libc::c_char,
                            ) as isize,
                        );
                    if *str_index as libc::c_int != '\0' as i32
                        && *str_index as libc::c_int != '#' as i32
                    {
                        fprintf(
                            stderr,
                            b"%s:%s:%d: malformed string in configuration file\n\0"
                                as *const u8 as *const libc::c_char,
                            b"ztests\0" as *const u8 as *const libc::c_char,
                            filename,
                            line_num,
                        );
                        result = 1 as libc::c_int;
                        break;
                    }
                }
            }
            if strcmp(fopt, b"include\0" as *const u8 as *const libc::c_char) == 0 {
                if !farg.is_null() && *farg as libc::c_int != 0 {
                    result = _cmdline_parser_configfile(farg, my_argc);
                } else {
                    fprintf(
                        stderr,
                        b"%s:%s:%d: include requires a filename argument.\n\0"
                            as *const u8 as *const libc::c_char,
                        b"ztests\0" as *const u8 as *const libc::c_char,
                        filename,
                        line_num,
                    );
                }
            } else {
                len = strlen(fopt);
                strcat(
                    my_argv.as_mut_ptr(),
                    if len > 1 as libc::c_int as libc::c_ulong {
                        b"--\0" as *const u8 as *const libc::c_char
                    } else {
                        b"-\0" as *const u8 as *const libc::c_char
                    },
                );
                strcat(my_argv.as_mut_ptr(), fopt);
                if len > 1 as libc::c_int as libc::c_ulong
                    && (!farg.is_null() && *farg as libc::c_int != 0 || equal != 0)
                {
                    strcat(
                        my_argv.as_mut_ptr(),
                        b"=\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !farg.is_null() && *farg as libc::c_int != 0 {
                    strcat(my_argv.as_mut_ptr(), farg);
                }
                *my_argc += 1;
                *my_argc;
                cmd_line_list_tmp = malloc(
                    ::std::mem::size_of::<line_list>() as libc::c_ulong,
                ) as *mut line_list;
                (*cmd_line_list_tmp).next = cmd_line_list;
                cmd_line_list = cmd_line_list_tmp;
                (*cmd_line_list).string_arg = gengetopt_strdup(my_argv.as_mut_ptr());
            }
        }
    }
    if !file.is_null() {
        fclose(file);
    }
    return result;
}
pub unsafe extern "C" fn cmdline_parser_required(
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
