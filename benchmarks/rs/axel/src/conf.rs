use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfscanf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn get_if_ip(
        dst: *mut libc::c_char,
        len: size_t,
        iface: *const libc::c_char,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
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
pub type uint16_t = __uint16_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_t {
    pub next: *mut libc::c_void,
    pub text: [libc::c_char; 1024],
}
pub type if_t = message_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_t {
    pub default_filename: [libc::c_char; 1024],
    pub http_proxy: [libc::c_char; 1024],
    pub no_proxy: [libc::c_char; 1024],
    pub num_connections: uint16_t,
    pub strip_cgi_parameters: libc::c_int,
    pub save_state_interval: libc::c_int,
    pub connection_timeout: libc::c_int,
    pub reconnect_delay: libc::c_int,
    pub max_redirect: libc::c_int,
    pub buffer_size: libc::c_int,
    pub max_speed: libc::c_ulonglong,
    pub verbose: libc::c_int,
    pub alternate_output: libc::c_int,
    pub insecure: libc::c_int,
    pub no_clobber: libc::c_int,
    pub percentage: libc::c_int,
    pub interfaces: *mut if_t,
    pub ai_family: sa_family_t,
    pub search_timeout: libc::c_int,
    pub search_threads: libc::c_int,
    pub search_amount: libc::c_int,
    pub search_top: libc::c_int,
    pub io_timeout: libc::c_uint,
    pub add_header_count: libc::c_int,
    pub add_header: [[libc::c_char; 1024]; 10],
}
pub const HDR_USER_AGENT: C2RustUnnamed_0 = 0;
pub const HDR_count_init: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn conf_hdr_make(
    mut dst: *mut libc::c_char,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) {
    snprintf(
        dst,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        k,
        v,
    );
}
unsafe extern "C" fn axel_fscanf(
    mut fp: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut params: ::std::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    params = args.clone();
    ret = vfscanf(fp, format, params.as_va_list());
    ret = !(ret == -(1 as libc::c_int) && ferror(fp) != 0) as libc::c_int;
    if ret == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"I/O error while reading config file: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    }
    return ret;
}
unsafe extern "C" fn parse_protocol(
    mut conf: *mut conf_t,
    mut value: *const libc::c_char,
) -> libc::c_int {
    if strcasecmp(value, b"ipv4\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*conf).ai_family = 2 as libc::c_int as sa_family_t;
    } else if strcasecmp(value, b"ipv6\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*conf).ai_family = 10 as libc::c_int as sa_family_t;
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown protocol %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            value,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conf_loadfile(
    mut conf: *mut conf_t,
    mut file: *const libc::c_char,
) -> libc::c_int {
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut key: [libc::c_char; 1024] = [0; 1024];
    fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 1 as libc::c_int;
    }
    let mut current_block_44: u64;
    while feof(fp) == 0 {
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dst: *mut libc::c_void = 0 as *mut libc::c_void;
        line += 1;
        line;
        *s.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        ret = axel_fscanf(
            fp,
            b"%100[^\n#]s\0" as *const u8 as *const libc::c_char,
            s.as_mut_ptr(),
        );
        if ret == 0 {
            break;
        }
        ret = axel_fscanf(fp, b"%*[^\n]s\0" as *const u8 as *const libc::c_char);
        if ret == 0 {
            break;
        }
        if fgetc(fp) != '\n' as i32 && feof(fp) == 0 {
            fprintf(stderr, b"Expected newline\n\0" as *const u8 as *const libc::c_char);
        } else {
            tmp = strchr(s.as_mut_ptr(), '=' as i32);
            if tmp.is_null() {
                continue;
            }
            sscanf(
                s.as_mut_ptr(),
                b"%[^= \t]s\0" as *const u8 as *const libc::c_char,
                key.as_mut_ptr(),
            );
            loop {
                tmp = tmp.offset(1);
                if !(*(*__ctype_b_loc()).offset(*tmp as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    break;
                }
            }
            value = tmp;
            while *tmp as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*tmp as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                tmp = tmp.offset(1);
                tmp;
            }
            *tmp = '\0' as i32 as libc::c_char;
            if strcmp(
                key.as_mut_ptr(),
                b"default_filename\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                dst = &mut (*conf).default_filename as *mut [libc::c_char; 1024]
                    as *mut libc::c_void;
                current_block_44 = 11307063007268554308;
            } else if strcmp(
                key.as_mut_ptr(),
                b"http_proxy\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                dst = &mut (*conf).http_proxy as *mut [libc::c_char; 1024]
                    as *mut libc::c_void;
                current_block_44 = 11307063007268554308;
            } else if strcmp(
                key.as_mut_ptr(),
                b"no_proxy\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                dst = &mut (*conf).no_proxy as *mut [libc::c_char; 1024]
                    as *mut libc::c_void;
                current_block_44 = 11307063007268554308;
            } else {
                if strcmp(
                    key.as_mut_ptr(),
                    b"strip_cgi_parameters\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).strip_cgi_parameters as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"save_state_interval\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).save_state_interval as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"connection_timeout\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).connection_timeout as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"reconnect_delay\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).reconnect_delay as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"max_redirect\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).max_redirect as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"buffer_size\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).buffer_size as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"max_speed\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).max_speed as *mut libc::c_ulonglong
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"verbose\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).verbose as *mut libc::c_int as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"alternate_output\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).alternate_output as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"percentage\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).percentage as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"insecure\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).insecure as *mut libc::c_int as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"no_clobber\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).no_clobber as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"search_timeout\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).search_timeout as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"search_threads\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).search_threads as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"search_amount\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).search_amount as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"search_top\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    dst = &mut (*conf).search_top as *mut libc::c_int
                        as *mut libc::c_void;
                    current_block_44 = 7420279277351916581;
                } else {
                    if strcmp(
                        key.as_mut_ptr(),
                        b"max_speed\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        dst = &mut (*conf).max_speed as *mut libc::c_ulonglong
                            as *mut libc::c_void;
                        *(dst
                            as *mut libc::c_ulonglong) = strtoull(
                            value,
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        );
                        continue;
                    } else {
                        if strcmp(
                            key.as_mut_ptr(),
                            b"speed_type\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            continue;
                        }
                        if strcmp(
                            key.as_mut_ptr(),
                            b"interfaces\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            if parse_interfaces(conf, value) != 0 {
                                continue;
                            }
                        } else if strcmp(
                            key.as_mut_ptr(),
                            b"use_protocol\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            if parse_protocol(conf, value) != 0 {
                                continue;
                            }
                        } else if strcmp(
                            key.as_mut_ptr(),
                            b"num_connections\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            let mut num: libc::c_int = atoi(value);
                            if num
                                <= 32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int
                            {
                                (*conf).num_connections = num as uint16_t;
                                continue;
                            } else {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Requested too many connections, max is %i\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
                                );
                            }
                        } else if strcmp(
                            key.as_mut_ptr(),
                            b"user_agent\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            conf_hdr_make(
                                ((*conf).add_header[HDR_USER_AGENT as libc::c_int as usize])
                                    .as_mut_ptr(),
                                b"User-Agent\0" as *const u8 as *const libc::c_char,
                                b"Axel/2.17.11 (linux-gnu)\0" as *const u8
                                    as *const libc::c_char,
                            );
                            continue;
                        }
                    }
                    current_block_44 = 6253702158529540806;
                }
                match current_block_44 {
                    6253702158529540806 => {}
                    _ => {
                        *(dst as *mut libc::c_int) = atoi(value);
                        continue;
                    }
                }
            }
            match current_block_44 {
                6253702158529540806 => {}
                _ => {
                    strlcpy(
                        dst as *mut libc::c_char,
                        value,
                        1024 as libc::c_int as size_t,
                    );
                    continue;
                }
            }
        }
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Error in %s line %i.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
        );
        ret = 0 as libc::c_int;
        break;
    }
    fclose(fp);
    return ret;
}
pub unsafe extern "C" fn conf_init(mut conf: *mut conf_t) -> libc::c_int {
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    memset(
        conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conf_t>() as libc::c_ulong,
    );
    strlcpy(
        ((*conf).default_filename).as_mut_ptr(),
        b"default\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    *((*conf).http_proxy).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    *((*conf).no_proxy).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    (*conf).strip_cgi_parameters = 1 as libc::c_int;
    (*conf).save_state_interval = 10 as libc::c_int;
    (*conf).connection_timeout = 45 as libc::c_int;
    (*conf).reconnect_delay = 20 as libc::c_int;
    (*conf).num_connections = 4 as libc::c_int as uint16_t;
    (*conf).max_redirect = 20 as libc::c_int;
    (*conf).io_timeout = 120 as libc::c_int as libc::c_uint;
    (*conf).buffer_size = 5120 as libc::c_int;
    (*conf).max_speed = 0 as libc::c_int as libc::c_ulonglong;
    (*conf).verbose = 1 as libc::c_int;
    (*conf).insecure = 0 as libc::c_int;
    (*conf).no_clobber = 0 as libc::c_int;
    (*conf).search_timeout = 10 as libc::c_int;
    (*conf).search_threads = 3 as libc::c_int;
    (*conf).search_amount = 15 as libc::c_int;
    (*conf).search_top = 3 as libc::c_int;
    (*conf).ai_family = 0 as libc::c_int as sa_family_t;
    conf_hdr_make(
        ((*conf).add_header[HDR_USER_AGENT as libc::c_int as usize]).as_mut_ptr(),
        b"User-Agent\0" as *const u8 as *const libc::c_char,
        b"Axel/2.17.11 (linux-gnu)\0" as *const u8 as *const libc::c_char,
    );
    (*conf).add_header_count = HDR_count_init as libc::c_int;
    (*conf)
        .interfaces = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<if_t>() as libc::c_ulong,
    ) as *mut if_t;
    if ((*conf).interfaces).is_null() {
        return 0 as libc::c_int;
    }
    (*(*conf).interfaces).next = (*conf).interfaces as *mut libc::c_void;
    (*conf).alternate_output = isatty(1 as libc::c_int);
    s2 = getenv(b"http_proxy\0" as *const u8 as *const libc::c_char);
    if !s2.is_null()
        || {
            s2 = getenv(b"HTTP_PROXY\0" as *const u8 as *const libc::c_char);
            !s2.is_null()
        }
    {
        strlcpy(
            ((*conf).http_proxy).as_mut_ptr(),
            s2,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    if conf_loadfile(conf, b"/etc/axelrc\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    s2 = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if !s2.is_null() {
        let mut s: [libc::c_char; 1024] = [0; 1024];
        let mut ret: libc::c_int = 0;
        ret = snprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"%s/.axelrc\0" as *const u8 as *const libc::c_char,
            s2,
        );
        if ret
            >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"HOME env variable too long\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if conf_loadfile(conf, s.as_mut_ptr()) == 0 {
            return 0 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while (*conf).no_proxy[i as usize] != 0 {
        if (*conf).no_proxy[i as usize] as libc::c_int == ',' as i32 {
            (*conf).no_proxy[i as usize] = 0 as libc::c_int as libc::c_char;
        }
        i += 1;
        i;
    }
    (*conf).no_proxy[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conf_free(mut conf: *mut conf_t) {
    free((*conf).interfaces as *mut libc::c_void);
}
pub unsafe extern "C" fn parse_interfaces(
    mut conf: *mut conf_t,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iface: *mut if_t = 0 as *mut if_t;
    iface = (*(*conf).interfaces).next as *mut if_t;
    while iface != (*conf).interfaces {
        let mut i: *mut if_t = 0 as *mut if_t;
        i = (*iface).next as *mut if_t;
        free(iface as *mut libc::c_void);
        iface = i;
    }
    free((*conf).interfaces as *mut libc::c_void);
    if *s == 0 {
        (*conf)
            .interfaces = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<if_t>() as libc::c_ulong,
        ) as *mut if_t;
        if ((*conf).interfaces).is_null() {
            return 0 as libc::c_int;
        }
        (*(*conf).interfaces).next = (*conf).interfaces as *mut libc::c_void;
        return 1 as libc::c_int;
    }
    *s
        .offset(
            (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    iface = malloc(::std::mem::size_of::<if_t>() as libc::c_ulong) as *mut if_t;
    (*conf).interfaces = iface;
    if ((*conf).interfaces).is_null() {
        return 0 as libc::c_int;
    }
    loop {
        while (*s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32)
            && *s as libc::c_int != 0
        {
            s = s.offset(1);
            s;
        }
        s2 = s;
        while *s2 as libc::c_int != ' ' as i32 && *s2 as libc::c_int != '\t' as i32
            && *s2 as libc::c_int != 0
        {
            s2 = s2.offset(1);
            s2;
        }
        *s2 = 0 as libc::c_int as libc::c_char;
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
            get_if_ip(
                ((*iface).text).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                s,
            );
        } else {
            strlcpy(
                ((*iface).text).as_mut_ptr(),
                s,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
        s = s2.offset(1 as libc::c_int as isize);
        if *s != 0 {
            (*iface).next = malloc(::std::mem::size_of::<if_t>() as libc::c_ulong);
            if ((*iface).next).is_null() {
                return 0 as libc::c_int;
            }
            iface = (*iface).next as *mut if_t;
        } else {
            (*iface).next = (*conf).interfaces as *mut libc::c_void;
            break;
        }
    }
    return 1 as libc::c_int;
}
