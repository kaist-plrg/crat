use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn blocklist_is_allowed(s_addr: uint32_t) -> libc::c_int;
    fn blocklist_init(
        allowlist: *mut libc::c_char,
        blocklist: *mut libc::c_char,
        allowlist_entries: *mut *mut libc::c_char,
        allowlist_entries_len: size_t,
        blocklist_entries: *mut *mut libc::c_char,
        blocklist_entries_len: size_t,
        ignore_invalid_hosts: libc::c_int,
    ) -> libc::c_int;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_init(
        stream: *mut FILE,
        level: LogLevel,
        syslog_enabled: libc::c_int,
        syslog_app: *const libc::c_char,
    ) -> libc::c_int;
    fn pbm_init() -> *mut *mut uint8_t;
    fn pbm_check(b: *mut *mut uint8_t, v: uint32_t) -> libc::c_int;
    fn pbm_set(b: *mut *mut uint8_t, v: uint32_t);
    fn cmdline_parser_ext(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        args_info: *mut gengetopt_args_info,
        params: *mut cmdline_parser_params,
    ) -> libc::c_int;
    fn cmdline_parser_print_help();
    fn cmdline_parser_print_version();
    fn cmdline_parser_params_create() -> *mut cmdline_parser_params;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type LogLevel = libc::c_uint;
