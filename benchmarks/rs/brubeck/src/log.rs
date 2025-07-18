use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn vsyslog(__pri: libc::c_int, __fmt: *const libc::c_char, __ap: ::std::ffi::VaList);
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
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
static mut gh_log_path: *const libc::c_char = 0 as *const libc::c_char;
static mut gh_log_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut gh_syslog_enabled: libc::c_int = 0;
pub unsafe extern "C" fn gh_log_open(mut path: *const libc::c_char) {
    let mut new_log: *mut FILE = 0 as *mut FILE;
    if path.is_null() {
        gh_syslog_enabled = 0 as libc::c_int;
        gh_log_file = 0 as *mut FILE;
        return;
    }
    if strcmp(path, b"syslog\0" as *const u8 as *const libc::c_char) == 0 {
        openlog(
            0 as *const libc::c_char,
            0x1 as libc::c_int,
            (23 as libc::c_int) << 3 as libc::c_int,
        );
        gh_syslog_enabled = 1 as libc::c_int;
        return;
    }
    new_log = fopen(path, b"a\0" as *const u8 as *const libc::c_char);
    if new_log.is_null() {
        fprintf(
            stderr,
            b"Failed to open log file at '%s'\n\0" as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    if !gh_log_file.is_null() {
        fclose(gh_log_file);
    }
    if gh_syslog_enabled != 0 {
        closelog();
        gh_syslog_enabled = 0 as libc::c_int;
    }
    gh_log_file = new_log;
    gh_log_path = path;
}
pub unsafe extern "C" fn gh_log_reopen() {
    if !gh_log_path.is_null() {
        gh_log_open(gh_log_path);
    }
}
pub unsafe extern "C" fn gh_log_write(mut message: *const libc::c_char, mut args: ...) {
    let mut vl: ::std::ffi::VaListImpl;
    vl = args.clone();
    if gh_syslog_enabled != 0 {
        vsyslog(6 as libc::c_int, message, vl.as_va_list());
    } else if !gh_log_file.is_null() {
        vfprintf(gh_log_file, message, vl.as_va_list());
        fflush(gh_log_file);
    } else {
        vfprintf(stderr, message, vl.as_va_list());
        fflush(stderr);
    };
}
pub unsafe extern "C" fn gh_log_die() -> ! {
    exit(1 as libc::c_int);
}
static mut _app_instance: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn gh_log_instance() -> *const libc::c_char {
    return _app_instance;
}
pub unsafe extern "C" fn gh_log_set_instance(mut instance: *const libc::c_char) {
    _app_instance = instance;
}
