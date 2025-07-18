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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    pub success_only_help: *const libc::c_char,
    pub monitor_help: *const libc::c_char,
    pub status_updates_file_arg: *mut libc::c_char,
    pub status_updates_file_orig: *mut libc::c_char,
    pub status_updates_file_help: *const libc::c_char,
    pub log_file_arg: *mut libc::c_char,
    pub log_file_orig: *mut libc::c_char,
    pub log_file_help: *const libc::c_char,
    pub raw_help: *const libc::c_char,
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub success_only_given: libc::c_uint,
    pub monitor_given: libc::c_uint,
    pub status_updates_file_given: libc::c_uint,
    pub log_file_given: libc::c_uint,
    pub raw_given: libc::c_uint,
    pub help_given: libc::c_uint,
    pub version_given: libc::c_uint,
    pub inputs: *mut *mut libc::c_char,
    pub inputs_num: libc::c_uint,
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
pub const ARG_STRING: cmdline_parser_arg_type = 1;
pub const ARG_NO: cmdline_parser_arg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_list {
    pub string_arg: *mut libc::c_char,
    pub next: *mut line_list,
}
pub static mut gengetopt_args_info_purpose: *const libc::c_char = b"A buffering output splitter\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_usage: *const libc::c_char = b"Usage: ztee [OPTION]... [FILE]...\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_description: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub static mut gengetopt_args_info_help: [*const libc::c_char; 11] = [
    b"Basic arguments:\0" as *const u8 as *const libc::c_char,
    b"      --success-only            Only write to stdout rows where success=1 or\n                                  success=true\0"
        as *const u8 as *const libc::c_char,
    b"  -m, --monitor                 Print monitor data to stderr\0" as *const u8
        as *const libc::c_char,
    b"  -u, --status-updates-file=monitor.csv\n                                File to write status updates, in CSV format\0"
        as *const u8 as *const libc::c_char,
    b"  -l, --log-file=STRING         File to log errors, etc. to\0" as *const u8
        as *const libc::c_char,
    b"  -r, --raw                     Ignore input formatting and pass through raw\n                                  input\0"
        as *const u8 as *const libc::c_char,
    b"\nAdditional options:\0" as *const u8 as *const libc::c_char,
    b"  -h, --help                    Print help and exit\0" as *const u8
        as *const libc::c_char,
    b"  -V, --version                 Print version and exit\0" as *const u8
        as *const libc::c_char,
    b"\nExamples:\n    zmap -p 80 -o - | ztee zmap.csv (save zmap output to zmap.csv and output\nall IP addresses to stdout)\n    zmap -p 80 --output-fields=* -o - | ztee --success-only zmap.csv  (save all\nzmap output to zmap.csv, print IPs from successful rows to stdout)\n    zmap -p 80 -o - | ztee -u status.csv zmap.csv (save zmap output to\nzmap.csv, write status updates to status.csv, print all IPs to stdout)\n    echo \"hello, ztee\" | ztee --raw out.txt (write text to out.txt and to\nstdout, like tee)\0"
        as *const u8 as *const libc::c_char,
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
    let mut i: libc::c_uint = 0;
    free_string_field(&mut (*args_info).status_updates_file_arg);
    free_string_field(&mut (*args_info).status_updates_file_orig);
    free_string_field(&mut (*args_info).log_file_arg);
    free_string_field(&mut (*args_info).log_file_orig);
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*args_info).inputs_num {
        free(*((*args_info).inputs).offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    if (*args_info).inputs_num != 0 {
        free((*args_info).inputs as *mut libc::c_void);
    }
    clear_given(args_info);
}
unsafe extern "C" fn clear_given(mut args_info: *mut gengetopt_args_info) {
    (*args_info).success_only_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).monitor_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).status_updates_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).log_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).raw_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).help_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).version_given = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn free_string_field(mut s: *mut *mut libc::c_char) {
    if !(*s).is_null() {
        free(*s as *mut libc::c_void);
        *s = 0 as *mut libc::c_char;
    }
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
        success_only_help: 0 as *const libc::c_char,
        monitor_help: 0 as *const libc::c_char,
        status_updates_file_arg: 0 as *mut libc::c_char,
        status_updates_file_orig: 0 as *mut libc::c_char,
        status_updates_file_help: 0 as *const libc::c_char,
        log_file_arg: 0 as *mut libc::c_char,
        log_file_orig: 0 as *mut libc::c_char,
        log_file_help: 0 as *const libc::c_char,
        raw_help: 0 as *const libc::c_char,
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        success_only_given: 0,
        monitor_given: 0,
        status_updates_file_given: 0,
        log_file_given: 0,
        raw_given: 0,
        help_given: 0,
        version_given: 0,
        inputs: 0 as *mut *mut libc::c_char,
        inputs_num: 0,
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
        static mut long_options: [option; 8] = [
            {
                let mut init = option {
                    name: b"success-only\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"monitor\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'm' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"status-updates-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'u' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"log-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'l' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"raw\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'r' as i32,
                };
                init
            },
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
            b"mu:l:rhV\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            current_block = 17500079516916021833;
            break;
        }
        match c {
            109 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).monitor_given,
                    &mut local_args_info.monitor_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"monitor\0" as *const u8 as *const libc::c_char,
                    'm' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 15442404127985124400;
                    break;
                }
            }
            117 => {
                if update_arg(
                    &mut (*args_info).status_updates_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).status_updates_file_orig,
                    &mut (*args_info).status_updates_file_given,
                    &mut local_args_info.status_updates_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"status-updates-file\0" as *const u8 as *const libc::c_char,
                    'u' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 15442404127985124400;
                    break;
                }
            }
            108 => {
                if update_arg(
                    &mut (*args_info).log_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).log_file_orig,
                    &mut (*args_info).log_file_given,
                    &mut local_args_info.log_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"log-file\0" as *const u8 as *const libc::c_char,
                    'l' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 15442404127985124400;
                    break;
                }
            }
            114 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).raw_given,
                    &mut local_args_info.raw_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"raw\0" as *const u8 as *const libc::c_char,
                    'r' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 15442404127985124400;
                    break;
                }
            }
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
                    current_block = 15442404127985124400;
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
                    current_block = 15442404127985124400;
                    break;
                }
            }
            0 => {
                if !(strcmp(
                    long_options[option_index as usize].name,
                    b"success-only\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
                {
                    continue;
                }
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).success_only_given,
                    &mut local_args_info.success_only_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"success-only\0" as *const u8 as *const libc::c_char,
                    '-' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 15442404127985124400;
                    break;
                }
            }
            63 => {
                current_block = 15442404127985124400;
                break;
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s: option unknown: %c%s\n\0" as *const u8 as *const libc::c_char,
                    b"ztee\0" as *const u8 as *const libc::c_char,
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
        15442404127985124400 => {
            cmdline_parser_release(&mut local_args_info);
            return 1 as libc::c_int;
        }
        _ => {
            cmdline_parser_release(&mut local_args_info);
            if error_occurred != 0 {
                return 1 as libc::c_int;
            }
            if optind < argc {
                let mut i: libc::c_int = 0 as libc::c_int;
                let mut found_prog_name: libc::c_int = 0 as libc::c_int;
                i = optind;
                while i < argc {
                    let fresh0 = i;
                    i = i + 1;
                    if !(*argv.offset(fresh0 as isize)
                        == *argv.offset(0 as libc::c_int as isize))
                    {
                        continue;
                    }
                    found_prog_name = 1 as libc::c_int;
                    break;
                }
                i = 0 as libc::c_int;
                (*args_info)
                    .inputs_num = (argc - optind - found_prog_name) as libc::c_uint;
                (*args_info)
                    .inputs = malloc(
                    ((*args_info).inputs_num as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                while optind < argc {
                    let fresh1 = optind;
                    optind = optind + 1;
                    if *argv.offset(fresh1 as isize)
                        != *argv.offset(0 as libc::c_int as isize)
                    {
                        let fresh2 = i;
                        i = i + 1;
                        let ref mut fresh3 = *((*args_info).inputs)
                            .offset(fresh2 as isize);
                        *fresh3 = gengetopt_strdup(
                            *argv.offset((optind - 1 as libc::c_int) as isize),
                        );
                    }
                }
            }
            return 0 as libc::c_int;
        }
    };
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
    let mut string_field: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
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
        1 => {
            if !val.is_null() {
                string_field = field as *mut *mut libc::c_char;
                if no_free == 0 && !(*string_field).is_null() {
                    free(*string_field as *mut libc::c_void);
                }
                *string_field = gengetopt_strdup(val);
            }
        }
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
static mut package_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn cmdline_parser_init(mut args_info: *mut gengetopt_args_info) {
    clear_given(args_info);
    clear_args(args_info);
    init_args_info(args_info);
    (*args_info).inputs = 0 as *mut *mut libc::c_char;
    (*args_info).inputs_num = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn init_args_info(mut args_info: *mut gengetopt_args_info) {
    (*args_info).success_only_help = gengetopt_args_info_help[1 as libc::c_int as usize];
    (*args_info).monitor_help = gengetopt_args_info_help[2 as libc::c_int as usize];
    (*args_info)
        .status_updates_file_help = gengetopt_args_info_help[3 as libc::c_int as usize];
    (*args_info).log_file_help = gengetopt_args_info_help[4 as libc::c_int as usize];
    (*args_info).raw_help = gengetopt_args_info_help[5 as libc::c_int as usize];
    (*args_info).help_help = gengetopt_args_info_help[7 as libc::c_int as usize];
    (*args_info).version_help = gengetopt_args_info_help[8 as libc::c_int as usize];
}
unsafe extern "C" fn clear_args(mut args_info: *mut gengetopt_args_info) {
    (*args_info).status_updates_file_arg = 0 as *mut libc::c_char;
    (*args_info).status_updates_file_orig = 0 as *mut libc::c_char;
    (*args_info).log_file_arg = 0 as *mut libc::c_char;
    (*args_info).log_file_orig = 0 as *mut libc::c_char;
}
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
            b"ztee\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if (*args_info).success_only_given != 0 {
        write_into_file(
            outfile,
            b"success-only\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).monitor_given != 0 {
        write_into_file(
            outfile,
            b"monitor\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).status_updates_file_given != 0 {
        write_into_file(
            outfile,
            b"status-updates-file\0" as *const u8 as *const libc::c_char,
            (*args_info).status_updates_file_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).log_file_given != 0 {
        write_into_file(
            outfile,
            b"log-file\0" as *const u8 as *const libc::c_char,
            (*args_info).log_file_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).raw_given != 0 {
        write_into_file(
            outfile,
            b"raw\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
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
            b"ztee\0" as *const u8 as *const libc::c_char,
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
        let fresh4 = i;
        i = i + 1;
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_help[fresh4 as usize],
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
        if strlen(b"ztee\0" as *const u8 as *const libc::c_char) != 0 {
            b"ztee\0" as *const u8 as *const libc::c_char
        } else {
            b"ztee\0" as *const u8 as *const libc::c_char
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
pub unsafe extern "C" fn cmdline_parser_string(
    mut cmdline: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
) -> libc::c_int {
    return cmdline_parser_string2(
        cmdline,
        args_info,
        prog_name,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
}
pub unsafe extern "C" fn cmdline_parser_string2(
    mut cmdline: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
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
    return cmdline_parser_string_ext(cmdline, args_info, prog_name, &mut params);
}
pub unsafe extern "C" fn cmdline_parser_string_ext(
    mut cmdline: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
    mut params: *mut cmdline_parser_params,
) -> libc::c_int {
    let mut argv_ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut result: libc::c_int = 0;
    let mut argc: libc::c_uint = 0;
    argc = cmdline_parser_create_argv(cmdline, &mut argv_ptr, prog_name);
    result = cmdline_parser_internal(
        argc as libc::c_int,
        argv_ptr,
        args_info,
        params,
        0 as *const libc::c_char,
    );
    if !argv_ptr.is_null() {
        free(argv_ptr as *mut libc::c_void);
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
unsafe extern "C" fn cmdline_parser_create_argv(
    mut cmdline_: *const libc::c_char,
    mut argv_ptr: *mut *mut *mut libc::c_char,
    mut prog_name: *const libc::c_char,
) -> libc::c_uint {
    let mut cmdline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0;
    let mut i: libc::c_int = 0;
    if !prog_name.is_null() {
        cmd_line_list_tmp = malloc(::std::mem::size_of::<line_list>() as libc::c_ulong)
            as *mut line_list;
        (*cmd_line_list_tmp).next = cmd_line_list;
        cmd_line_list = cmd_line_list_tmp;
        (*cmd_line_list).string_arg = gengetopt_strdup(prog_name);
        n = n.wrapping_add(1);
        n;
    }
    cmdline = gengetopt_strdup(cmdline_);
    p = cmdline;
    while !p.is_null() && strlen(p) != 0 {
        j = strcspn(p, b" \t\0" as *const u8 as *const libc::c_char);
        n = n.wrapping_add(1);
        n;
        if j != 0 && j < strlen(p) {
            *p.offset(j as isize) = '\0' as i32 as libc::c_char;
            cmd_line_list_tmp = malloc(
                ::std::mem::size_of::<line_list>() as libc::c_ulong,
            ) as *mut line_list;
            (*cmd_line_list_tmp).next = cmd_line_list;
            cmd_line_list = cmd_line_list_tmp;
            (*cmd_line_list).string_arg = gengetopt_strdup(p);
            p = p.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            p = p
                .offset(
                    strspn(p, b" \t\0" as *const u8 as *const libc::c_char) as isize,
                );
        } else {
            cmd_line_list_tmp = malloc(
                ::std::mem::size_of::<line_list>() as libc::c_ulong,
            ) as *mut line_list;
            (*cmd_line_list_tmp).next = cmd_line_list;
            cmd_line_list = cmd_line_list_tmp;
            (*cmd_line_list).string_arg = gengetopt_strdup(p);
            break;
        }
    }
    *argv_ptr = malloc(
        n
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    cmd_line_list_tmp = cmd_line_list;
    i = n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int {
        let ref mut fresh5 = *(*argv_ptr).offset(i as isize);
        *fresh5 = (*cmd_line_list_tmp).string_arg;
        cmd_line_list_tmp = (*cmd_line_list_tmp).next;
        i -= 1;
        i;
    }
    let ref mut fresh6 = *(*argv_ptr).offset(n as isize);
    *fresh6 = 0 as *mut libc::c_char;
    free(cmdline as *mut libc::c_void);
    return n as libc::c_uint;
}
pub unsafe extern "C" fn cmdline_parser_required(
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
