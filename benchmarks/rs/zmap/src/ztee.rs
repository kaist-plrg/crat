use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn lock_file(f: *mut FILE) -> libc::c_int;
    fn unlock_file(f: *mut FILE) -> libc::c_int;
    fn log_init(
        stream: *mut FILE,
        level: LogLevel,
        syslog_enabled: libc::c_int,
        syslog_app: *const libc::c_char,
    ) -> libc::c_int;
    fn now() -> libc::c_double;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_info(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn queue_init() -> *mut zqueue_t;
    fn is_empty(queue: *mut zqueue_t) -> libc::c_int;
    fn push_back(data: *mut libc::c_char, queue: *mut zqueue_t);
    fn pop_front_unsafe(queue: *mut zqueue_t) -> *mut znode_t;
    fn get_size(queue: *mut zqueue_t) -> size_t;
    fn time_string(
        time: uint32_t,
        est: libc::c_int,
        buf: *mut libc::c_char,
        len: size_t,
    );
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn csv_find_index(
        header: *mut libc::c_char,
        names: *mut *const libc::c_char,
        names_len: size_t,
    ) -> libc::c_int;
    fn csv_get_index(row: *mut libc::c_char, idx: size_t) -> *mut libc::c_char;
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
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub struct zqueue_node {
    pub data: *mut libc::c_char,
    pub prev: *mut zqueue_node,
    pub next: *mut zqueue_node,
}
pub type znode_t = zqueue_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zqueue {
    pub front: *mut zqueue_node,
    pub back: *mut zqueue_node,
    pub size: size_t,
    pub lock: pthread_mutex_t,
    pub empty: pthread_cond_t,
}
pub type zqueue_t = zqueue;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type file_format = libc::c_uint;
pub const FORMAT_RAW: file_format = 2;
pub const FORMAT_JSON: file_format = 1;
pub const FORMAT_CSV: file_format = 0;
pub type format_t = file_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ztee_conf {
    pub output_filename: *mut libc::c_char,
    pub status_updates_filename: *mut libc::c_char,
    pub log_file_name: *mut libc::c_char,
    pub output_file: *mut FILE,
    pub status_updates_file: *mut FILE,
    pub log_file: *mut FILE,
    pub log_level: libc::c_int,
    pub in_format: format_t,
    pub out_format: format_t,
    pub success_only: libc::c_int,
    pub monitor: libc::c_int,
    pub ip_field: size_t,
    pub success_field: size_t,
}
pub type ztee_conf_t = ztee_conf;
pub type stats_t = ztee_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ztee_stats {
    pub total_read: uint32_t,
    pub read_per_sec_avg: uint32_t,
    pub read_last_sec: uint32_t,
    pub buffer_cur_size: uint32_t,
    pub buffer_avg_size: uint32_t,
    pub _buffer_size_sum: uint64_t,
    pub _last_age: libc::c_double,
    pub time_past: uint32_t,
    pub time_past_str: [libc::c_char; 20],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut format_names: [*const libc::c_char; 3] = [
    b"csv\0" as *const u8 as *const libc::c_char,
    b"json\0" as *const u8 as *const libc::c_char,
    b"raw\0" as *const u8 as *const libc::c_char,
];
static mut tconf: ztee_conf_t = ztee_conf_t {
    output_filename: 0 as *const libc::c_char as *mut libc::c_char,
    status_updates_filename: 0 as *const libc::c_char as *mut libc::c_char,
    log_file_name: 0 as *const libc::c_char as *mut libc::c_char,
    output_file: 0 as *const FILE as *mut FILE,
    status_updates_file: 0 as *const FILE as *mut FILE,
    log_file: 0 as *const FILE as *mut FILE,
    log_level: 0,
    in_format: FORMAT_CSV,
    out_format: FORMAT_CSV,
    success_only: 0,
    monitor: 0,
    ip_field: 0,
    success_field: 0,
};
unsafe extern "C" fn test_input_format(
    mut line: *mut libc::c_char,
    mut len: size_t,
) -> format_t {
    if len < 2 as libc::c_int as libc::c_ulong {
        return FORMAT_RAW;
    }
    if len >= 3 as libc::c_int as libc::c_ulong {
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
            && *line.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '}' as i32
        {
            return FORMAT_JSON;
        }
    }
    if !(strchr(line, ',' as i32)).is_null() {
        return FORMAT_CSV;
    }
    return FORMAT_RAW;
}
pub static mut done: libc::c_int = 0 as libc::c_int;
pub static mut process_done: libc::c_int = 0 as libc::c_int;
pub static mut total_read_in: libc::c_int = 0 as libc::c_int;
pub static mut read_in_last_sec: libc::c_int = 0 as libc::c_int;
pub static mut total_written: libc::c_int = 0 as libc::c_int;
pub static mut start_time: libc::c_double = 0.;
pub static mut threads: [pthread_t; 3] = [0; 3];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut args: gengetopt_args_info = gengetopt_args_info {
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
    let mut params: *mut cmdline_parser_params = 0 as *mut cmdline_parser_params;
    params = cmdline_parser_params_create();
    if !params.is_null() {} else {
        __assert_fail(
            b"params\0" as *const u8 as *const libc::c_char,
            b"ztee.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_6139: {
        if !params.is_null() {} else {
            __assert_fail(
                b"params\0" as *const u8 as *const libc::c_char,
                b"ztee.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
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
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if args.help_given != 0 {
        cmdline_parser_print_help();
        exit(0 as libc::c_int);
    }
    if args.version_given != 0 {
        cmdline_parser_print_version();
        exit(0 as libc::c_int);
    }
    tconf.log_level = ZLOG_WARN as libc::c_int;
    if args.log_file_given != 0 {
        tconf
            .log_file = fopen(
            args.log_file_arg,
            b"w\0" as *const u8 as *const libc::c_char,
        );
    } else {
        tconf.log_file = stderr;
    }
    if (tconf.log_file).is_null() {
        log_init(
            stderr,
            tconf.log_level as LogLevel,
            0 as libc::c_int,
            b"ztee\0" as *const u8 as *const libc::c_char,
        );
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"Could not open log file\0" as *const u8 as *const libc::c_char,
        );
    }
    log_init(
        tconf.log_file,
        tconf.log_level as LogLevel,
        0 as libc::c_int,
        b"ztee\0" as *const u8 as *const libc::c_char,
    );
    if args.inputs_num < 1 as libc::c_int as libc::c_uint {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"No output file specified\0" as *const u8 as *const libc::c_char,
        );
    }
    if args.inputs_num > 1 as libc::c_int as libc::c_uint {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"Extra positional arguments starting with %s\0" as *const u8
                as *const libc::c_char,
            *(args.inputs).offset(1 as libc::c_int as isize),
        );
    }
    tconf.output_filename = *(args.inputs).offset(0 as libc::c_int as isize);
    tconf
        .output_file = fopen(
        tconf.output_filename,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if (tconf.output_file).is_null() {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"Could not open output file %s, %s\0" as *const u8 as *const libc::c_char,
            tconf.output_filename,
            strerror(*__errno_location()),
        );
    }
    let mut raw: libc::c_int = 0 as libc::c_int;
    if args.success_only_given != 0 {
        tconf.success_only = 1 as libc::c_int;
    }
    if args.monitor_given != 0 {
        tconf.monitor = 1 as libc::c_int;
    }
    if args.raw_given != 0 {
        raw = 1 as libc::c_int;
    }
    if args.status_updates_file_given != 0 {
        let mut filename: *mut libc::c_char = args.status_updates_file_arg;
        let mut file: *mut FILE = fopen(
            filename,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if file.is_null() {
            let mut err: *mut libc::c_char = strerror(*__errno_location());
            log_fatal(
                b"ztee\0" as *const u8 as *const libc::c_char,
                b"unable to open status updates file %s (%s)\0" as *const u8
                    as *const libc::c_char,
                filename,
                err,
            );
        }
        tconf.status_updates_filename = filename;
        tconf.status_updates_file = file;
    }
    let mut first_line_len: size_t = 1024 as libc::c_int as size_t;
    let mut first_line: *mut libc::c_char = xmalloc(first_line_len) as *mut libc::c_char;
    if getline(&mut first_line, &mut first_line_len, stdin)
        < 0 as libc::c_int as libc::c_long
    {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"reading input to test format failed\0" as *const u8 as *const libc::c_char,
        );
    }
    if raw == 0 {
        let mut format: format_t = test_input_format(first_line, first_line_len);
        log_info(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"detected input format %s\0" as *const u8 as *const libc::c_char,
            format_names[format as usize],
        );
        tconf.in_format = format;
    } else {
        tconf.in_format = FORMAT_RAW;
        log_info(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"raw input\0" as *const u8 as *const libc::c_char,
        );
    }
    if tconf.in_format as libc::c_uint == FORMAT_JSON as libc::c_int as libc::c_uint {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"json input not implemented\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut header: *mut libc::c_char = strdup(first_line);
    let mut found_success: libc::c_int = 0 as libc::c_int;
    let mut found_ip: libc::c_int = 0 as libc::c_int;
    if tconf.in_format as libc::c_uint == FORMAT_CSV as libc::c_int as libc::c_uint {
        static mut success_names: [*const libc::c_char; 1] = [
            b"success\0" as *const u8 as *const libc::c_char,
        ];
        static mut ip_names: [*const libc::c_char; 2] = [
            b"saddr\0" as *const u8 as *const libc::c_char,
            b"ip\0" as *const u8 as *const libc::c_char,
        ];
        let mut success_idx: libc::c_int = csv_find_index(
            header,
            success_names.as_mut_ptr(),
            1 as libc::c_int as size_t,
        );
        if success_idx >= 0 as libc::c_int {
            found_success = 1 as libc::c_int;
            tconf.success_field = success_idx as size_t;
        }
        let mut ip_idx: libc::c_int = csv_find_index(
            header,
            ip_names.as_mut_ptr(),
            2 as libc::c_int as size_t,
        );
        if found_ip >= 0 as libc::c_int {
            found_ip = 1 as libc::c_int;
            tconf.ip_field = ip_idx as size_t;
        }
        if found_ip == 0 {
            log_fatal(
                b"ztee\0" as *const u8 as *const libc::c_char,
                b"Unable to find IP/SADDR field\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if tconf.success_only != 0 {
        if tconf.in_format as libc::c_uint != FORMAT_CSV as libc::c_int as libc::c_uint {
            log_fatal(
                b"ztee\0" as *const u8 as *const libc::c_char,
                b"success filter requires csv input\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if found_success == 0 {
            log_fatal(
                b"ztee\0" as *const u8 as *const libc::c_char,
                b"Could not find success field\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let mut queue: *mut zqueue_t = queue_init();
    if !queue.is_null() {} else {
        __assert_fail(
            b"queue\0" as *const u8 as *const libc::c_char,
            b"ztee.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_5535: {
        if !queue.is_null() {} else {
            __assert_fail(
                b"queue\0" as *const u8 as *const libc::c_char,
                b"ztee.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    push_back(first_line, queue);
    let mut read_thread: pthread_t = 0;
    if pthread_create(
        &mut read_thread,
        0 as *const pthread_attr_t,
        Some(read_in as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        queue as *mut libc::c_void,
    ) != 0
    {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"unable to start read thread\0" as *const u8 as *const libc::c_char,
        );
    }
    start_time = now();
    let mut process_thread: pthread_t = 0;
    if pthread_create(
        &mut process_thread,
        0 as *const pthread_attr_t,
        Some(
            process_queue as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        queue as *mut libc::c_void,
    ) != 0
    {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"unable to start process thread\0" as *const u8 as *const libc::c_char,
        );
    }
    if tconf.monitor != 0 || !(tconf.status_updates_file).is_null() {
        let mut monitor_thread: pthread_t = 0;
        if pthread_create(
            &mut monitor_thread,
            0 as *const pthread_attr_t,
            Some(
                monitor_ztee
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            queue as *mut libc::c_void,
        ) != 0
        {
            log_fatal(
                b"ztee\0" as *const u8 as *const libc::c_char,
                b"unable to create monitor thread\0" as *const u8 as *const libc::c_char,
            );
        }
        pthread_join(monitor_thread, 0 as *mut *mut libc::c_void);
    }
    pthread_join(read_thread, 0 as *mut *mut libc::c_void);
    pthread_join(process_thread, 0 as *mut *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn process_queue(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut queue: *mut zqueue_t = arg as *mut zqueue_t;
    let mut output_file: *mut FILE = tconf.output_file;
    while process_done == 0 {
        pthread_mutex_lock(&mut (*queue).lock);
        while done == 0 && is_empty(queue) != 0 {
            pthread_cond_wait(&mut (*queue).empty, &mut (*queue).lock);
        }
        if done != 0 && is_empty(queue) != 0 {
            process_done = 1 as libc::c_int;
            pthread_mutex_unlock(&mut (*queue).lock);
        } else {
            let mut node: *mut znode_t = pop_front_unsafe(queue);
            pthread_mutex_unlock(&mut (*queue).lock);
            fprintf(
                output_file,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*node).data,
            );
            fflush(output_file);
            if ferror(output_file) != 0 {
                log_fatal(
                    b"ztee\0" as *const u8 as *const libc::c_char,
                    b"Error writing to output file\0" as *const u8 as *const libc::c_char,
                );
            }
            match tconf.in_format as libc::c_uint {
                1 => {
                    log_fatal(
                        b"ztee\0" as *const u8 as *const libc::c_char,
                        b"JSON input format unimplemented\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                0 => {
                    print_from_csv((*node).data);
                }
                _ => {
                    fprintf(
                        stdout,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*node).data,
                    );
                }
            }
            fflush(stdout);
            if ferror(stdout) != 0 {
                log_fatal(
                    b"ztee\0" as *const u8 as *const libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"Error writing to stdout\0" as *const u8 as *const libc::c_char,
                );
            }
            total_written += 1;
            total_written;
            free((*node).data as *mut libc::c_void);
            free(node as *mut libc::c_void);
        }
    }
    process_done = 1 as libc::c_int;
    fflush(output_file);
    fclose(output_file);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn read_in(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut queue: *mut zqueue_t = arg as *mut zqueue_t;
    let mut length: size_t = 1000 as libc::c_int as size_t;
    let mut input: *mut libc::c_char = xcalloc(
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        length,
    ) as *mut libc::c_char;
    while getline(&mut input, &mut length, stdin) > 0 as libc::c_int as libc::c_long {
        push_back(input, queue);
        total_read_in += 1;
        total_read_in;
        read_in_last_sec += 1;
        read_in_last_sec;
    }
    pthread_mutex_lock(&mut (*queue).lock);
    done = 1 as libc::c_int;
    pthread_cond_signal(&mut (*queue).empty);
    pthread_mutex_unlock(&mut (*queue).lock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn print_from_csv(mut line: *mut libc::c_char) -> libc::c_int {
    if total_written == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if tconf.success_only != 0 {
        let mut success_entry: *mut libc::c_char = csv_get_index(
            line,
            tconf.success_field,
        );
        if success_entry.is_null() {
            return 1 as libc::c_int;
        }
        let mut success: libc::c_int = 0 as libc::c_int;
        if atoi(success_entry) != 0 {
            success = 1 as libc::c_int;
        } else if strcasecmp(
            success_entry,
            b"true\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            success = 1 as libc::c_int;
        }
        if success == 0 {
            return 1 as libc::c_int;
        }
    }
    let mut ip: *mut libc::c_char = csv_get_index(line, tconf.ip_field);
    let mut ret: libc::c_int = fprintf(
        stdout,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        ip,
    );
    if ferror(stdout) != 0 {
        log_fatal(
            b"ztee\0" as *const u8 as *const libc::c_char,
            b"unable to write to stdout\0" as *const u8 as *const libc::c_char,
        );
    }
    return ret;
}
pub unsafe extern "C" fn output_file_is_csv() {}
pub unsafe extern "C" fn print_thread_error(mut string: *mut libc::c_char) {
    fprintf(
        stderr,
        b"Could not create thread %s\n\0" as *const u8 as *const libc::c_char,
        string,
    );
}
pub unsafe extern "C" fn update_stats(
    mut stats: *mut stats_t,
    mut queue: *mut zqueue_t,
) {
    let mut age: libc::c_double = now() - start_time;
    let mut delta: libc::c_double = age - (*stats)._last_age;
    (*stats)._last_age = age;
    (*stats).time_past = age as uint32_t;
    time_string(
        age as libc::c_int as uint32_t,
        0 as libc::c_int,
        ((*stats).time_past_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    let mut total_read: uint32_t = total_read_in as uint32_t;
    (*stats)
        .read_last_sec = (total_read.wrapping_sub((*stats).total_read) as libc::c_double
        / delta) as uint32_t;
    (*stats).total_read = total_read;
    (*stats)
        .read_per_sec_avg = ((*stats).total_read as libc::c_double / age) as uint32_t;
    (*stats).buffer_cur_size = get_size(queue) as uint32_t;
    (*stats)
        ._buffer_size_sum = ((*stats)._buffer_size_sum as libc::c_ulong)
        .wrapping_add((*stats).buffer_cur_size as libc::c_ulong) as uint64_t as uint64_t;
    (*stats)
        .buffer_avg_size = ((*stats)._buffer_size_sum as libc::c_double / age)
        as uint32_t;
}
pub unsafe extern "C" fn monitor_ztee(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut queue: *mut zqueue_t = arg as *mut zqueue_t;
    let mut stats: *mut stats_t = xmalloc(
        ::std::mem::size_of::<stats_t>() as libc::c_ulong,
    ) as *mut stats_t;
    if !(tconf.status_updates_file).is_null() {
        fprintf(
            tconf.status_updates_file,
            b"time_past,total_read_in,read_in_last_sec,read_per_sec_avg,buffer_current_size,buffer_avg_size\n\0"
                as *const u8 as *const libc::c_char,
        );
        fflush(tconf.status_updates_file);
        if ferror(tconf.status_updates_file) != 0 {
            log_fatal(
                b"ztee\0" as *const u8 as *const libc::c_char,
                b"unable to write to status updates file\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    while process_done == 0 {
        sleep(1 as libc::c_int as libc::c_uint);
        update_stats(stats, queue);
        if tconf.monitor != 0 {
            lock_file(stderr);
            fprintf(
                stderr,
                b"%5s read_rate: %u rows/s (avg %u rows/s), buffer_size: %u (avg %u)\n\0"
                    as *const u8 as *const libc::c_char,
                ((*stats).time_past_str).as_mut_ptr(),
                (*stats).read_last_sec,
                (*stats).read_per_sec_avg,
                (*stats).buffer_cur_size,
                (*stats).buffer_avg_size,
            );
            fflush(stderr);
            unlock_file(stderr);
            if ferror(stderr) != 0 {
                log_fatal(
                    b"ztee\0" as *const u8 as *const libc::c_char,
                    b"unable to write status updates to stderr\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        if !(tconf.status_updates_file).is_null() {
            fprintf(
                tconf.status_updates_file,
                b"%u,%u,%u,%u,%u,%u\n\0" as *const u8 as *const libc::c_char,
                (*stats).time_past,
                (*stats).total_read,
                (*stats).read_last_sec,
                (*stats).read_per_sec_avg,
                (*stats).buffer_cur_size,
                (*stats).buffer_avg_size,
            );
            fflush(tconf.status_updates_file);
            if ferror(tconf.status_updates_file) != 0 {
                log_fatal(
                    b"ztee\0" as *const u8 as *const libc::c_char,
                    b"unable to write to status updates file\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if tconf.monitor != 0 {
        lock_file(stderr);
        fflush(stderr);
        unlock_file(stderr);
    }
    if !(tconf.status_updates_file).is_null() {
        fflush(tconf.status_updates_file);
        fclose(tconf.status_updates_file);
    }
    return 0 as *mut libc::c_void;
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
