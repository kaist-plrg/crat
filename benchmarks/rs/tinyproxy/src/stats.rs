use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    pub type buffer_s;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut config: *mut config_s;
    fn add_error_variable(
        connptr: *mut conn_s,
        key: *const libc::c_char,
        val: *const libc::c_char,
    ) -> libc::c_int;
    fn send_html_file(infile: *mut FILE, connptr: *mut conn_s) -> libc::c_int;
    fn send_http_headers(
        connptr: *mut conn_s,
        code: libc::c_int,
        message: *const libc::c_char,
        extra: *const libc::c_char,
    ) -> libc::c_int;
    fn add_standard_vars(connptr: *mut conn_s) -> libc::c_int;
    fn send_http_message(
        connptr: *mut conn_s,
        http_code: libc::c_int,
        error_title: *const libc::c_char,
        message: *const libc::c_char,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
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
pub type status_t = libc::c_uint;
pub const STAT_DENIED: status_t = 4;
pub const STAT_REFUSE: status_t = 3;
pub const STAT_CLOSE: status_t = 2;
pub const STAT_OPEN: status_t = 1;
pub const STAT_BADCONN: status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_s {
    pub num_reqs: libc::c_ulong,
    pub num_badcons: libc::c_ulong,
    pub num_open: libc::c_ulong,
    pub num_refused: libc::c_ulong,
    pub num_denied: libc::c_ulong,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_1 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_1 = 1;
static mut stats_buf: stat_s = stat_s {
    num_reqs: 0,
    num_badcons: 0,
    num_open: 0,
    num_refused: 0,
    num_denied: 0,
};
static mut stats: *mut stat_s = 0 as *const stat_s as *mut stat_s;
static mut stats_update_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut stats_file_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub unsafe extern "C" fn init_stats() {
    stats = &mut stats_buf;
}
pub unsafe extern "C" fn showstats(mut connptr: *mut conn_s) -> libc::c_int {
    let mut message_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opens: [libc::c_char; 16] = [0; 16];
    let mut reqs: [libc::c_char; 16] = [0; 16];
    let mut badconns: [libc::c_char; 16] = [0; 16];
    let mut denied: [libc::c_char; 16] = [0; 16];
    let mut refused: [libc::c_char; 16] = [0; 16];
    let mut statfile: *mut FILE = 0 as *mut FILE;
    snprintf(
        opens.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        (*stats).num_open,
    );
    snprintf(
        reqs.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        (*stats).num_reqs,
    );
    snprintf(
        badconns.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        (*stats).num_badcons,
    );
    snprintf(
        denied.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        (*stats).num_denied,
    );
    snprintf(
        refused.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        (*stats).num_refused,
    );
    pthread_mutex_lock(&mut stats_file_lock);
    if ((*config).statpage).is_null()
        || {
            statfile = fopen(
                (*config).statpage,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            statfile.is_null()
        }
    {
        message_buffer = malloc((1024 as libc::c_int * 96 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if !message_buffer.is_null() {
            snprintf(
                message_buffer,
                (1024 as libc::c_int * 96 as libc::c_int) as size_t,
                b"<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">\n<html>\n<head><title>%s version %s run-time statistics</title></head>\n<body>\n<h1>%s version %s run-time statistics</h1>\n<p>\nNumber of open connections: %lu<br />\nNumber of requests: %lu<br />\nNumber of bad connections: %lu<br />\nNumber of denied connections: %lu<br />\nNumber of refused connections due to high load: %lu\n</p>\n<hr />\n<p><em>Generated by %s version %s.</em></p>\n</body>\n</html>\n\0"
                    as *const u8 as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                b"1.11.2\0" as *const u8 as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                b"1.11.2\0" as *const u8 as *const libc::c_char,
                (*stats).num_open,
                (*stats).num_reqs,
                (*stats).num_badcons,
                (*stats).num_denied,
                (*stats).num_refused,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                b"1.11.2\0" as *const u8 as *const libc::c_char,
            );
            if send_http_message(
                connptr,
                200 as libc::c_int,
                b"OK\0" as *const u8 as *const libc::c_char,
                message_buffer,
            ) < 0 as libc::c_int
            {
                free(message_buffer as *mut libc::c_void);
                message_buffer = 0 as *mut libc::c_char;
            } else {
                free(message_buffer as *mut libc::c_void);
                message_buffer = 0 as *mut libc::c_char;
                pthread_mutex_unlock(&mut stats_file_lock);
                return 0 as libc::c_int;
            }
        }
        pthread_mutex_unlock(&mut stats_file_lock);
        return -(1 as libc::c_int);
    }
    add_error_variable(
        connptr,
        b"opens\0" as *const u8 as *const libc::c_char,
        opens.as_mut_ptr(),
    );
    add_error_variable(
        connptr,
        b"reqs\0" as *const u8 as *const libc::c_char,
        reqs.as_mut_ptr(),
    );
    add_error_variable(
        connptr,
        b"badconns\0" as *const u8 as *const libc::c_char,
        badconns.as_mut_ptr(),
    );
    add_error_variable(
        connptr,
        b"deniedconns\0" as *const u8 as *const libc::c_char,
        denied.as_mut_ptr(),
    );
    add_error_variable(
        connptr,
        b"refusedconns\0" as *const u8 as *const libc::c_char,
        refused.as_mut_ptr(),
    );
    add_standard_vars(connptr);
    send_http_headers(
        connptr,
        200 as libc::c_int,
        b"Statistic requested\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    send_html_file(statfile, connptr);
    fclose(statfile);
    pthread_mutex_unlock(&mut stats_file_lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn update_stats(mut update_level: status_t) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    pthread_mutex_lock(&mut stats_update_lock);
    match update_level as libc::c_uint {
        0 => {
            (*stats).num_badcons = ((*stats).num_badcons).wrapping_add(1);
            (*stats).num_badcons;
        }
        1 => {
            (*stats).num_open = ((*stats).num_open).wrapping_add(1);
            (*stats).num_open;
            (*stats).num_reqs = ((*stats).num_reqs).wrapping_add(1);
            (*stats).num_reqs;
        }
        2 => {
            (*stats).num_open = ((*stats).num_open).wrapping_sub(1);
            (*stats).num_open;
        }
        3 => {
            (*stats).num_refused = ((*stats).num_refused).wrapping_add(1);
            (*stats).num_refused;
        }
        4 => {
            (*stats).num_denied = ((*stats).num_denied).wrapping_add(1);
            (*stats).num_denied;
        }
        _ => {
            ret = -(1 as libc::c_int);
        }
    }
    pthread_mutex_unlock(&mut stats_update_lock);
    return ret;
}
