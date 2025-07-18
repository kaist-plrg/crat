use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_stream_printf(
        label: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        reason: *const libc::c_char,
    );
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
pub type gsl_error_handler_t = unsafe extern "C" fn(
    *const libc::c_char,
    *const libc::c_char,
    libc::c_int,
    libc::c_int,
) -> ();
pub static mut gsl_error_handler: Option::<gsl_error_handler_t> = None;
pub unsafe extern "C" fn gsl_error(
    mut reason: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut gsl_errno: libc::c_int,
) {
    if gsl_error_handler.is_some() {
        (Some(gsl_error_handler.unwrap())).unwrap()(reason, file, line, gsl_errno);
        return;
    }
    gsl_stream_printf(
        b"ERROR\0" as *const u8 as *const libc::c_char,
        file,
        line,
        reason,
    );
    fflush(stdout);
    fprintf(
        stderr,
        b"Default GSL error handler invoked.\n\0" as *const u8 as *const libc::c_char,
    );
    fflush(stderr);
    abort();
}
pub unsafe extern "C" fn gsl_set_error_handler(
    mut new_handler: Option::<gsl_error_handler_t>,
) -> Option::<gsl_error_handler_t> {
    let mut previous_handler: Option::<gsl_error_handler_t> = gsl_error_handler;
    gsl_error_handler = new_handler;
    return previous_handler;
}
pub unsafe extern "C" fn gsl_set_error_handler_off() -> Option::<gsl_error_handler_t> {
    let mut previous_handler: Option::<gsl_error_handler_t> = gsl_error_handler;
    gsl_error_handler = Some(
        no_error_handler
            as unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_char,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    return previous_handler;
}
unsafe extern "C" fn no_error_handler(
    mut reason: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut gsl_errno: libc::c_int,
) {
    reason = 0 as *const libc::c_char;
    file = 0 as *const libc::c_char;
    line = 0 as libc::c_int;
    gsl_errno = 0 as libc::c_int;
}