pub const ZNUM_LOGLEVELS: LogLevel = 6;
pub const ZLOG_TRACE: LogLevel = 5;
pub const ZLOG_DEBUG: LogLevel = 4;
pub const ZLOG_INFO: LogLevel = 3;
pub const ZLOG_WARN: LogLevel = 2;
pub const ZLOG_ERROR: LogLevel = 1;
pub const ZLOG_FATAL: LogLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gengetopt_args_info {
    pub blocklist_file_arg: *mut libc::c_char,
    pub blocklist_file_orig: *mut libc::c_char,
    pub blocklist_file_help: *const libc::c_char,
    pub allowlist_file_arg: *mut libc::c_char,
    pub allowlist_file_orig: *mut libc::c_char,
    pub allowlist_file_help: *const libc::c_char,
    pub log_file_arg: *mut libc::c_char,
    pub log_file_orig: *mut libc::c_char,
    pub log_file_help: *const libc::c_char,
    pub verbosity_arg: libc::c_int,
    pub verbosity_orig: *mut libc::c_char,
    pub verbosity_help: *const libc::c_char,
    pub no_duplicate_checking_help: *const libc::c_char,
    pub ignore_blocklist_errors_help: *const libc::c_char,
    pub ignore_input_errors_help: *const libc::c_char,
    pub disable_syslog_help: *const libc::c_char,
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub blocklist_file_given: libc::c_uint,
    pub allowlist_file_given: libc::c_uint,
    pub log_file_given: libc::c_uint,
    pub verbosity_given: libc::c_uint,
    pub no_duplicate_checking_given: libc::c_uint,
    pub ignore_blocklist_errors_given: libc::c_uint,
    pub ignore_input_errors_given: libc::c_uint,
    pub disable_syslog_given: libc::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbl_conf {
    pub blocklist_filename: *mut libc::c_char,
    pub allowlist_filename: *mut libc::c_char,
    pub log_filename: *mut libc::c_char,
    pub check_duplicates: libc::c_int,
    pub ignore_blocklist_errors: libc::c_int,
    pub ignore_input_errors: libc::c_int,
    pub verbosity: libc::c_int,
    pub disable_syslog: libc::c_int,
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn zmin(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
) -> *mut libc::c_char {
    if !a.is_null() && b.is_null() {
        return a
    } else if !b.is_null() && a.is_null() {
        return b
    } else {
        return if a < b { a } else { b }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut conf: zbl_conf = zbl_conf {
        blocklist_filename: 0 as *mut libc::c_char,
        allowlist_filename: 0 as *mut libc::c_char,
        log_filename: 0 as *mut libc::c_char,
        check_duplicates: 0,
        ignore_blocklist_errors: 0,
        ignore_input_errors: 0,
        verbosity: 0,
        disable_syslog: 0,
    };
    conf.verbosity = 3 as libc::c_int;
    memset(
        &mut conf as *mut zbl_conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zbl_conf>() as libc::c_ulong,
    );
    let mut no_dupchk_pres: libc::c_int = 0 as libc::c_int;
    conf.ignore_blocklist_errors = 0 as libc::c_int;
    conf.ignore_input_errors = 0 as libc::c_int;
    let mut args: gengetopt_args_info = gengetopt_args_info {
        blocklist_file_arg: 0 as *mut libc::c_char,
        blocklist_file_orig: 0 as *mut libc::c_char,
        blocklist_file_help: 0 as *const libc::c_char,
        allowlist_file_arg: 0 as *mut libc::c_char,
        allowlist_file_orig: 0 as *mut libc::c_char,
        allowlist_file_help: 0 as *const libc::c_char,
        log_file_arg: 0 as *mut libc::c_char,
        log_file_orig: 0 as *mut libc::c_char,
        log_file_help: 0 as *const libc::c_char,
        verbosity_arg: 0,
        verbosity_orig: 0 as *mut libc::c_char,
        verbosity_help: 0 as *const libc::c_char,
        no_duplicate_checking_help: 0 as *const libc::c_char,
        ignore_blocklist_errors_help: 0 as *const libc::c_char,
        ignore_input_errors_help: 0 as *const libc::c_char,
        disable_syslog_help: 0 as *const libc::c_char,
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        blocklist_file_given: 0,
        allowlist_file_given: 0,
        log_file_given: 0,
        verbosity_given: 0,
        no_duplicate_checking_given: 0,
        ignore_blocklist_errors_given: 0,
        ignore_input_errors_given: 0,
        disable_syslog_given: 0,
        help_given: 0,
        version_given: 0,
    };
    let mut params: *mut cmdline_parser_params = 0 as *mut cmdline_parser_params;
    params = cmdline_parser_params_create();
    if !params.is_null() {} else {
        __assert_fail(
            b"params\0" as *const u8 as *const libc::c_char,
            b"zblocklist.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_6573: {
        if !params.is_null() {} else {
            __assert_fail(
                b"params\0" as *const u8 as *const libc::c_char,
                b"zblocklist.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    (*params).initialize = 1 as libc::c_int;
    (*params).override_0 = 0 as libc::c_int;
    (*params).check_required = 0 as libc::c_int;
    if cmdline_parser_ext(argc, argv, &mut args, params) != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    if args.help_given != 0 {
        cmdline_parser_print_help();
        exit(0 as libc::c_int);
    }
    if args.version_given != 0 {
        cmdline_parser_print_version();
        exit(0 as libc::c_int);
    }
    if args.log_file_given != 0 {
        conf.log_filename = strdup(args.log_file_arg);
    }
    if args.verbosity_given != 0 {
        conf.verbosity = args.verbosity_arg;
    }
    if args.blocklist_file_given != 0 {
        conf.blocklist_filename = strdup(args.blocklist_file_arg);
    }
    if args.allowlist_file_given != 0 {
        conf.allowlist_filename = strdup(args.allowlist_file_arg);
    }
    if args.no_duplicate_checking_given != 0 {
        no_dupchk_pres = 1 as libc::c_int;
    }
    conf.check_duplicates = (no_dupchk_pres == 0) as libc::c_int;
    if args.ignore_blocklist_errors_given != 0 {
        conf.ignore_blocklist_errors = 1 as libc::c_int;
    }
    if args.ignore_input_errors_given != 0 {
        conf.ignore_input_errors = 1 as libc::c_int;
    }
    if args.disable_syslog_given != 0 {
        conf.disable_syslog = 1 as libc::c_int;
    }
    let mut logfile: *mut FILE = stderr;
    if !(conf.log_filename).is_null() {
        logfile = fopen(conf.log_filename, b"w\0" as *const u8 as *const libc::c_char);
        if logfile.is_null() {
            fprintf(
                stderr,
                b"FATAL: unable to open specified logfile (%s)\n\0" as *const u8
                    as *const libc::c_char,
                conf.log_filename,
            );
            exit(1 as libc::c_int);
        }
    }
    if log_init(
        logfile,
        conf.verbosity as LogLevel,
        (conf.disable_syslog == 0) as libc::c_int,
        b"zblocklist\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"FATAL: unable able to initialize logging\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if (conf.blocklist_filename).is_null() && (conf.allowlist_filename).is_null() {
        log_fatal(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"must specify either a allowlist or blocklist file\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(conf.blocklist_filename).is_null() {
        log_debug(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"blocklist file at %s to be used\0" as *const u8 as *const libc::c_char,
            conf.blocklist_filename,
        );
    } else {
        log_debug(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"no blocklist file specified\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(conf.blocklist_filename).is_null()
        && access(conf.blocklist_filename, 4 as libc::c_int) == -(1 as libc::c_int)
    {
        log_fatal(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"unable to read specified blocklist file (%s)\0" as *const u8
                as *const libc::c_char,
            conf.blocklist_filename,
        );
    }
    if !(conf.allowlist_filename).is_null() {
        log_debug(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"allowlist file at %s to be used\0" as *const u8 as *const libc::c_char,
            conf.allowlist_filename,
        );
    } else {
        log_debug(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"no allowlist file specified\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(conf.allowlist_filename).is_null()
        && access(conf.allowlist_filename, 4 as libc::c_int) == -(1 as libc::c_int)
    {
        log_fatal(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"unable to read specified allowlist file (%s)\0" as *const u8
                as *const libc::c_char,
            conf.allowlist_filename,
        );
    }
    if blocklist_init(
        conf.allowlist_filename,
        conf.blocklist_filename,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        conf.ignore_blocklist_errors,
    ) != 0
    {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"unable to initialize blocklist / allowlist\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut seen: *mut *mut uint8_t = 0 as *mut *mut uint8_t;
    if conf.check_duplicates != 0 {
        seen = pbm_init();
        if seen.is_null() {
            log_fatal(
                b"zblocklist\0" as *const u8 as *const libc::c_char,
                b"unable to initialize paged bitmap\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    let mut line: *mut libc::c_char = malloc(
        (1024 as libc::c_int * 1024 as libc::c_int + 2 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if !line.is_null() {} else {
        __assert_fail(
            b"line\0" as *const u8 as *const libc::c_char,
            b"zblocklist.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_6103: {
        if !line.is_null() {} else {
            __assert_fail(
                b"line\0" as *const u8 as *const libc::c_char,
                b"zblocklist.c\0" as *const u8 as *const libc::c_char,
                207 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut original: *mut libc::c_char = malloc(
        (1024 as libc::c_int * 1024 as libc::c_int + 2 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if !original.is_null() {} else {
        __assert_fail(
            b"original\0" as *const u8 as *const libc::c_char,
            b"zblocklist.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_6070: {
        if !original.is_null() {} else {
            __assert_fail(
                b"original\0" as *const u8 as *const libc::c_char,
                b"zblocklist.c\0" as *const u8 as *const libc::c_char,
                209 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    while !(fgets(
        line,
        1024 as libc::c_int * 1024 as libc::c_int + 2 as libc::c_int,
        stdin,
    ))
        .is_null()
    {
        let mut len: size_t = strlen(line);
        if len
            >= (1024 as libc::c_int * 1024 as libc::c_int + 2 as libc::c_int
                - 1 as libc::c_int) as libc::c_ulong
        {
            log_fatal(
                b"zblocklist\0" as *const u8 as *const libc::c_char,
                b"received line longer than max length: %i\0" as *const u8
                    as *const libc::c_char,
                1024 as libc::c_int * 1024 as libc::c_int + 2 as libc::c_int,
            );
        }
        memcpy(
            original as *mut libc::c_void,
            line as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        let mut n: *mut libc::c_char = zmin(
            zmin(
                zmin(
                    zmin(strchr(line, '\n' as i32), strchr(line, ',' as i32)),
                    strchr(line, '\t' as i32),
                ),
                strchr(line, ' ' as i32),
            ),
            strchr(line, '#' as i32),
        );
        if !n.is_null() {} else {
            __assert_fail(
                b"n\0" as *const u8 as *const libc::c_char,
                b"zblocklist.c\0" as *const u8 as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_5965: {
            if !n.is_null() {} else {
                __assert_fail(
                    b"n\0" as *const u8 as *const libc::c_char,
                    b"zblocklist.c\0" as *const u8 as *const libc::c_char,
                    224 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        *n.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        log_debug(
            b"zblocklist\0" as *const u8 as *const libc::c_char,
            b"input value %s\0" as *const u8 as *const libc::c_char,
            line,
        );
        let mut addr: in_addr = in_addr { s_addr: 0 };
        if inet_aton(line, &mut addr) == 0 {
            log_warn(
                b"zblocklist\0" as *const u8 as *const libc::c_char,
                b"invalid input address: %s\0" as *const u8 as *const libc::c_char,
                line,
            );
            if conf.ignore_input_errors == 0 {
                printf(b"%s\0" as *const u8 as *const libc::c_char, original);
            }
        } else {
            if conf.check_duplicates != 0 {
                if pbm_check(seen, __bswap_32(addr.s_addr)) != 0 {
                    log_debug(
                        b"zblocklist\0" as *const u8 as *const libc::c_char,
                        b"%s is a duplicate: skipped\0" as *const u8
                            as *const libc::c_char,
                        line,
                    );
                    continue;
                } else {
                    log_debug(
                        b"zblocklist\0" as *const u8 as *const libc::c_char,
                        b"%s not a duplicate: skipped\0" as *const u8
                            as *const libc::c_char,
                        line,
                    );
                }
            } else {
                log_debug(
                    b"zblocklist\0" as *const u8 as *const libc::c_char,
                    b"no duplicate checking for %s\0" as *const u8
                        as *const libc::c_char,
                    line,
                );
            }
            if blocklist_is_allowed(addr.s_addr) != 0 {
                if conf.check_duplicates != 0 {
                    if pbm_check(seen, __bswap_32(addr.s_addr)) == 0 {
                        pbm_set(seen, __bswap_32(addr.s_addr));
                        printf(b"%s\0" as *const u8 as *const libc::c_char, original);
                    }
                } else {
                    printf(b"%s\0" as *const u8 as *const libc::c_char, original);
                }
            }
        }
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
