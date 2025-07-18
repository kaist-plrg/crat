use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut program_invocation_name: *mut libc::c_char;
    static mut program_invocation_short_name: *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn abort() -> !;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub static mut program_name: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn set_program_name(mut argv0: *const libc::c_char) {
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    if argv0.is_null() {
        fputs(
            b"A NULL argv[0] was passed through an exec system call.\n\0" as *const u8
                as *const libc::c_char,
            stderr,
        );
        abort();
    }
    slash = strrchr(argv0, '/' as i32);
    base = if !slash.is_null() {
        slash.offset(1 as libc::c_int as isize)
    } else {
        argv0
    };
    if base.offset_from(argv0) as libc::c_long >= 7 as libc::c_int as libc::c_long
        && strncmp(
            base.offset(-(7 as libc::c_int as isize)),
            b"/.libs/\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        argv0 = base;
        if strncmp(
            base,
            b"lt-\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            argv0 = base.offset(3 as libc::c_int as isize);
            program_invocation_short_name = argv0 as *mut libc::c_char;
        }
    }
    program_name = argv0;
    program_invocation_name = argv0 as *mut libc::c_char;
}
