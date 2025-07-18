use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    static mut afSignal: [sig_atomic_t; 5];
    fn esysdep_fopen(
        zfile: *const libc::c_char,
        fpublic: boolean,
        fappend: boolean,
        fmkdirs: boolean,
    ) -> *mut FILE;
    fn fsysdep_sync(e: openfile_t, zmsg: *const libc::c_char) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type boolean = libc::c_int;
pub type openfile_t = *mut FILE;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub static mut copy_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: copy.c,v 1.22 2002/03/05 19:10:41 ian Rel $\0")
};
pub unsafe extern "C" fn fcopy_file(
    mut zfrom: *const libc::c_char,
    mut zto: *const libc::c_char,
    mut fpublic: boolean,
    mut fmkdirs: boolean,
    mut fsignals: boolean,
) -> boolean {
    let mut efrom: *mut FILE = 0 as *mut FILE;
    let mut fret: boolean = 0;
    efrom = fopen(zfrom, b"r\0" as *const u8 as *const libc::c_char);
    if efrom.is_null() {
        ulog(
            LOG_ERROR,
            b"fopen (%s): %s\0" as *const u8 as *const libc::c_char,
            zfrom,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    fret = fcopy_open_file(efrom, zto, fpublic, fmkdirs, fsignals);
    fclose(efrom);
    return fret;
}
pub unsafe extern "C" fn fcopy_open_file(
    mut efrom: *mut FILE,
    mut zto: *const libc::c_char,
    mut fpublic: boolean,
    mut fmkdirs: boolean,
    mut fsignals: boolean,
) -> boolean {
    let mut eto: *mut FILE = 0 as *mut FILE;
    let mut ab: [libc::c_char; 8192] = [0; 8192];
    let mut c: size_t = 0;
    eto = esysdep_fopen(zto, fpublic, 0 as libc::c_int, fmkdirs);
    if eto.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        c = fread(
            ab.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            efrom,
        );
        if !(c != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if fwrite(
            ab.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            c,
            eto,
        ) != c
        {
            ulog(
                LOG_ERROR,
                b"fwrite: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            fclose(eto);
            remove(zto);
            return 0 as libc::c_int;
        }
        if fsignals != 0
            && (afSignal[0 as libc::c_int as usize] != 0
                || afSignal[1 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0)
        {
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
            fclose(eto);
            remove(zto);
            return 0 as libc::c_int;
        }
    }
    if fsysdep_sync(eto, zto) == 0 {
        fclose(eto);
        remove(zto);
        return 0 as libc::c_int;
    }
    if fclose(eto) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"fclose: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        remove(zto);
        return 0 as libc::c_int;
    }
    if ferror(efrom) != 0 {
        ulog(
            LOG_ERROR,
            b"fread: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        remove(zto);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
