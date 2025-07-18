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
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    pub target_port_arg: libc::c_int,
    pub target_port_orig: *mut libc::c_char,
    pub target_port_help: *const libc::c_char,
    pub output_file_arg: *mut libc::c_char,
    pub output_file_orig: *mut libc::c_char,
    pub output_file_help: *const libc::c_char,
    pub blocklist_file_arg: *mut libc::c_char,
    pub blocklist_file_orig: *mut libc::c_char,
    pub blocklist_file_help: *const libc::c_char,
    pub allowlist_file_arg: *mut libc::c_char,
    pub allowlist_file_orig: *mut libc::c_char,
    pub allowlist_file_help: *const libc::c_char,
    pub list_of_ips_file_arg: *mut libc::c_char,
    pub list_of_ips_file_orig: *mut libc::c_char,
    pub list_of_ips_file_help: *const libc::c_char,
    pub rate_arg: libc::c_int,
    pub rate_orig: *mut libc::c_char,
    pub rate_help: *const libc::c_char,
    pub bandwidth_arg: *mut libc::c_char,
    pub bandwidth_orig: *mut libc::c_char,
    pub bandwidth_help: *const libc::c_char,
    pub batch_arg: libc::c_int,
    pub batch_orig: *mut libc::c_char,
    pub batch_help: *const libc::c_char,
    pub max_targets_arg: *mut libc::c_char,
    pub max_targets_orig: *mut libc::c_char,
    pub max_targets_help: *const libc::c_char,
    pub max_runtime_arg: libc::c_int,
    pub max_runtime_orig: *mut libc::c_char,
    pub max_runtime_help: *const libc::c_char,
    pub max_results_arg: libc::c_int,
    pub max_results_orig: *mut libc::c_char,
    pub max_results_help: *const libc::c_char,
    pub probes_arg: libc::c_int,
    pub probes_orig: *mut libc::c_char,
    pub probes_help: *const libc::c_char,
    pub cooldown_time_arg: libc::c_int,
    pub cooldown_time_orig: *mut libc::c_char,
    pub cooldown_time_help: *const libc::c_char,
    pub seed_arg: libc::c_long,
    pub seed_orig: *mut libc::c_char,
    pub seed_help: *const libc::c_char,
    pub retries_arg: libc::c_int,
    pub retries_orig: *mut libc::c_char,
    pub retries_help: *const libc::c_char,
    pub dryrun_help: *const libc::c_char,
    pub shards_arg: libc::c_int,
    pub shards_orig: *mut libc::c_char,
    pub shards_help: *const libc::c_char,
    pub shard_arg: libc::c_int,
    pub shard_orig: *mut libc::c_char,
    pub shard_help: *const libc::c_char,
    pub source_port_arg: *mut libc::c_char,
    pub source_port_orig: *mut libc::c_char,
    pub source_port_help: *const libc::c_char,
    pub source_ip_arg: *mut libc::c_char,
    pub source_ip_orig: *mut libc::c_char,
    pub source_ip_help: *const libc::c_char,
    pub gateway_mac_arg: *mut libc::c_char,
    pub gateway_mac_orig: *mut libc::c_char,
    pub gateway_mac_help: *const libc::c_char,
    pub source_mac_arg: *mut libc::c_char,
    pub source_mac_orig: *mut libc::c_char,
    pub source_mac_help: *const libc::c_char,
    pub interface_arg: *mut libc::c_char,
    pub interface_orig: *mut libc::c_char,
    pub interface_help: *const libc::c_char,
    pub iplayer_help: *const libc::c_char,
    pub probe_module_arg: *mut libc::c_char,
    pub probe_module_orig: *mut libc::c_char,
    pub probe_module_help: *const libc::c_char,
    pub probe_args_arg: *mut libc::c_char,
    pub probe_args_orig: *mut libc::c_char,
    pub probe_args_help: *const libc::c_char,
    pub probe_ttl_arg: libc::c_int,
    pub probe_ttl_orig: *mut libc::c_char,
    pub probe_ttl_help: *const libc::c_char,
    pub list_probe_modules_help: *const libc::c_char,
    pub output_fields_arg: *mut libc::c_char,
    pub output_fields_orig: *mut libc::c_char,
    pub output_fields_help: *const libc::c_char,
    pub output_module_arg: *mut libc::c_char,
    pub output_module_orig: *mut libc::c_char,
    pub output_module_help: *const libc::c_char,
    pub output_args_arg: *mut libc::c_char,
    pub output_args_orig: *mut libc::c_char,
    pub output_args_help: *const libc::c_char,
    pub output_filter_arg: *mut libc::c_char,
    pub output_filter_orig: *mut libc::c_char,
    pub output_filter_help: *const libc::c_char,
    pub list_output_modules_help: *const libc::c_char,
    pub list_output_fields_help: *const libc::c_char,
    pub no_header_row_help: *const libc::c_char,
    pub verbosity_arg: libc::c_int,
    pub verbosity_orig: *mut libc::c_char,
    pub verbosity_help: *const libc::c_char,
    pub log_file_arg: *mut libc::c_char,
    pub log_file_orig: *mut libc::c_char,
    pub log_file_help: *const libc::c_char,
    pub log_directory_arg: *mut libc::c_char,
    pub log_directory_orig: *mut libc::c_char,
    pub log_directory_help: *const libc::c_char,
    pub metadata_file_arg: *mut libc::c_char,
    pub metadata_file_orig: *mut libc::c_char,
    pub metadata_file_help: *const libc::c_char,
    pub status_updates_file_arg: *mut libc::c_char,
    pub status_updates_file_orig: *mut libc::c_char,
    pub status_updates_file_help: *const libc::c_char,
    pub quiet_help: *const libc::c_char,
    pub disable_syslog_help: *const libc::c_char,
    pub notes_arg: *mut libc::c_char,
    pub notes_orig: *mut libc::c_char,
    pub notes_help: *const libc::c_char,
    pub user_metadata_arg: *mut libc::c_char,
    pub user_metadata_orig: *mut libc::c_char,
    pub user_metadata_help: *const libc::c_char,
    pub config_arg: *mut libc::c_char,
    pub config_orig: *mut libc::c_char,
    pub config_help: *const libc::c_char,
    pub max_sendto_failures_arg: libc::c_int,
    pub max_sendto_failures_orig: *mut libc::c_char,
    pub max_sendto_failures_help: *const libc::c_char,
    pub min_hitrate_arg: libc::c_float,
    pub min_hitrate_orig: *mut libc::c_char,
    pub min_hitrate_help: *const libc::c_char,
    pub sender_threads_arg: libc::c_int,
    pub sender_threads_orig: *mut libc::c_char,
    pub sender_threads_help: *const libc::c_char,
    pub cores_arg: *mut libc::c_char,
    pub cores_orig: *mut libc::c_char,
    pub cores_help: *const libc::c_char,
    pub ignore_blocklist_errors_help: *const libc::c_char,
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub target_port_given: libc::c_uint,
    pub output_file_given: libc::c_uint,
    pub blocklist_file_given: libc::c_uint,
    pub allowlist_file_given: libc::c_uint,
    pub list_of_ips_file_given: libc::c_uint,
    pub rate_given: libc::c_uint,
    pub bandwidth_given: libc::c_uint,
    pub batch_given: libc::c_uint,
    pub max_targets_given: libc::c_uint,
    pub max_runtime_given: libc::c_uint,
    pub max_results_given: libc::c_uint,
    pub probes_given: libc::c_uint,
    pub cooldown_time_given: libc::c_uint,
    pub seed_given: libc::c_uint,
    pub retries_given: libc::c_uint,
    pub dryrun_given: libc::c_uint,
    pub shards_given: libc::c_uint,
    pub shard_given: libc::c_uint,
    pub source_port_given: libc::c_uint,
    pub source_ip_given: libc::c_uint,
    pub gateway_mac_given: libc::c_uint,
    pub source_mac_given: libc::c_uint,
    pub interface_given: libc::c_uint,
    pub iplayer_given: libc::c_uint,
    pub probe_module_given: libc::c_uint,
    pub probe_args_given: libc::c_uint,
    pub probe_ttl_given: libc::c_uint,
    pub list_probe_modules_given: libc::c_uint,
    pub output_fields_given: libc::c_uint,
    pub output_module_given: libc::c_uint,
    pub output_args_given: libc::c_uint,
    pub output_filter_given: libc::c_uint,
    pub list_output_modules_given: libc::c_uint,
    pub list_output_fields_given: libc::c_uint,
    pub no_header_row_given: libc::c_uint,
    pub verbosity_given: libc::c_uint,
    pub log_file_given: libc::c_uint,
    pub log_directory_given: libc::c_uint,
    pub metadata_file_given: libc::c_uint,
    pub status_updates_file_given: libc::c_uint,
    pub quiet_given: libc::c_uint,
    pub disable_syslog_given: libc::c_uint,
    pub notes_given: libc::c_uint,
    pub user_metadata_given: libc::c_uint,
    pub config_given: libc::c_uint,
    pub max_sendto_failures_given: libc::c_uint,
    pub min_hitrate_given: libc::c_uint,
    pub sender_threads_given: libc::c_uint,
    pub cores_given: libc::c_uint,
    pub ignore_blocklist_errors_given: libc::c_uint,
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
pub const ARG_LONGLONG: cmdline_parser_arg_type = 4;
pub const ARG_FLOAT: cmdline_parser_arg_type = 3;
pub const ARG_INT: cmdline_parser_arg_type = 2;
pub const ARG_STRING: cmdline_parser_arg_type = 1;
pub const ARG_NO: cmdline_parser_arg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_list {
    pub string_arg: *mut libc::c_char,
    pub next: *mut line_list,
}
pub unsafe extern "C" fn cmdline_parser_required(
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
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
            b"zmap\0" as *const u8 as *const libc::c_char,
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
                b"zmap\0" as *const u8 as *const libc::c_char,
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
                let fresh0 = next_token;
                next_token = next_token.wrapping_add(1);
                *fopt.offset(fresh0 as isize) = '\0' as i32 as libc::c_char;
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
                            b"zmap\0" as *const u8 as *const libc::c_char,
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
                let fresh1 = str_index;
                str_index = str_index.offset(1);
                *fresh1 = '\0' as i32 as libc::c_char;
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
                            b"zmap\0" as *const u8 as *const libc::c_char,
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
                        b"zmap\0" as *const u8 as *const libc::c_char,
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
static mut cmd_line_list: *mut line_list = 0 as *const line_list as *mut line_list;
static mut cmd_line_list_tmp: *mut line_list = 0 as *const line_list as *mut line_list;
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
        .string_arg = gengetopt_strdup(b"zmap\0" as *const u8 as *const libc::c_char);
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
            let ref mut fresh2 = *my_argv_arg.offset(i as isize);
            *fresh2 = (*cmd_line_list_tmp).string_arg;
            cmd_line_list_tmp = (*cmd_line_list_tmp).next;
            i -= 1;
            i;
        }
        let ref mut fresh3 = *my_argv_arg.offset(my_argc as isize);
        *fresh3 = 0 as *mut libc::c_char;
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
pub unsafe extern "C" fn cmdline_parser_params_create() -> *mut cmdline_parser_params {
    let mut params: *mut cmdline_parser_params = malloc(
        ::std::mem::size_of::<cmdline_parser_params>() as libc::c_ulong,
    ) as *mut cmdline_parser_params;
    cmdline_parser_params_init(params);
    return params;
}
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
pub static mut gengetopt_args_info_versiontext: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn cmdline_parser_print_version() {
    printf(
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        if strlen(b"zmap\0" as *const u8 as *const libc::c_char) != 0 {
            b"zmap\0" as *const u8 as *const libc::c_char
        } else {
            b"zmap\0" as *const u8 as *const libc::c_char
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
            b"zmap\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    i = cmdline_parser_dump(outfile, args_info);
    fclose(outfile);
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
pub unsafe extern "C" fn cmdline_parser_dump(
    mut outfile: *mut FILE,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if outfile.is_null() {
        fprintf(
            stderr,
            b"%s: cannot dump options to stream\n\0" as *const u8 as *const libc::c_char,
            b"zmap\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if (*args_info).target_port_given != 0 {
        write_into_file(
            outfile,
            b"target-port\0" as *const u8 as *const libc::c_char,
            (*args_info).target_port_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_file_given != 0 {
        write_into_file(
            outfile,
            b"output-file\0" as *const u8 as *const libc::c_char,
            (*args_info).output_file_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).blocklist_file_given != 0 {
        write_into_file(
            outfile,
            b"blocklist-file\0" as *const u8 as *const libc::c_char,
            (*args_info).blocklist_file_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).allowlist_file_given != 0 {
        write_into_file(
            outfile,
            b"allowlist-file\0" as *const u8 as *const libc::c_char,
            (*args_info).allowlist_file_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_of_ips_file_given != 0 {
        write_into_file(
            outfile,
            b"list-of-ips-file\0" as *const u8 as *const libc::c_char,
            (*args_info).list_of_ips_file_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).rate_given != 0 {
        write_into_file(
            outfile,
            b"rate\0" as *const u8 as *const libc::c_char,
            (*args_info).rate_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).bandwidth_given != 0 {
        write_into_file(
            outfile,
            b"bandwidth\0" as *const u8 as *const libc::c_char,
            (*args_info).bandwidth_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).batch_given != 0 {
        write_into_file(
            outfile,
            b"batch\0" as *const u8 as *const libc::c_char,
            (*args_info).batch_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_targets_given != 0 {
        write_into_file(
            outfile,
            b"max-targets\0" as *const u8 as *const libc::c_char,
            (*args_info).max_targets_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_runtime_given != 0 {
        write_into_file(
            outfile,
            b"max-runtime\0" as *const u8 as *const libc::c_char,
            (*args_info).max_runtime_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_results_given != 0 {
        write_into_file(
            outfile,
            b"max-results\0" as *const u8 as *const libc::c_char,
            (*args_info).max_results_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probes_given != 0 {
        write_into_file(
            outfile,
            b"probes\0" as *const u8 as *const libc::c_char,
            (*args_info).probes_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).cooldown_time_given != 0 {
        write_into_file(
            outfile,
            b"cooldown-time\0" as *const u8 as *const libc::c_char,
            (*args_info).cooldown_time_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).seed_given != 0 {
        write_into_file(
            outfile,
            b"seed\0" as *const u8 as *const libc::c_char,
            (*args_info).seed_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).retries_given != 0 {
        write_into_file(
            outfile,
            b"retries\0" as *const u8 as *const libc::c_char,
            (*args_info).retries_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).dryrun_given != 0 {
        write_into_file(
            outfile,
            b"dryrun\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).shards_given != 0 {
        write_into_file(
            outfile,
            b"shards\0" as *const u8 as *const libc::c_char,
            (*args_info).shards_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).shard_given != 0 {
        write_into_file(
            outfile,
            b"shard\0" as *const u8 as *const libc::c_char,
            (*args_info).shard_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).source_port_given != 0 {
        write_into_file(
            outfile,
            b"source-port\0" as *const u8 as *const libc::c_char,
            (*args_info).source_port_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).source_ip_given != 0 {
        write_into_file(
            outfile,
            b"source-ip\0" as *const u8 as *const libc::c_char,
            (*args_info).source_ip_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).gateway_mac_given != 0 {
        write_into_file(
            outfile,
            b"gateway-mac\0" as *const u8 as *const libc::c_char,
            (*args_info).gateway_mac_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).source_mac_given != 0 {
        write_into_file(
            outfile,
            b"source-mac\0" as *const u8 as *const libc::c_char,
            (*args_info).source_mac_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).interface_given != 0 {
        write_into_file(
            outfile,
            b"interface\0" as *const u8 as *const libc::c_char,
            (*args_info).interface_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).iplayer_given != 0 {
        write_into_file(
            outfile,
            b"iplayer\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probe_module_given != 0 {
        write_into_file(
            outfile,
            b"probe-module\0" as *const u8 as *const libc::c_char,
            (*args_info).probe_module_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probe_args_given != 0 {
        write_into_file(
            outfile,
            b"probe-args\0" as *const u8 as *const libc::c_char,
            (*args_info).probe_args_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probe_ttl_given != 0 {
        write_into_file(
            outfile,
            b"probe-ttl\0" as *const u8 as *const libc::c_char,
            (*args_info).probe_ttl_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_probe_modules_given != 0 {
        write_into_file(
            outfile,
            b"list-probe-modules\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_fields_given != 0 {
        write_into_file(
            outfile,
            b"output-fields\0" as *const u8 as *const libc::c_char,
            (*args_info).output_fields_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_module_given != 0 {
        write_into_file(
            outfile,
            b"output-module\0" as *const u8 as *const libc::c_char,
            (*args_info).output_module_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_args_given != 0 {
        write_into_file(
            outfile,
            b"output-args\0" as *const u8 as *const libc::c_char,
            (*args_info).output_args_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_filter_given != 0 {
        write_into_file(
            outfile,
            b"output-filter\0" as *const u8 as *const libc::c_char,
            (*args_info).output_filter_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_output_modules_given != 0 {
        write_into_file(
            outfile,
            b"list-output-modules\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_output_fields_given != 0 {
        write_into_file(
            outfile,
            b"list-output-fields\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).no_header_row_given != 0 {
        write_into_file(
            outfile,
            b"no-header-row\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).verbosity_given != 0 {
        write_into_file(
            outfile,
            b"verbosity\0" as *const u8 as *const libc::c_char,
            (*args_info).verbosity_orig,
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
    if (*args_info).log_directory_given != 0 {
        write_into_file(
            outfile,
            b"log-directory\0" as *const u8 as *const libc::c_char,
            (*args_info).log_directory_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).metadata_file_given != 0 {
        write_into_file(
            outfile,
            b"metadata-file\0" as *const u8 as *const libc::c_char,
            (*args_info).metadata_file_orig,
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
    if (*args_info).quiet_given != 0 {
        write_into_file(
            outfile,
            b"quiet\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).disable_syslog_given != 0 {
        write_into_file(
            outfile,
            b"disable-syslog\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).notes_given != 0 {
        write_into_file(
            outfile,
            b"notes\0" as *const u8 as *const libc::c_char,
            (*args_info).notes_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).user_metadata_given != 0 {
        write_into_file(
            outfile,
            b"user-metadata\0" as *const u8 as *const libc::c_char,
            (*args_info).user_metadata_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).config_given != 0 {
        write_into_file(
            outfile,
            b"config\0" as *const u8 as *const libc::c_char,
            (*args_info).config_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_sendto_failures_given != 0 {
        write_into_file(
            outfile,
            b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
            (*args_info).max_sendto_failures_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).min_hitrate_given != 0 {
        write_into_file(
            outfile,
            b"min-hitrate\0" as *const u8 as *const libc::c_char,
            (*args_info).min_hitrate_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).sender_threads_given != 0 {
        write_into_file(
            outfile,
            b"sender-threads\0" as *const u8 as *const libc::c_char,
            (*args_info).sender_threads_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).cores_given != 0 {
        write_into_file(
            outfile,
            b"cores\0" as *const u8 as *const libc::c_char,
            (*args_info).cores_orig,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).ignore_blocklist_errors_given != 0 {
        write_into_file(
            outfile,
            b"ignore-blocklist-errors\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn clear_args(mut args_info: *mut gengetopt_args_info) {
    (*args_info).target_port_orig = 0 as *mut libc::c_char;
    (*args_info).output_file_arg = 0 as *mut libc::c_char;
    (*args_info).output_file_orig = 0 as *mut libc::c_char;
    (*args_info).blocklist_file_arg = 0 as *mut libc::c_char;
    (*args_info).blocklist_file_orig = 0 as *mut libc::c_char;
    (*args_info).allowlist_file_arg = 0 as *mut libc::c_char;
    (*args_info).allowlist_file_orig = 0 as *mut libc::c_char;
    (*args_info).list_of_ips_file_arg = 0 as *mut libc::c_char;
    (*args_info).list_of_ips_file_orig = 0 as *mut libc::c_char;
    (*args_info).rate_orig = 0 as *mut libc::c_char;
    (*args_info).bandwidth_arg = 0 as *mut libc::c_char;
    (*args_info).bandwidth_orig = 0 as *mut libc::c_char;
    (*args_info).batch_orig = 0 as *mut libc::c_char;
    (*args_info).max_targets_arg = 0 as *mut libc::c_char;
    (*args_info).max_targets_orig = 0 as *mut libc::c_char;
    (*args_info).max_runtime_orig = 0 as *mut libc::c_char;
    (*args_info).max_results_orig = 0 as *mut libc::c_char;
    (*args_info).probes_arg = 1 as libc::c_int;
    (*args_info).probes_orig = 0 as *mut libc::c_char;
    (*args_info).cooldown_time_arg = 8 as libc::c_int;
    (*args_info).cooldown_time_orig = 0 as *mut libc::c_char;
    (*args_info).seed_orig = 0 as *mut libc::c_char;
    (*args_info).retries_arg = 10 as libc::c_int;
    (*args_info).retries_orig = 0 as *mut libc::c_char;
    (*args_info).shards_arg = 1 as libc::c_int;
    (*args_info).shards_orig = 0 as *mut libc::c_char;
    (*args_info).shard_arg = 0 as libc::c_int;
    (*args_info).shard_orig = 0 as *mut libc::c_char;
    (*args_info).source_port_arg = 0 as *mut libc::c_char;
    (*args_info).source_port_orig = 0 as *mut libc::c_char;
    (*args_info).source_ip_arg = 0 as *mut libc::c_char;
    (*args_info).source_ip_orig = 0 as *mut libc::c_char;
    (*args_info).gateway_mac_arg = 0 as *mut libc::c_char;
    (*args_info).gateway_mac_orig = 0 as *mut libc::c_char;
    (*args_info).source_mac_arg = 0 as *mut libc::c_char;
    (*args_info).source_mac_orig = 0 as *mut libc::c_char;
    (*args_info).interface_arg = 0 as *mut libc::c_char;
    (*args_info).interface_orig = 0 as *mut libc::c_char;
    (*args_info)
        .probe_module_arg = gengetopt_strdup(
        b"tcp_synscan\0" as *const u8 as *const libc::c_char,
    );
    (*args_info).probe_module_orig = 0 as *mut libc::c_char;
    (*args_info).probe_args_arg = 0 as *mut libc::c_char;
    (*args_info).probe_args_orig = 0 as *mut libc::c_char;
    (*args_info).probe_ttl_arg = 255 as libc::c_int;
    (*args_info).probe_ttl_orig = 0 as *mut libc::c_char;
    (*args_info).output_fields_arg = 0 as *mut libc::c_char;
    (*args_info).output_fields_orig = 0 as *mut libc::c_char;
    (*args_info).output_module_arg = 0 as *mut libc::c_char;
    (*args_info).output_module_orig = 0 as *mut libc::c_char;
    (*args_info).output_args_arg = 0 as *mut libc::c_char;
    (*args_info).output_args_orig = 0 as *mut libc::c_char;
    (*args_info).output_filter_arg = 0 as *mut libc::c_char;
    (*args_info).output_filter_orig = 0 as *mut libc::c_char;
    (*args_info).verbosity_arg = 3 as libc::c_int;
    (*args_info).verbosity_orig = 0 as *mut libc::c_char;
    (*args_info).log_file_arg = 0 as *mut libc::c_char;
    (*args_info).log_file_orig = 0 as *mut libc::c_char;
    (*args_info).log_directory_arg = 0 as *mut libc::c_char;
    (*args_info).log_directory_orig = 0 as *mut libc::c_char;
    (*args_info).metadata_file_arg = 0 as *mut libc::c_char;
    (*args_info).metadata_file_orig = 0 as *mut libc::c_char;
    (*args_info).status_updates_file_arg = 0 as *mut libc::c_char;
    (*args_info).status_updates_file_orig = 0 as *mut libc::c_char;
    (*args_info).notes_arg = 0 as *mut libc::c_char;
    (*args_info).notes_orig = 0 as *mut libc::c_char;
    (*args_info).user_metadata_arg = 0 as *mut libc::c_char;
    (*args_info).user_metadata_orig = 0 as *mut libc::c_char;
    (*args_info)
        .config_arg = gengetopt_strdup(
        b"/etc/zmap/zmap.conf\0" as *const u8 as *const libc::c_char,
    );
    (*args_info).config_orig = 0 as *mut libc::c_char;
    (*args_info).max_sendto_failures_arg = -(1 as libc::c_int);
    (*args_info).max_sendto_failures_orig = 0 as *mut libc::c_char;
    (*args_info).min_hitrate_arg = 0.0f64 as libc::c_float;
    (*args_info).min_hitrate_orig = 0 as *mut libc::c_char;
    (*args_info).sender_threads_arg = 1 as libc::c_int;
    (*args_info).sender_threads_orig = 0 as *mut libc::c_char;
    (*args_info).cores_arg = 0 as *mut libc::c_char;
    (*args_info).cores_orig = 0 as *mut libc::c_char;
}
unsafe extern "C" fn init_args_info(mut args_info: *mut gengetopt_args_info) {
    (*args_info).target_port_help = gengetopt_args_info_help[1 as libc::c_int as usize];
    (*args_info).output_file_help = gengetopt_args_info_help[2 as libc::c_int as usize];
    (*args_info)
        .blocklist_file_help = gengetopt_args_info_help[3 as libc::c_int as usize];
    (*args_info)
        .allowlist_file_help = gengetopt_args_info_help[4 as libc::c_int as usize];
    (*args_info)
        .list_of_ips_file_help = gengetopt_args_info_help[5 as libc::c_int as usize];
    (*args_info).rate_help = gengetopt_args_info_help[7 as libc::c_int as usize];
    (*args_info).bandwidth_help = gengetopt_args_info_help[8 as libc::c_int as usize];
    (*args_info).batch_help = gengetopt_args_info_help[9 as libc::c_int as usize];
    (*args_info).max_targets_help = gengetopt_args_info_help[10 as libc::c_int as usize];
    (*args_info).max_runtime_help = gengetopt_args_info_help[11 as libc::c_int as usize];
    (*args_info).max_results_help = gengetopt_args_info_help[12 as libc::c_int as usize];
    (*args_info).probes_help = gengetopt_args_info_help[13 as libc::c_int as usize];
    (*args_info)
        .cooldown_time_help = gengetopt_args_info_help[14 as libc::c_int as usize];
    (*args_info).seed_help = gengetopt_args_info_help[15 as libc::c_int as usize];
    (*args_info).retries_help = gengetopt_args_info_help[16 as libc::c_int as usize];
    (*args_info).dryrun_help = gengetopt_args_info_help[17 as libc::c_int as usize];
    (*args_info).shards_help = gengetopt_args_info_help[19 as libc::c_int as usize];
    (*args_info).shard_help = gengetopt_args_info_help[20 as libc::c_int as usize];
    (*args_info).source_port_help = gengetopt_args_info_help[22 as libc::c_int as usize];
    (*args_info).source_ip_help = gengetopt_args_info_help[23 as libc::c_int as usize];
    (*args_info).gateway_mac_help = gengetopt_args_info_help[24 as libc::c_int as usize];
    (*args_info).source_mac_help = gengetopt_args_info_help[25 as libc::c_int as usize];
    (*args_info).interface_help = gengetopt_args_info_help[26 as libc::c_int as usize];
    (*args_info).iplayer_help = gengetopt_args_info_help[27 as libc::c_int as usize];
    (*args_info)
        .probe_module_help = gengetopt_args_info_help[29 as libc::c_int as usize];
    (*args_info).probe_args_help = gengetopt_args_info_help[30 as libc::c_int as usize];
    (*args_info).probe_ttl_help = gengetopt_args_info_help[31 as libc::c_int as usize];
    (*args_info)
        .list_probe_modules_help = gengetopt_args_info_help[32 as libc::c_int as usize];
    (*args_info)
        .output_fields_help = gengetopt_args_info_help[34 as libc::c_int as usize];
    (*args_info)
        .output_module_help = gengetopt_args_info_help[35 as libc::c_int as usize];
    (*args_info).output_args_help = gengetopt_args_info_help[36 as libc::c_int as usize];
    (*args_info)
        .output_filter_help = gengetopt_args_info_help[37 as libc::c_int as usize];
    (*args_info)
        .list_output_modules_help = gengetopt_args_info_help[38 as libc::c_int as usize];
    (*args_info)
        .list_output_fields_help = gengetopt_args_info_help[39 as libc::c_int as usize];
    (*args_info)
        .no_header_row_help = gengetopt_args_info_help[40 as libc::c_int as usize];
    (*args_info).verbosity_help = gengetopt_args_info_help[42 as libc::c_int as usize];
    (*args_info).log_file_help = gengetopt_args_info_help[43 as libc::c_int as usize];
    (*args_info)
        .log_directory_help = gengetopt_args_info_help[44 as libc::c_int as usize];
    (*args_info)
        .metadata_file_help = gengetopt_args_info_help[45 as libc::c_int as usize];
    (*args_info)
        .status_updates_file_help = gengetopt_args_info_help[46 as libc::c_int as usize];
    (*args_info).quiet_help = gengetopt_args_info_help[47 as libc::c_int as usize];
    (*args_info)
        .disable_syslog_help = gengetopt_args_info_help[48 as libc::c_int as usize];
    (*args_info).notes_help = gengetopt_args_info_help[49 as libc::c_int as usize];
    (*args_info)
        .user_metadata_help = gengetopt_args_info_help[50 as libc::c_int as usize];
    (*args_info).config_help = gengetopt_args_info_help[52 as libc::c_int as usize];
    (*args_info)
        .max_sendto_failures_help = gengetopt_args_info_help[53 as libc::c_int as usize];
    (*args_info).min_hitrate_help = gengetopt_args_info_help[54 as libc::c_int as usize];
    (*args_info)
        .sender_threads_help = gengetopt_args_info_help[55 as libc::c_int as usize];
    (*args_info).cores_help = gengetopt_args_info_help[56 as libc::c_int as usize];
    (*args_info)
        .ignore_blocklist_errors_help = gengetopt_args_info_help[57 as libc::c_int
        as usize];
    (*args_info).help_help = gengetopt_args_info_help[58 as libc::c_int as usize];
    (*args_info).version_help = gengetopt_args_info_help[59 as libc::c_int as usize];
}
pub unsafe extern "C" fn cmdline_parser_init(mut args_info: *mut gengetopt_args_info) {
    clear_given(args_info);
    clear_args(args_info);
    init_args_info(args_info);
    (*args_info).inputs = 0 as *mut *mut libc::c_char;
    (*args_info).inputs_num = 0 as libc::c_int as libc::c_uint;
}
static mut package_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
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
        2 => {
            if !val.is_null() {
                *(field
                    as *mut libc::c_int) = strtol(val, &mut stop_char, 0 as libc::c_int)
                    as libc::c_int;
            }
        }
        3 => {
            if !val.is_null() {
                *(field
                    as *mut libc::c_float) = strtod(val, &mut stop_char)
                    as libc::c_float;
            }
        }
        4 => {
            if !val.is_null() {
                *(field
                    as *mut libc::c_long) = strtol(
                    val,
                    &mut stop_char,
                    0 as libc::c_int,
                );
            }
        }
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
        2 | 3 | 4 => {
            if !val.is_null()
                && !(!stop_char.is_null() && *stop_char as libc::c_int == '\0' as i32)
            {
                fprintf(
                    stderr,
                    b"%s: invalid numeric value: %s\n\0" as *const u8
                        as *const libc::c_char,
                    package_name,
                    val,
                );
                return 1 as libc::c_int;
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
        target_port_arg: 0,
        target_port_orig: 0 as *mut libc::c_char,
        target_port_help: 0 as *const libc::c_char,
        output_file_arg: 0 as *mut libc::c_char,
        output_file_orig: 0 as *mut libc::c_char,
        output_file_help: 0 as *const libc::c_char,
        blocklist_file_arg: 0 as *mut libc::c_char,
        blocklist_file_orig: 0 as *mut libc::c_char,
        blocklist_file_help: 0 as *const libc::c_char,
        allowlist_file_arg: 0 as *mut libc::c_char,
        allowlist_file_orig: 0 as *mut libc::c_char,
        allowlist_file_help: 0 as *const libc::c_char,
        list_of_ips_file_arg: 0 as *mut libc::c_char,
        list_of_ips_file_orig: 0 as *mut libc::c_char,
        list_of_ips_file_help: 0 as *const libc::c_char,
        rate_arg: 0,
        rate_orig: 0 as *mut libc::c_char,
        rate_help: 0 as *const libc::c_char,
        bandwidth_arg: 0 as *mut libc::c_char,
        bandwidth_orig: 0 as *mut libc::c_char,
        bandwidth_help: 0 as *const libc::c_char,
        batch_arg: 0,
        batch_orig: 0 as *mut libc::c_char,
        batch_help: 0 as *const libc::c_char,
        max_targets_arg: 0 as *mut libc::c_char,
        max_targets_orig: 0 as *mut libc::c_char,
        max_targets_help: 0 as *const libc::c_char,
        max_runtime_arg: 0,
        max_runtime_orig: 0 as *mut libc::c_char,
        max_runtime_help: 0 as *const libc::c_char,
        max_results_arg: 0,
        max_results_orig: 0 as *mut libc::c_char,
        max_results_help: 0 as *const libc::c_char,
        probes_arg: 0,
        probes_orig: 0 as *mut libc::c_char,
        probes_help: 0 as *const libc::c_char,
        cooldown_time_arg: 0,
        cooldown_time_orig: 0 as *mut libc::c_char,
        cooldown_time_help: 0 as *const libc::c_char,
        seed_arg: 0,
        seed_orig: 0 as *mut libc::c_char,
        seed_help: 0 as *const libc::c_char,
        retries_arg: 0,
        retries_orig: 0 as *mut libc::c_char,
        retries_help: 0 as *const libc::c_char,
        dryrun_help: 0 as *const libc::c_char,
        shards_arg: 0,
        shards_orig: 0 as *mut libc::c_char,
        shards_help: 0 as *const libc::c_char,
        shard_arg: 0,
        shard_orig: 0 as *mut libc::c_char,
        shard_help: 0 as *const libc::c_char,
        source_port_arg: 0 as *mut libc::c_char,
        source_port_orig: 0 as *mut libc::c_char,
        source_port_help: 0 as *const libc::c_char,
        source_ip_arg: 0 as *mut libc::c_char,
        source_ip_orig: 0 as *mut libc::c_char,
        source_ip_help: 0 as *const libc::c_char,
        gateway_mac_arg: 0 as *mut libc::c_char,
        gateway_mac_orig: 0 as *mut libc::c_char,
        gateway_mac_help: 0 as *const libc::c_char,
        source_mac_arg: 0 as *mut libc::c_char,
        source_mac_orig: 0 as *mut libc::c_char,
        source_mac_help: 0 as *const libc::c_char,
        interface_arg: 0 as *mut libc::c_char,
        interface_orig: 0 as *mut libc::c_char,
        interface_help: 0 as *const libc::c_char,
        iplayer_help: 0 as *const libc::c_char,
        probe_module_arg: 0 as *mut libc::c_char,
        probe_module_orig: 0 as *mut libc::c_char,
        probe_module_help: 0 as *const libc::c_char,
        probe_args_arg: 0 as *mut libc::c_char,
        probe_args_orig: 0 as *mut libc::c_char,
        probe_args_help: 0 as *const libc::c_char,
        probe_ttl_arg: 0,
        probe_ttl_orig: 0 as *mut libc::c_char,
        probe_ttl_help: 0 as *const libc::c_char,
        list_probe_modules_help: 0 as *const libc::c_char,
        output_fields_arg: 0 as *mut libc::c_char,
        output_fields_orig: 0 as *mut libc::c_char,
        output_fields_help: 0 as *const libc::c_char,
        output_module_arg: 0 as *mut libc::c_char,
        output_module_orig: 0 as *mut libc::c_char,
        output_module_help: 0 as *const libc::c_char,
        output_args_arg: 0 as *mut libc::c_char,
        output_args_orig: 0 as *mut libc::c_char,
        output_args_help: 0 as *const libc::c_char,
        output_filter_arg: 0 as *mut libc::c_char,
        output_filter_orig: 0 as *mut libc::c_char,
        output_filter_help: 0 as *const libc::c_char,
        list_output_modules_help: 0 as *const libc::c_char,
        list_output_fields_help: 0 as *const libc::c_char,
        no_header_row_help: 0 as *const libc::c_char,
        verbosity_arg: 0,
        verbosity_orig: 0 as *mut libc::c_char,
        verbosity_help: 0 as *const libc::c_char,
        log_file_arg: 0 as *mut libc::c_char,
        log_file_orig: 0 as *mut libc::c_char,
        log_file_help: 0 as *const libc::c_char,
        log_directory_arg: 0 as *mut libc::c_char,
        log_directory_orig: 0 as *mut libc::c_char,
        log_directory_help: 0 as *const libc::c_char,
        metadata_file_arg: 0 as *mut libc::c_char,
        metadata_file_orig: 0 as *mut libc::c_char,
        metadata_file_help: 0 as *const libc::c_char,
        status_updates_file_arg: 0 as *mut libc::c_char,
        status_updates_file_orig: 0 as *mut libc::c_char,
        status_updates_file_help: 0 as *const libc::c_char,
        quiet_help: 0 as *const libc::c_char,
        disable_syslog_help: 0 as *const libc::c_char,
        notes_arg: 0 as *mut libc::c_char,
        notes_orig: 0 as *mut libc::c_char,
        notes_help: 0 as *const libc::c_char,
        user_metadata_arg: 0 as *mut libc::c_char,
        user_metadata_orig: 0 as *mut libc::c_char,
        user_metadata_help: 0 as *const libc::c_char,
        config_arg: 0 as *mut libc::c_char,
        config_orig: 0 as *mut libc::c_char,
        config_help: 0 as *const libc::c_char,
        max_sendto_failures_arg: 0,
        max_sendto_failures_orig: 0 as *mut libc::c_char,
        max_sendto_failures_help: 0 as *const libc::c_char,
        min_hitrate_arg: 0.,
        min_hitrate_orig: 0 as *mut libc::c_char,
        min_hitrate_help: 0 as *const libc::c_char,
        sender_threads_arg: 0,
        sender_threads_orig: 0 as *mut libc::c_char,
        sender_threads_help: 0 as *const libc::c_char,
        cores_arg: 0 as *mut libc::c_char,
        cores_orig: 0 as *mut libc::c_char,
        cores_help: 0 as *const libc::c_char,
        ignore_blocklist_errors_help: 0 as *const libc::c_char,
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        target_port_given: 0,
        output_file_given: 0,
        blocklist_file_given: 0,
        allowlist_file_given: 0,
        list_of_ips_file_given: 0,
        rate_given: 0,
        bandwidth_given: 0,
        batch_given: 0,
        max_targets_given: 0,
        max_runtime_given: 0,
        max_results_given: 0,
        probes_given: 0,
        cooldown_time_given: 0,
        seed_given: 0,
        retries_given: 0,
        dryrun_given: 0,
        shards_given: 0,
        shard_given: 0,
        source_port_given: 0,
        source_ip_given: 0,
        gateway_mac_given: 0,
        source_mac_given: 0,
        interface_given: 0,
        iplayer_given: 0,
        probe_module_given: 0,
        probe_args_given: 0,
        probe_ttl_given: 0,
        list_probe_modules_given: 0,
        output_fields_given: 0,
        output_module_given: 0,
        output_args_given: 0,
        output_filter_given: 0,
        list_output_modules_given: 0,
        list_output_fields_given: 0,
        no_header_row_given: 0,
        verbosity_given: 0,
        log_file_given: 0,
        log_directory_given: 0,
        metadata_file_given: 0,
        status_updates_file_given: 0,
        quiet_given: 0,
        disable_syslog_given: 0,
        notes_given: 0,
        user_metadata_given: 0,
        config_given: 0,
        max_sendto_failures_given: 0,
        min_hitrate_given: 0,
        sender_threads_given: 0,
        cores_given: 0,
        ignore_blocklist_errors_given: 0,
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
        static mut long_options: [option; 53] = [
            {
                let mut init = option {
                    name: b"target-port\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'p' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"output-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'o' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"blocklist-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'b' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"allowlist-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'w' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"list-of-ips-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'I' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"rate\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'r' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"bandwidth\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'B' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"batch\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"max-targets\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'n' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"max-runtime\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 't' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"max-results\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'N' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"probes\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'P' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"cooldown-time\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'c' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"seed\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'e' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"retries\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"dryrun\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'd' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"shards\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"shard\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"source-port\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 's' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"source-ip\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'S' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"gateway-mac\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'G' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"source-mac\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"interface\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'i' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"iplayer\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'X' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"probe-module\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'M' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"probe-args\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"probe-ttl\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"list-probe-modules\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"output-fields\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'f' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"output-module\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'O' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"output-args\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"output-filter\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"list-output-modules\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"list-output-fields\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"no-header-row\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"verbosity\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'v' as i32,
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
                    name: b"log-directory\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'L' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"metadata-file\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
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
                    name: b"quiet\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'q' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"disable-syslog\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"notes\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"user-metadata\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"config\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'C' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"min-hitrate\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"sender-threads\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'T' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"cores\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"ignore-blocklist-errors\0" as *const u8
                        as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
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
            b"p:o:b:w:I:r:B:n:t:N:P:c:e:ds:S:G:i:XM:f:O:v:l:L:m:u:qC:T:hV\0" as *const u8
                as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            current_block = 10468276026569382870;
            break;
        }
        match c {
            112 => {
                if update_arg(
                    &mut (*args_info).target_port_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).target_port_orig,
                    &mut (*args_info).target_port_given,
                    &mut local_args_info.target_port_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"target-port\0" as *const u8 as *const libc::c_char,
                    'p' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            111 => {
                if update_arg(
                    &mut (*args_info).output_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).output_file_orig,
                    &mut (*args_info).output_file_given,
                    &mut local_args_info.output_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"output-file\0" as *const u8 as *const libc::c_char,
                    'o' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            98 => {
                if update_arg(
                    &mut (*args_info).blocklist_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).blocklist_file_orig,
                    &mut (*args_info).blocklist_file_given,
                    &mut local_args_info.blocklist_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"blocklist-file\0" as *const u8 as *const libc::c_char,
                    'b' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            119 => {
                if update_arg(
                    &mut (*args_info).allowlist_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).allowlist_file_orig,
                    &mut (*args_info).allowlist_file_given,
                    &mut local_args_info.allowlist_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"allowlist-file\0" as *const u8 as *const libc::c_char,
                    'w' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            73 => {
                if update_arg(
                    &mut (*args_info).list_of_ips_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).list_of_ips_file_orig,
                    &mut (*args_info).list_of_ips_file_given,
                    &mut local_args_info.list_of_ips_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"list-of-ips-file\0" as *const u8 as *const libc::c_char,
                    'I' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            114 => {
                if update_arg(
                    &mut (*args_info).rate_arg as *mut libc::c_int as *mut libc::c_void,
                    &mut (*args_info).rate_orig,
                    &mut (*args_info).rate_given,
                    &mut local_args_info.rate_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"rate\0" as *const u8 as *const libc::c_char,
                    'r' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            66 => {
                if update_arg(
                    &mut (*args_info).bandwidth_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).bandwidth_orig,
                    &mut (*args_info).bandwidth_given,
                    &mut local_args_info.bandwidth_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"bandwidth\0" as *const u8 as *const libc::c_char,
                    'B' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            110 => {
                if update_arg(
                    &mut (*args_info).max_targets_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).max_targets_orig,
                    &mut (*args_info).max_targets_given,
                    &mut local_args_info.max_targets_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"max-targets\0" as *const u8 as *const libc::c_char,
                    'n' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            116 => {
                if update_arg(
                    &mut (*args_info).max_runtime_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).max_runtime_orig,
                    &mut (*args_info).max_runtime_given,
                    &mut local_args_info.max_runtime_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"max-runtime\0" as *const u8 as *const libc::c_char,
                    't' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            78 => {
                if update_arg(
                    &mut (*args_info).max_results_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).max_results_orig,
                    &mut (*args_info).max_results_given,
                    &mut local_args_info.max_results_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"max-results\0" as *const u8 as *const libc::c_char,
                    'N' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            80 => {
                if update_arg(
                    &mut (*args_info).probes_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).probes_orig,
                    &mut (*args_info).probes_given,
                    &mut local_args_info.probes_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"probes\0" as *const u8 as *const libc::c_char,
                    'P' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            99 => {
                if update_arg(
                    &mut (*args_info).cooldown_time_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).cooldown_time_orig,
                    &mut (*args_info).cooldown_time_given,
                    &mut local_args_info.cooldown_time_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"8\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"cooldown-time\0" as *const u8 as *const libc::c_char,
                    'c' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            101 => {
                if update_arg(
                    &mut (*args_info).seed_arg as *mut libc::c_long as *mut libc::c_void,
                    &mut (*args_info).seed_orig,
                    &mut (*args_info).seed_given,
                    &mut local_args_info.seed_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_LONGLONG,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"seed\0" as *const u8 as *const libc::c_char,
                    'e' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            100 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).dryrun_given,
                    &mut local_args_info.dryrun_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"dryrun\0" as *const u8 as *const libc::c_char,
                    'd' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            115 => {
                if update_arg(
                    &mut (*args_info).source_port_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).source_port_orig,
                    &mut (*args_info).source_port_given,
                    &mut local_args_info.source_port_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"source-port\0" as *const u8 as *const libc::c_char,
                    's' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            83 => {
                if update_arg(
                    &mut (*args_info).source_ip_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).source_ip_orig,
                    &mut (*args_info).source_ip_given,
                    &mut local_args_info.source_ip_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"source-ip\0" as *const u8 as *const libc::c_char,
                    'S' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            71 => {
                if update_arg(
                    &mut (*args_info).gateway_mac_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).gateway_mac_orig,
                    &mut (*args_info).gateway_mac_given,
                    &mut local_args_info.gateway_mac_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"gateway-mac\0" as *const u8 as *const libc::c_char,
                    'G' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            105 => {
                if update_arg(
                    &mut (*args_info).interface_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).interface_orig,
                    &mut (*args_info).interface_given,
                    &mut local_args_info.interface_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"interface\0" as *const u8 as *const libc::c_char,
                    'i' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            88 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).iplayer_given,
                    &mut local_args_info.iplayer_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"iplayer\0" as *const u8 as *const libc::c_char,
                    'X' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            77 => {
                if update_arg(
                    &mut (*args_info).probe_module_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).probe_module_orig,
                    &mut (*args_info).probe_module_given,
                    &mut local_args_info.probe_module_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"tcp_synscan\0" as *const u8 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"probe-module\0" as *const u8 as *const libc::c_char,
                    'M' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            102 => {
                if update_arg(
                    &mut (*args_info).output_fields_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).output_fields_orig,
                    &mut (*args_info).output_fields_given,
                    &mut local_args_info.output_fields_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"output-fields\0" as *const u8 as *const libc::c_char,
                    'f' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            79 => {
                if update_arg(
                    &mut (*args_info).output_module_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).output_module_orig,
                    &mut (*args_info).output_module_given,
                    &mut local_args_info.output_module_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"output-module\0" as *const u8 as *const libc::c_char,
                    'O' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            118 => {
                if update_arg(
                    &mut (*args_info).verbosity_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).verbosity_orig,
                    &mut (*args_info).verbosity_given,
                    &mut local_args_info.verbosity_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"3\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"verbosity\0" as *const u8 as *const libc::c_char,
                    'v' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
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
                    current_block = 11891211785256829551;
                    break;
                }
            }
            76 => {
                if update_arg(
                    &mut (*args_info).log_directory_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).log_directory_orig,
                    &mut (*args_info).log_directory_given,
                    &mut local_args_info.log_directory_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"log-directory\0" as *const u8 as *const libc::c_char,
                    'L' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            109 => {
                if update_arg(
                    &mut (*args_info).metadata_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).metadata_file_orig,
                    &mut (*args_info).metadata_file_given,
                    &mut local_args_info.metadata_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"metadata-file\0" as *const u8 as *const libc::c_char,
                    'm' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
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
                    current_block = 11891211785256829551;
                    break;
                }
            }
            113 => {
                if update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).quiet_given,
                    &mut local_args_info.quiet_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"quiet\0" as *const u8 as *const libc::c_char,
                    'q' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            67 => {
                if update_arg(
                    &mut (*args_info).config_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).config_orig,
                    &mut (*args_info).config_given,
                    &mut local_args_info.config_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"/etc/zmap/zmap.conf\0" as *const u8 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"config\0" as *const u8 as *const libc::c_char,
                    'C' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
                    break;
                }
            }
            84 => {
                if update_arg(
                    &mut (*args_info).sender_threads_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).sender_threads_orig,
                    &mut (*args_info).sender_threads_given,
                    &mut local_args_info.sender_threads_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"sender-threads\0" as *const u8 as *const libc::c_char,
                    'T' as i32 as libc::c_char,
                    additional_error,
                ) != 0
                {
                    current_block = 11891211785256829551;
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
                    current_block = 11891211785256829551;
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
                    current_block = 11891211785256829551;
                    break;
                }
            }
            0 => {
                if strcmp(
                    long_options[option_index as usize].name,
                    b"batch\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).batch_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).batch_orig,
                        &mut (*args_info).batch_given,
                        &mut local_args_info.batch_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"batch\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"retries\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).retries_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).retries_orig,
                        &mut (*args_info).retries_given,
                        &mut local_args_info.retries_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        b"10\0" as *const u8 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"retries\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"shards\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).shards_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).shards_orig,
                        &mut (*args_info).shards_given,
                        &mut local_args_info.shards_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        b"1\0" as *const u8 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"shards\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"shard\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).shard_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).shard_orig,
                        &mut (*args_info).shard_given,
                        &mut local_args_info.shard_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        b"0\0" as *const u8 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"shard\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"source-mac\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).source_mac_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).source_mac_orig,
                        &mut (*args_info).source_mac_given,
                        &mut local_args_info.source_mac_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"source-mac\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"probe-args\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).probe_args_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).probe_args_orig,
                        &mut (*args_info).probe_args_given,
                        &mut local_args_info.probe_args_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"probe-args\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"probe-ttl\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).probe_ttl_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).probe_ttl_orig,
                        &mut (*args_info).probe_ttl_given,
                        &mut local_args_info.probe_ttl_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        b"255\0" as *const u8 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"probe-ttl\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"list-probe-modules\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        0 as *mut libc::c_void,
                        0 as *mut *mut libc::c_char,
                        &mut (*args_info).list_probe_modules_given,
                        &mut local_args_info.list_probe_modules_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_NO,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"list-probe-modules\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"output-args\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).output_args_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).output_args_orig,
                        &mut (*args_info).output_args_given,
                        &mut local_args_info.output_args_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"output-args\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"output-filter\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).output_filter_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).output_filter_orig,
                        &mut (*args_info).output_filter_given,
                        &mut local_args_info.output_filter_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"output-filter\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"list-output-modules\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        0 as *mut libc::c_void,
                        0 as *mut *mut libc::c_char,
                        &mut (*args_info).list_output_modules_given,
                        &mut local_args_info.list_output_modules_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_NO,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"list-output-modules\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"list-output-fields\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        0 as *mut libc::c_void,
                        0 as *mut *mut libc::c_char,
                        &mut (*args_info).list_output_fields_given,
                        &mut local_args_info.list_output_fields_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_NO,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"list-output-fields\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"no-header-row\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        0 as *mut libc::c_void,
                        0 as *mut *mut libc::c_char,
                        &mut (*args_info).no_header_row_given,
                        &mut local_args_info.no_header_row_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_NO,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"no-header-row\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"disable-syslog\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        0 as *mut libc::c_void,
                        0 as *mut *mut libc::c_char,
                        &mut (*args_info).disable_syslog_given,
                        &mut local_args_info.disable_syslog_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_NO,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"disable-syslog\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"notes\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).notes_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).notes_orig,
                        &mut (*args_info).notes_given,
                        &mut local_args_info.notes_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"notes\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"user-metadata\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).user_metadata_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).user_metadata_orig,
                        &mut (*args_info).user_metadata_given,
                        &mut local_args_info.user_metadata_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"user-metadata\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).max_sendto_failures_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).max_sendto_failures_orig,
                        &mut (*args_info).max_sendto_failures_given,
                        &mut local_args_info.max_sendto_failures_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        b"-1\0" as *const u8 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"min-hitrate\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).min_hitrate_arg as *mut libc::c_float
                            as *mut libc::c_void,
                        &mut (*args_info).min_hitrate_orig,
                        &mut (*args_info).min_hitrate_given,
                        &mut local_args_info.min_hitrate_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        b"0.0\0" as *const u8 as *const libc::c_char,
                        ARG_FLOAT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"min-hitrate\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"cores\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if update_arg(
                        &mut (*args_info).cores_arg as *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*args_info).cores_orig,
                        &mut (*args_info).cores_given,
                        &mut local_args_info.cores_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_STRING,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"cores\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                } else {
                    if !(strcmp(
                        long_options[option_index as usize].name,
                        b"ignore-blocklist-errors\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int)
                    {
                        continue;
                    }
                    if update_arg(
                        0 as *mut libc::c_void,
                        0 as *mut *mut libc::c_char,
                        &mut (*args_info).ignore_blocklist_errors_given,
                        &mut local_args_info.ignore_blocklist_errors_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_NO,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"ignore-blocklist-errors\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    ) != 0
                    {
                        current_block = 11891211785256829551;
                        break;
                    }
                }
            }
            63 => {
                current_block = 11891211785256829551;
                break;
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s: option unknown: %c%s\n\0" as *const u8 as *const libc::c_char,
                    b"zmap\0" as *const u8 as *const libc::c_char,
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
        11891211785256829551 => {
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
                    let fresh5 = i;
                    i = i + 1;
                    if !(*argv.offset(fresh5 as isize)
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
                    let fresh6 = optind;
                    optind = optind + 1;
                    if *argv.offset(fresh6 as isize)
                        != *argv.offset(0 as libc::c_int as isize)
                    {
                        let fresh7 = i;
                        i = i + 1;
                        let ref mut fresh8 = *((*args_info).inputs)
                            .offset(fresh7 as isize);
                        *fresh8 = gengetopt_strdup(
                            *argv.offset((optind - 1 as libc::c_int) as isize),
                        );
                    }
                }
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn free_string_field(mut s: *mut *mut libc::c_char) {
    if !(*s).is_null() {
        free(*s as *mut libc::c_void);
        *s = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn clear_given(mut args_info: *mut gengetopt_args_info) {
    (*args_info).target_port_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).output_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).blocklist_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).allowlist_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).list_of_ips_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).rate_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).bandwidth_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).batch_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).max_targets_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).max_runtime_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).max_results_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).probes_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).cooldown_time_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).seed_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).retries_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).dryrun_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).shards_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).shard_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).source_port_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).source_ip_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).gateway_mac_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).source_mac_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).interface_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).iplayer_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).probe_module_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).probe_args_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).probe_ttl_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).list_probe_modules_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).output_fields_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).output_module_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).output_args_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).output_filter_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).list_output_modules_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).list_output_fields_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).no_header_row_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).verbosity_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).log_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).log_directory_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).metadata_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).status_updates_file_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).quiet_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).disable_syslog_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).notes_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).user_metadata_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).config_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).max_sendto_failures_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).min_hitrate_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).sender_threads_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).cores_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).ignore_blocklist_errors_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).help_given = 0 as libc::c_int as libc::c_uint;
    (*args_info).version_given = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn cmdline_parser_release(mut args_info: *mut gengetopt_args_info) {
    let mut i: libc::c_uint = 0;
    free_string_field(&mut (*args_info).target_port_orig);
    free_string_field(&mut (*args_info).output_file_arg);
    free_string_field(&mut (*args_info).output_file_orig);
    free_string_field(&mut (*args_info).blocklist_file_arg);
    free_string_field(&mut (*args_info).blocklist_file_orig);
    free_string_field(&mut (*args_info).allowlist_file_arg);
    free_string_field(&mut (*args_info).allowlist_file_orig);
    free_string_field(&mut (*args_info).list_of_ips_file_arg);
    free_string_field(&mut (*args_info).list_of_ips_file_orig);
    free_string_field(&mut (*args_info).rate_orig);
    free_string_field(&mut (*args_info).bandwidth_arg);
    free_string_field(&mut (*args_info).bandwidth_orig);
    free_string_field(&mut (*args_info).batch_orig);
    free_string_field(&mut (*args_info).max_targets_arg);
    free_string_field(&mut (*args_info).max_targets_orig);
    free_string_field(&mut (*args_info).max_runtime_orig);
    free_string_field(&mut (*args_info).max_results_orig);
    free_string_field(&mut (*args_info).probes_orig);
    free_string_field(&mut (*args_info).cooldown_time_orig);
    free_string_field(&mut (*args_info).seed_orig);
    free_string_field(&mut (*args_info).retries_orig);
    free_string_field(&mut (*args_info).shards_orig);
    free_string_field(&mut (*args_info).shard_orig);
    free_string_field(&mut (*args_info).source_port_arg);
    free_string_field(&mut (*args_info).source_port_orig);
    free_string_field(&mut (*args_info).source_ip_arg);
    free_string_field(&mut (*args_info).source_ip_orig);
    free_string_field(&mut (*args_info).gateway_mac_arg);
    free_string_field(&mut (*args_info).gateway_mac_orig);
    free_string_field(&mut (*args_info).source_mac_arg);
    free_string_field(&mut (*args_info).source_mac_orig);
    free_string_field(&mut (*args_info).interface_arg);
    free_string_field(&mut (*args_info).interface_orig);
    free_string_field(&mut (*args_info).probe_module_arg);
    free_string_field(&mut (*args_info).probe_module_orig);
    free_string_field(&mut (*args_info).probe_args_arg);
    free_string_field(&mut (*args_info).probe_args_orig);
    free_string_field(&mut (*args_info).probe_ttl_orig);
    free_string_field(&mut (*args_info).output_fields_arg);
    free_string_field(&mut (*args_info).output_fields_orig);
    free_string_field(&mut (*args_info).output_module_arg);
    free_string_field(&mut (*args_info).output_module_orig);
    free_string_field(&mut (*args_info).output_args_arg);
    free_string_field(&mut (*args_info).output_args_orig);
    free_string_field(&mut (*args_info).output_filter_arg);
    free_string_field(&mut (*args_info).output_filter_orig);
    free_string_field(&mut (*args_info).verbosity_orig);
    free_string_field(&mut (*args_info).log_file_arg);
    free_string_field(&mut (*args_info).log_file_orig);
    free_string_field(&mut (*args_info).log_directory_arg);
    free_string_field(&mut (*args_info).log_directory_orig);
    free_string_field(&mut (*args_info).metadata_file_arg);
    free_string_field(&mut (*args_info).metadata_file_orig);
    free_string_field(&mut (*args_info).status_updates_file_arg);
    free_string_field(&mut (*args_info).status_updates_file_orig);
    free_string_field(&mut (*args_info).notes_arg);
    free_string_field(&mut (*args_info).notes_orig);
    free_string_field(&mut (*args_info).user_metadata_arg);
    free_string_field(&mut (*args_info).user_metadata_orig);
    free_string_field(&mut (*args_info).config_arg);
    free_string_field(&mut (*args_info).config_orig);
    free_string_field(&mut (*args_info).max_sendto_failures_orig);
    free_string_field(&mut (*args_info).min_hitrate_orig);
    free_string_field(&mut (*args_info).sender_threads_orig);
    free_string_field(&mut (*args_info).cores_arg);
    free_string_field(&mut (*args_info).cores_orig);
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
pub unsafe extern "C" fn cmdline_parser_free(mut args_info: *mut gengetopt_args_info) {
    cmdline_parser_release(args_info);
}
pub static mut gengetopt_args_info_purpose: *const libc::c_char = b"A fast Internet-wide scanner.\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_usage: *const libc::c_char = b"Usage: zmap [OPTION]... [SUBNETS]...\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_description: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub static mut gengetopt_args_info_help: [*const libc::c_char; 62] = [
    b"Basic Arguments:\0" as *const u8 as *const libc::c_char,
    b"  -p, --target-port=port        port number to scan (for TCP and UDP scans)\0"
        as *const u8 as *const libc::c_char,
    b"  -o, --output-file=name        Output file\0" as *const u8 as *const libc::c_char,
    b"  -b, --blocklist-file=path     File of subnets to exclude, in CIDR notation,\n                                  e.g. 192.168.0.0/16\0"
        as *const u8 as *const libc::c_char,
    b"  -w, --allowlist-file=path     File of subnets to constrain scan to, in CIDR\n                                  notation, e.g. 192.168.0.0/16\0"
        as *const u8 as *const libc::c_char,
    b"  -I, --list-of-ips-file=path   List of individual addresses to scan in random\n                                  order. Use --white-list file unless >1\n                                  million IPs\0"
        as *const u8 as *const libc::c_char,
    b"\nScan Options:\0" as *const u8 as *const libc::c_char,
    b"  -r, --rate=pps                Set send rate in packets/sec\0" as *const u8
        as *const libc::c_char,
    b"  -B, --bandwidth=bps           Set send rate in bits/second (supports suffixes\n                                  G, M and K)\0"
        as *const u8 as *const libc::c_char,
    b"      --batch=pps               Set the number of packets to send per iteration\0"
        as *const u8 as *const libc::c_char,
    b"  -n, --max-targets=n           Cap number of targets to probe (as a number or\n                                  a percentage of the address space)\0"
        as *const u8 as *const libc::c_char,
    b"  -t, --max-runtime=secs        Cap length of time for sending packets\0"
        as *const u8 as *const libc::c_char,
    b"  -N, --max-results=n           Cap number of results to return\0" as *const u8
        as *const libc::c_char,
    b"  -P, --probes=n                Number of probes to send to each IP\n                                  (default=`1')\0"
        as *const u8 as *const libc::c_char,
    b"  -c, --cooldown-time=secs      How long to continue receiving after sending\n                                  last probe  (default=`8')\0"
        as *const u8 as *const libc::c_char,
    b"  -e, --seed=n                  Seed used to select address permutation\0"
        as *const u8 as *const libc::c_char,
    b"      --retries=n               Max number of times to try to send packet if\n                                  send fails  (default=`10')\0"
        as *const u8 as *const libc::c_char,
    b"  -d, --dryrun                  Don't actually send packets\0" as *const u8
        as *const libc::c_char,
    b"\nScan Sharding:\0" as *const u8 as *const libc::c_char,
    b"      --shards=N                Set the total number of shards  (default=`1')\0"
        as *const u8 as *const libc::c_char,
    b"      --shard=n                 Set which shard this scan is (0 indexed)\n                                  (default=`0')\0"
        as *const u8 as *const libc::c_char,
    b"\nNetwork Options:\0" as *const u8 as *const libc::c_char,
    b"  -s, --source-port=port|range  Source port(s) for scan packets\0" as *const u8
        as *const libc::c_char,
    b"  -S, --source-ip=ip|range      Source address(es) for scan packets\0" as *const u8
        as *const libc::c_char,
    b"  -G, --gateway-mac=addr        Specify gateway MAC address\0" as *const u8
        as *const libc::c_char,
    b"      --source-mac=addr         Source MAC address\0" as *const u8
        as *const libc::c_char,
    b"  -i, --interface=name          Specify network interface to use\0" as *const u8
        as *const libc::c_char,
    b"  -X, --iplayer                 Sends IP packets instead of Ethernet (for VPNs)\0"
        as *const u8 as *const libc::c_char,
    b"\nProbe Modules:\0" as *const u8 as *const libc::c_char,
    b"  -M, --probe-module=name       Select probe module  (default=`tcp_synscan')\0"
        as *const u8 as *const libc::c_char,
    b"      --probe-args=args         Arguments to pass to probe module\0" as *const u8
        as *const libc::c_char,
    b"      --probe-ttl=n             Set TTL value for probe IP packets\n                                  (default=`255')\0"
        as *const u8 as *const libc::c_char,
    b"      --list-probe-modules      List available probe modules\0" as *const u8
        as *const libc::c_char,
    b"\nResults Output:\0" as *const u8 as *const libc::c_char,
    b"  -f, --output-fields=fields    Fields that should be output in result set\0"
        as *const u8 as *const libc::c_char,
    b"  -O, --output-module=name      Select output module\0" as *const u8
        as *const libc::c_char,
    b"      --output-args=args        Arguments to pass to output module\0" as *const u8
        as *const libc::c_char,
    b"      --output-filter=filter    Specify a filter over the response fields to\n                                  limit what responses get sent to the output\n                                  module\0"
        as *const u8 as *const libc::c_char,
    b"      --list-output-modules     List available output modules\0" as *const u8
        as *const libc::c_char,
    b"      --list-output-fields      List all fields that can be output by selected\n                                  probe module\0"
        as *const u8 as *const libc::c_char,
    b"      --no-header-row           Precludes outputting any header rows in data\n                                  (e.g., CSV headers)\0"
        as *const u8 as *const libc::c_char,
    b"\nLogging and Metadata:\0" as *const u8 as *const libc::c_char,
    b"  -v, --verbosity=n             Level of log detail (0-5)  (default=`3')\0"
        as *const u8 as *const libc::c_char,
    b"  -l, --log-file=name           Write log entries to file\0" as *const u8
        as *const libc::c_char,
    b"  -L, --log-directory=directory Write log entries to a timestamped file in this\n                                  directory\0"
        as *const u8 as *const libc::c_char,
    b"  -m, --metadata-file=name      Output file for scan metadata (JSON)\0"
        as *const u8 as *const libc::c_char,
    b"  -u, --status-updates-file=name\n                                Write scan progress updates to CSV file\0"
        as *const u8 as *const libc::c_char,
    b"  -q, --quiet                   Do not print status updates\0" as *const u8
        as *const libc::c_char,
    b"      --disable-syslog          Disables logging messages to syslog\0" as *const u8
        as *const libc::c_char,
    b"      --notes=notes             Inject user-specified notes into scan metadata\0"
        as *const u8 as *const libc::c_char,
    b"      --user-metadata=json      Inject user-specified JSON metadata into scan\n                                  metadata\0"
        as *const u8 as *const libc::c_char,
    b"\nAdditional Options:\0" as *const u8 as *const libc::c_char,
    b"  -C, --config=filename         Read a configuration file, which can specify\n                                  any of these options\n                                  (default=`/etc/zmap/zmap.conf')\0"
        as *const u8 as *const libc::c_char,
    b"      --max-sendto-failures=n   Maximum NIC sendto failures before scan is\n                                  aborted  (default=`-1')\0"
        as *const u8 as *const libc::c_char,
    b"      --min-hitrate=n           Minimum hitrate that scan can hit before scan\n                                  is aborted  (default=`0.0')\0"
        as *const u8 as *const libc::c_char,
    b"  -T, --sender-threads=n        Threads used to send packets  (default=`1')\0"
        as *const u8 as *const libc::c_char,
    b"      --cores=STRING            Comma-separated list of cores to pin to\0"
        as *const u8 as *const libc::c_char,
    b"      --ignore-blocklist-errors Ignore invalid entries in allowlist/blocklist\n                                  file.\0"
        as *const u8 as *const libc::c_char,
    b"  -h, --help                    Print help and exit\0" as *const u8
        as *const libc::c_char,
    b"  -V, --version                 Print version and exit\0" as *const u8
        as *const libc::c_char,
    b"\nExamples:\n    zmap -p 80 (scan full IPv4 address space for hosts on TCP/80)\n    zmap -N 5 -B 10M -p 80 (find 5 HTTP servers, scanning at 10 Mb/s)\n    zmap -p 80 10.0.0.0/8 192.168.0.0/16 (scan both subnets on TCP/80)\n    zmap -p 80 1.2.3.4 10.0.0.3 (scan 1.2.3.4, 10.0.0.3 on TCP/80)\0"
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
