use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    pub type buffer_s;
    pub type re_dfa_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    static mut config: *mut config_s;
    fn htab_create(_: size_t) -> *mut htab;
    fn htab_find(_: *mut htab, key: *const libc::c_char) -> *mut htab_value;
    fn htab_insert(_: *mut htab, _: *mut libc::c_char, _: htab_value) -> libc::c_int;
    fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
    fn write_message(fd: libc::c_int, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_list_t = *mut sblist;
#[derive(Copy, Clone)]
#[repr(C)]
pub union htab_value {
    pub p: *mut libc::c_void,
    pub n: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_s {
    pub client_fd: libc::c_int,
    pub server_fd: libc::c_int,
    pub cbuffer: *mut buffer_s,
    pub sbuffer: *mut buffer_s,
    pub request_line: *mut libc::c_char,
    pub connect_method: libc::c_uint,
    pub show_stats: libc::c_uint,
    pub error_variables: *mut htab,
    pub error_number: libc::c_int,
    pub error_string: *mut libc::c_char,
    pub content_length: C2RustUnnamed_0,
    pub server_ip_addr: *mut libc::c_char,
    pub client_ip_addr: *mut libc::c_char,
    pub protocol: C2RustUnnamed,
    pub reversepath: *mut libc::c_char,
    pub upstream_proxy: *mut upstream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub major: libc::c_uint,
    pub minor: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub server: libc::c_long,
    pub client: libc::c_long,
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type reg_syntax_t = libc::c_ulong;
pub type __re_long_size_t = libc::c_ulong;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub unsafe extern "C" fn add_new_errorpage(
    mut conf: *mut config_s,
    mut filepath: *mut libc::c_char,
    mut errornum: libc::c_uint,
) -> libc::c_int {
    let mut errornbuf: [libc::c_char; 8] = [0; 8];
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*conf).errorpages).is_null() {
        (*conf).errorpages = htab_create(16 as libc::c_int as size_t);
    }
    if ((*conf).errorpages).is_null() {
        return -(1 as libc::c_int);
    }
    snprintf(
        errornbuf.as_mut_ptr(),
        8 as libc::c_int as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        errornum,
    );
    k = strdup(errornbuf.as_mut_ptr());
    if k.is_null() {
        return -(1 as libc::c_int);
    }
    if htab_insert(
        (*conf).errorpages,
        k,
        htab_value {
            p: filepath as *mut libc::c_void,
        },
    ) == 0
    {
        free(k as *mut libc::c_void);
        k = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_html_file(mut errornum: libc::c_uint) -> *mut libc::c_char {
    let mut errornbuf: [libc::c_char; 8] = [0; 8];
    let mut hv: *mut htab_value = 0 as *mut htab_value;
    if ((*config).errorpages).is_null() {
        return (*config).errorpage_undef;
    }
    snprintf(
        errornbuf.as_mut_ptr(),
        8 as libc::c_int as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        errornum,
    );
    hv = htab_find((*config).errorpages, errornbuf.as_mut_ptr());
    if hv.is_null() {
        return (*config).errorpage_undef;
    }
    return (*hv).p as *mut libc::c_char;
}
unsafe extern "C" fn lookup_variable(
    mut map: *mut htab,
    mut varname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut v: *mut htab_value = 0 as *mut htab_value;
    v = htab_find(map, varname);
    return (if !v.is_null() { (*v).p } else { 0 as *mut libc::c_void })
        as *mut libc::c_char;
}
unsafe extern "C" fn varsubst_sendline(
    mut connptr: *mut conn_s,
    mut re: *mut regex_t,
    mut p: *mut libc::c_char,
) {
    let mut fd: libc::c_int = (*connptr).client_fd;
    while *p != 0 {
        let mut match_0: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
        let mut varname: [libc::c_char; 33] = [0; 33];
        let mut varval: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut l: size_t = 0;
        let mut st: libc::c_int = regexec(
            re,
            p,
            1 as libc::c_int as size_t,
            &mut match_0,
            0 as libc::c_int,
        );
        if st == 0 as libc::c_int {
            if match_0.rm_so > 0 as libc::c_int {
                safe_write(fd, p as *const libc::c_void, match_0.rm_so as size_t);
            }
            l = (match_0.rm_eo - match_0.rm_so) as size_t;
            p = p.offset(match_0.rm_so as isize);
            memcpy(
                varname.as_mut_ptr() as *mut libc::c_void,
                p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                l.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            varname[l.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as usize] = 0 as libc::c_int as libc::c_char;
            varval = lookup_variable((*connptr).error_variables, varname.as_mut_ptr());
            if !varval.is_null() {
                write_message(fd, b"%s\0" as *const u8 as *const libc::c_char, varval);
            } else if !varval.is_null() && *varval == 0 {
                write_message(fd, b"(unknown)\0" as *const u8 as *const libc::c_char);
            } else {
                safe_write(fd, p as *const libc::c_void, l);
            }
            p = p.offset(l as isize);
        } else {
            write_message(fd, b"%s\0" as *const u8 as *const libc::c_char, p);
            break;
        }
    }
}
pub unsafe extern "C" fn send_html_file(
    mut infile: *mut FILE,
    mut connptr: *mut conn_s,
) -> libc::c_int {
    let mut re: regex_t = regex_t {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut inbuf: *mut libc::c_char = malloc(4096 as libc::c_int as libc::c_ulong)
        as *mut libc::c_char;
    regcomp(
        &mut re,
        b"{[a-z]\\{1,32\\}}\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    while !(fgets(inbuf, 4096 as libc::c_int, infile)).is_null() {
        varsubst_sendline(connptr, &mut re, inbuf);
    }
    regfree(&mut re);
    free(inbuf as *mut libc::c_void);
    inbuf = 0 as *mut libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn send_http_headers(
    mut connptr: *mut conn_s,
    mut code: libc::c_int,
    mut message: *const libc::c_char,
    mut extra: *const libc::c_char,
) -> libc::c_int {
    let headers: [libc::c_char; 81] = *::std::mem::transmute::<
        &[u8; 81],
        &[libc::c_char; 81],
    >(
        b"HTTP/1.%u %d %s\r\nServer: %s/%s\r\nContent-Type: text/html\r\n%sConnection: close\r\n\r\n\0",
    );
    return write_message(
        (*connptr).client_fd,
        headers.as_ptr(),
        if (*connptr).protocol.major != 1 as libc::c_int as libc::c_uint {
            0 as libc::c_int as libc::c_uint
        } else {
            (*connptr).protocol.minor
        },
        code,
        message,
        b"tinyproxy\0" as *const u8 as *const libc::c_char,
        b"1.11.2\0" as *const u8 as *const libc::c_char,
        extra,
    );
}
pub unsafe extern "C" fn send_http_error_message(
    mut connptr: *mut conn_s,
) -> libc::c_int {
    let mut error_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut infile: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = 0;
    let mut fallback_error: *const libc::c_char = b"<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">\n<html>\n<head><title>%d %s</title></head>\n<body>\n<h1>%s</h1>\n<p>%s</p>\n<hr />\n<p><em>Generated by %s version %s.</em></p>\n</body>\n</html>\n\0"
        as *const u8 as *const libc::c_char;
    let p_auth_str: [libc::c_char; 46] = *::std::mem::transmute::<
        &[u8; 46],
        &[libc::c_char; 46],
    >(b"Proxy-Authenticate: Basic realm=\"Tinyproxy\"\r\n\0");
    let w_auth_str: [libc::c_char; 44] = *::std::mem::transmute::<
        &[u8; 44],
        &[libc::c_char; 44],
    >(b"WWW-Authenticate: Basic realm=\"Tinyproxy\"\r\n\0");
    let mut add: *const libc::c_char = if (*connptr).error_number == 407 as libc::c_int {
        p_auth_str.as_ptr()
    } else if (*connptr).error_number == 401 as libc::c_int {
        w_auth_str.as_ptr()
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    send_http_headers(connptr, (*connptr).error_number, (*connptr).error_string, add);
    error_file = get_html_file((*connptr).error_number as libc::c_uint);
    if error_file.is_null()
        || {
            infile = fopen(error_file, b"r\0" as *const u8 as *const libc::c_char);
            infile.is_null()
        }
    {
        let mut detail: *mut libc::c_char = 0 as *mut libc::c_char;
        if !error_file.is_null() {
            log_message(
                3 as libc::c_int,
                b"Error opening error file '%s' (%s)\0" as *const u8
                    as *const libc::c_char,
                error_file,
                strerror(*__errno_location()),
            );
        }
        detail = lookup_variable(
            (*connptr).error_variables,
            b"detail\0" as *const u8 as *const libc::c_char,
        );
        return write_message(
            (*connptr).client_fd,
            fallback_error,
            (*connptr).error_number,
            (*connptr).error_string,
            (*connptr).error_string,
            detail,
            b"tinyproxy\0" as *const u8 as *const libc::c_char,
            b"1.11.2\0" as *const u8 as *const libc::c_char,
        );
    }
    ret = send_html_file(infile, connptr);
    fclose(infile);
    return ret;
}
pub unsafe extern "C" fn add_error_variable(
    mut connptr: *mut conn_s,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*connptr).error_variables).is_null() {
        (*connptr).error_variables = htab_create(16 as libc::c_int as size_t);
        if ((*connptr).error_variables).is_null() {
            return -(1 as libc::c_int);
        }
    }
    k = strdup(key);
    v = strdup(val);
    if !(v.is_null() || k.is_null()) {
        if htab_insert(
            (*connptr).error_variables,
            k,
            htab_value {
                p: v as *mut libc::c_void,
            },
        ) != 0
        {
            return 1 as libc::c_int;
        }
    }
    free(k as *mut libc::c_void);
    k = 0 as *mut libc::c_char;
    free(v as *mut libc::c_void);
    v = 0 as *mut libc::c_char;
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn add_standard_vars(mut connptr: *mut conn_s) -> libc::c_int {
    let mut errnobuf: [libc::c_char; 16] = [0; 16];
    let mut timebuf: [libc::c_char; 30] = [0; 30];
    let mut global_time: time_t = 0;
    let mut tm_buf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    snprintf(
        errnobuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*connptr).error_number,
    );
    if !errnobuf.as_mut_ptr().is_null() {
        if add_error_variable(
            connptr,
            b"errno\0" as *const u8 as *const libc::c_char,
            errnobuf.as_mut_ptr(),
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    if !((*connptr).error_string).is_null() {
        if add_error_variable(
            connptr,
            b"cause\0" as *const u8 as *const libc::c_char,
            (*connptr).error_string,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    if !((*connptr).request_line).is_null() {
        if add_error_variable(
            connptr,
            b"request\0" as *const u8 as *const libc::c_char,
            (*connptr).request_line,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    if !((*connptr).client_ip_addr).is_null() {
        if add_error_variable(
            connptr,
            b"clientip\0" as *const u8 as *const libc::c_char,
            (*connptr).client_ip_addr,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    global_time = time(0 as *mut time_t);
    strftime(
        timebuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
        b"%a, %d %b %Y %H:%M:%S GMT\0" as *const u8 as *const libc::c_char,
        gmtime_r(&mut global_time, &mut tm_buf),
    );
    add_error_variable(
        connptr,
        b"date\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
    );
    add_error_variable(
        connptr,
        b"website\0" as *const u8 as *const libc::c_char,
        b"https://tinyproxy.github.io/\0" as *const u8 as *const libc::c_char,
    );
    add_error_variable(
        connptr,
        b"version\0" as *const u8 as *const libc::c_char,
        b"1.11.2\0" as *const u8 as *const libc::c_char,
    );
    add_error_variable(
        connptr,
        b"package\0" as *const u8 as *const libc::c_char,
        b"tinyproxy\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn indicate_http_error(
    mut connptr: *mut conn_s,
    mut number: libc::c_int,
    mut message: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    loop {
        key = ap.arg::<*mut libc::c_char>();
        if key.is_null() {
            break;
        }
        val = ap.arg::<*mut libc::c_char>();
        if add_error_variable(connptr, key, val) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    (*connptr).error_number = number;
    (*connptr).error_string = strdup(message);
    return add_standard_vars(connptr);
}
