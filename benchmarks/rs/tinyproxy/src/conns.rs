use ::libc;
extern "C" {
    pub type buffer_s;
    pub type htab;
    pub type upstream;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn new_buffer() -> *mut buffer_s;
    fn delete_buffer(buffptr: *mut buffer_s);
    fn htab_destroy(_: *mut htab);
    fn htab_next(
        _: *mut htab,
        iterator: size_t,
        key: *mut *mut libc::c_char,
        v: *mut *mut htab_value,
    ) -> size_t;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn update_stats(update_level: status_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type status_t = libc::c_uint;
pub const STAT_DENIED: status_t = 4;
pub const STAT_REFUSE: status_t = 3;
pub const STAT_CLOSE: status_t = 2;
pub const STAT_OPEN: status_t = 1;
pub const STAT_BADCONN: status_t = 0;
pub unsafe extern "C" fn conn_struct_init(mut connptr: *mut conn_s) {
    (*connptr).error_number = -(1 as libc::c_int);
    (*connptr).client_fd = -(1 as libc::c_int);
    (*connptr).server_fd = -(1 as libc::c_int);
    (*connptr).content_length.client = -(1 as libc::c_int) as libc::c_long;
    (*connptr).content_length.server = (*connptr).content_length.client;
}
pub unsafe extern "C" fn conn_init_contents(
    mut connptr: *mut conn_s,
    mut ipaddr: *const libc::c_char,
    mut sock_ipaddr: *const libc::c_char,
) -> libc::c_int {
    let mut cbuffer: *mut buffer_s = 0 as *mut buffer_s;
    let mut sbuffer: *mut buffer_s = 0 as *mut buffer_s;
    cbuffer = new_buffer();
    sbuffer = new_buffer();
    if cbuffer.is_null() || sbuffer.is_null() {
        if !cbuffer.is_null() {
            delete_buffer(cbuffer);
        }
        if !sbuffer.is_null() {
            delete_buffer(sbuffer);
        }
        return 0 as libc::c_int;
    } else {
        (*connptr).cbuffer = cbuffer;
        (*connptr).sbuffer = sbuffer;
        (*connptr)
            .server_ip_addr = if !sock_ipaddr.is_null() {
            strdup(sock_ipaddr)
        } else {
            0 as *mut libc::c_char
        };
        (*connptr).client_ip_addr = strdup(ipaddr);
        update_stats(STAT_OPEN);
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn conn_destroy_contents(mut connptr: *mut conn_s) {
    if (*connptr).client_fd != -(1 as libc::c_int) {
        if close((*connptr).client_fd) < 0 as libc::c_int {
            log_message(
                6 as libc::c_int,
                b"Client (%d) close message: %s\0" as *const u8 as *const libc::c_char,
                (*connptr).client_fd,
                strerror(*__errno_location()),
            );
        }
    }
    (*connptr).client_fd = -(1 as libc::c_int);
    if (*connptr).server_fd != -(1 as libc::c_int) {
        if close((*connptr).server_fd) < 0 as libc::c_int {
            log_message(
                6 as libc::c_int,
                b"Server (%d) close message: %s\0" as *const u8 as *const libc::c_char,
                (*connptr).server_fd,
                strerror(*__errno_location()),
            );
        }
    }
    (*connptr).server_fd = -(1 as libc::c_int);
    if !((*connptr).cbuffer).is_null() {
        delete_buffer((*connptr).cbuffer);
    }
    if !((*connptr).sbuffer).is_null() {
        delete_buffer((*connptr).sbuffer);
    }
    if !((*connptr).request_line).is_null() {
        free((*connptr).request_line as *mut libc::c_void);
        (*connptr).request_line = 0 as *mut libc::c_char;
    }
    if !((*connptr).error_variables).is_null() {
        let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut v: *mut htab_value = 0 as *mut htab_value;
        let mut it: size_t = 0 as libc::c_int as size_t;
        loop {
            it = htab_next((*connptr).error_variables, it, &mut k, &mut v);
            if !(it != 0) {
                break;
            }
            free((*v).p);
            (*v).p = 0 as *mut libc::c_void;
            free(k as *mut libc::c_void);
            k = 0 as *mut libc::c_char;
        }
        htab_destroy((*connptr).error_variables);
    }
    if !((*connptr).error_string).is_null() {
        free((*connptr).error_string as *mut libc::c_void);
        (*connptr).error_string = 0 as *mut libc::c_char;
    }
    if !((*connptr).server_ip_addr).is_null() {
        free((*connptr).server_ip_addr as *mut libc::c_void);
        (*connptr).server_ip_addr = 0 as *mut libc::c_char;
    }
    if !((*connptr).client_ip_addr).is_null() {
        free((*connptr).client_ip_addr as *mut libc::c_void);
        (*connptr).client_ip_addr = 0 as *mut libc::c_char;
    }
    if !((*connptr).reversepath).is_null() {
        free((*connptr).reversepath as *mut libc::c_void);
        (*connptr).reversepath = 0 as *mut libc::c_char;
    }
    update_stats(STAT_CLOSE);
}
