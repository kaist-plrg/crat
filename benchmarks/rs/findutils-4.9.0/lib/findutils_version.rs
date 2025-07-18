use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub static mut version_string: *const libc::c_char = b"4.9.0\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn display_findutils_version(
    mut official_name: *const libc::c_char,
) {
    rpl_fflush(stderr);
    version_etc(
        stdout,
        official_name,
        b"GNU findutils\0" as *const u8 as *const libc::c_char,
        version_string,
        dcgettext(
            0 as *const libc::c_char,
            b"Eric B. Decker\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"James Youngman\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Kevin Dalley\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        0 as *mut libc::c_void as *const libc::c_char,
    );
}
