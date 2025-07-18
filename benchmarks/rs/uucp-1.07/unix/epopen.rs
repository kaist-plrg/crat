use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn ixsspawn(
        pazargs: *mut *const libc::c_char,
        aidescs: *mut libc::c_int,
        fkeepuid: boolean,
        fkeepenv: boolean,
        zchdir: *const libc::c_char,
        fnosigs: boolean,
        fshell: boolean,
        zpath: *const libc::c_char,
        zuu_machine: *const libc::c_char,
        zuu_user: *const libc::c_char,
    ) -> pid_t;
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type pid_t = __pid_t;
pub type boolean = libc::c_int;
pub unsafe extern "C" fn espopen(
    mut pazargs: *mut *const libc::c_char,
    mut frd: boolean,
    mut pipid: *mut pid_t,
) -> *mut FILE {
    let mut aidescs: [libc::c_int; 3] = [0; 3];
    let mut ipid: pid_t = 0;
    let mut eret: *mut FILE = 0 as *mut FILE;
    if frd != 0 {
        aidescs[0 as libc::c_int as usize] = -(1 as libc::c_int);
        aidescs[1 as libc::c_int as usize] = -(2 as libc::c_int);
    } else {
        aidescs[0 as libc::c_int as usize] = -(3 as libc::c_int);
        aidescs[1 as libc::c_int as usize] = -(1 as libc::c_int);
    }
    aidescs[2 as libc::c_int as usize] = -(1 as libc::c_int);
    ipid = ixsspawn(
        pazargs,
        aidescs.as_mut_ptr(),
        1 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if ipid < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    if frd != 0 {
        eret = fdopen(
            aidescs[1 as libc::c_int as usize],
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        eret = fdopen(
            aidescs[0 as libc::c_int as usize],
            b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if eret.is_null() {
        let mut ierr: libc::c_int = 0;
        ierr = *__errno_location();
        close(
            if frd != 0 {
                aidescs[1 as libc::c_int as usize]
            } else {
                aidescs[0 as libc::c_int as usize]
            },
        );
        kill(ipid, 9 as libc::c_int);
        ixswait(ipid as libc::c_ulong, 0 as *mut libc::c_void as *const libc::c_char);
        *__errno_location() = ierr;
        return 0 as *mut FILE;
    }
    *pipid = ipid;
    return eret;
}
