use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    static mut zSlocalname: *const libc::c_char;
    fn espopen(
        pazargs: *mut *const libc::c_char,
        frd: boolean,
        pipid: *mut pid_t,
    ) -> *mut FILE;
    fn ixswait(ipid: libc::c_ulong, zreport: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
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
pub type pointer = *mut libc::c_void;
pub type time_t = __time_t;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn fsysdep_mail(
    mut zto: *const libc::c_char,
    mut zsubject: *const libc::c_char,
    mut cstrs: libc::c_int,
    mut paz: *mut *const libc::c_char,
) -> boolean {
    let mut pazargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ztok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cargs: size_t = 0;
    let mut iarg: size_t = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut ipid: pid_t = 0;
    let mut itime: time_t = 0;
    let mut i: libc::c_int = 0;
    zcopy = zbufcpy(b"/usr/lib/sendmail -t\0" as *const u8 as *const libc::c_char);
    cargs = 0 as libc::c_int as size_t;
    ztok = strtok(zcopy, b" \t\0" as *const u8 as *const libc::c_char);
    while !ztok.is_null() {
        cargs = cargs.wrapping_add(1);
        cargs;
        ztok = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
    }
    pazargs = xmalloc(
        cargs
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    memcpy(
        zcopy as *mut libc::c_void,
        b"/usr/lib/sendmail -t\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
    );
    ztok = strtok(zcopy, b" \t\0" as *const u8 as *const libc::c_char);
    iarg = 0 as libc::c_int as size_t;
    while !ztok.is_null() {
        let ref mut fresh0 = *pazargs.offset(iarg as isize);
        *fresh0 = ztok;
        ztok = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
        iarg = iarg.wrapping_add(1);
        iarg;
    }
    let ref mut fresh1 = *pazargs.offset(iarg as isize);
    *fresh1 = 0 as *mut libc::c_char;
    e = espopen(pazargs as *mut *const libc::c_char, 0 as libc::c_int, &mut ipid);
    ubuffree(zcopy);
    xfree(pazargs as pointer);
    if e.is_null() {
        ulog(
            LOG_ERROR,
            b"espopen (%s): %s\0" as *const u8 as *const libc::c_char,
            b"/usr/lib/sendmail -t\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    fprintf(e, b"To: %s\n\0" as *const u8 as *const libc::c_char, zto);
    fprintf(e, b"Subject: %s\n\0" as *const u8 as *const libc::c_char, zsubject);
    fprintf(e, b"\n\0" as *const u8 as *const libc::c_char);
    time(&mut itime);
    fprintf(
        e,
        b"Message from UUCP on %s %s\n\0" as *const u8 as *const libc::c_char,
        zSlocalname,
        ctime(&mut itime),
    );
    i = 0 as libc::c_int;
    while i < cstrs {
        fputs(*paz.offset(i as isize), e);
        i += 1;
        i;
    }
    fclose(e);
    return (ixswait(
        ipid as libc::c_ulong,
        b"/usr/lib/sendmail -t\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
}
