use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
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
unsafe extern "C" fn orig_fopen(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    return fopen(filename, mode);
}
pub unsafe extern "C" fn rpl_fopen(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut open_direction: libc::c_int = 0;
    let mut open_flags: libc::c_int = 0;
    let mut open_flags_gnu: bool = false;
    let mut fdopen_mode_buf: [libc::c_char; 81] = [0; 81];
    open_direction = 0 as libc::c_int;
    open_flags = 0 as libc::c_int;
    open_flags_gnu = 0 as libc::c_int != 0;
    let mut p: *const libc::c_char = mode;
    let mut q: *mut libc::c_char = fdopen_mode_buf.as_mut_ptr();
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            114 => {
                open_direction = 0 as libc::c_int;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as libc::c_int as isize) {
                    let fresh0 = q;
                    q = q.offset(1);
                    *fresh0 = *p;
                }
            }
            119 => {
                open_direction = 0o1 as libc::c_int;
                open_flags |= 0o100 as libc::c_int | 0o1000 as libc::c_int;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as libc::c_int as isize) {
                    let fresh1 = q;
                    q = q.offset(1);
                    *fresh1 = *p;
                }
            }
            97 => {
                open_direction = 0o1 as libc::c_int;
                open_flags |= 0o100 as libc::c_int | 0o2000 as libc::c_int;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as libc::c_int as isize) {
                    let fresh2 = q;
                    q = q.offset(1);
                    *fresh2 = *p;
                }
            }
            98 => {
                open_flags |= 0 as libc::c_int;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as libc::c_int as isize) {
                    let fresh3 = q;
                    q = q.offset(1);
                    *fresh3 = *p;
                }
            }
            43 => {
                open_direction = 0o2 as libc::c_int;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as libc::c_int as isize) {
                    let fresh4 = q;
                    q = q.offset(1);
                    *fresh4 = *p;
                }
            }
            120 => {
                open_flags |= 0o200 as libc::c_int;
                open_flags_gnu = 1 as libc::c_int != 0;
            }
            101 => {
                open_flags |= 0o2000000 as libc::c_int;
                open_flags_gnu = 1 as libc::c_int != 0;
            }
            _ => {
                let mut len: size_t = strlen(p);
                if len
                    > fdopen_mode_buf
                        .as_mut_ptr()
                        .offset(80 as libc::c_int as isize)
                        .offset_from(q) as libc::c_long as libc::c_ulong
                {
                    len = fdopen_mode_buf
                        .as_mut_ptr()
                        .offset(80 as libc::c_int as isize)
                        .offset_from(q) as libc::c_long as size_t;
                }
                memcpy(q as *mut libc::c_void, p as *const libc::c_void, len);
                q = q.offset(len as isize);
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    *q = '\0' as i32 as libc::c_char;
    if open_flags_gnu {
        let mut fd: libc::c_int = 0;
        let mut fp: *mut FILE = 0 as *mut FILE;
        fd = open(
            filename,
            open_direction | open_flags,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if fd < 0 as libc::c_int {
            return 0 as *mut FILE;
        }
        fp = fdopen(fd, fdopen_mode_buf.as_mut_ptr());
        if fp.is_null() {
            let mut saved_errno: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = saved_errno;
        }
        return fp;
    }
    return orig_fopen(filename, mode);
}
