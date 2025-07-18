use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    static mut in_name: *mut libc::c_char;
    fn cleanup_user_specials();
    static mut line_no: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn uninit_parser();
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
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
pub type exit_values = libc::c_uint;
pub const system_error: exit_values = 5;
pub const indent_fatal: exit_values = 4;
pub const indent_punt: exit_values = 3;
pub const indent_error: exit_values = 2;
pub const invocation_error: exit_values = 64;
pub const total_success: exit_values = 0;
pub unsafe extern "C" fn xmalloc(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut val: *mut libc::c_void = calloc(
        1 as libc::c_int as libc::c_ulong,
        size as libc::c_ulong,
    );
    if val.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"indent: Virtual memory exhausted.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        do_exit(system_error as libc::c_int);
    }
    return val;
}
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: libc::c_uint,
) -> *mut libc::c_void {
    let mut val: *mut libc::c_void = realloc(ptr, size as libc::c_ulong);
    if val.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"indent: Virtual memory exhausted.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        do_exit(system_error as libc::c_int);
    }
    return val;
}
pub unsafe extern "C" fn xfree(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    free(ptr);
}
pub unsafe extern "C" fn message(
    mut kind: *mut libc::c_char,
    mut string: *mut libc::c_char,
    mut a0: *mut libc::c_char,
    mut a1: *mut libc::c_char,
) {
    if !kind.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"indent: %s:%d: %s:\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            in_name,
            line_no,
            kind,
        );
    }
    fprintf(stderr, string, a0, a1);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn do_exit(mut code: libc::c_int) {
    uninit_parser();
    cleanup_user_specials();
    exit(code);
}
pub unsafe extern "C" fn fatal(
    mut string: *const libc::c_char,
    mut a0: *const libc::c_char,
) {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"indent: Fatal Error: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fprintf(stderr, string, a0);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if *__errno_location() != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"indent: System Error: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        perror(0 as *const libc::c_char);
    }
    do_exit(indent_fatal as libc::c_int);
}
