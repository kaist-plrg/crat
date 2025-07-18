use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
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
pub unsafe extern "C" fn snoopy_datasource_domain(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut hostname: [libc::c_char; 66] = [0; 66];
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut retVal: libc::c_int = 0;
    let mut hostnameLen: libc::c_int = 0;
    retVal = gethostname(hostname.as_mut_ptr(), 64 as libc::c_int as size_t);
    if 0 as libc::c_int != retVal {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ gethostname(): %d)\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
    }
    hostname[(64 as libc::c_int + 2 as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    hostnameLen = strlen(hostname.as_mut_ptr()) as libc::c_int;
    if 0 as libc::c_int == hostnameLen {
        snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"Got empty hostname\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if hostnameLen > 64 as libc::c_int + 2 as libc::c_int - 2 as libc::c_int {
        snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"INTERNAL ERROR: Got too long hostname, length: %d\0" as *const u8
                as *const libc::c_char,
            hostnameLen,
        );
        return -(1 as libc::c_int);
    }
    hostname[hostnameLen as usize] = '.' as i32 as libc::c_char;
    hostname[(hostnameLen + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    fp = fopen(
        b"/etc/hosts\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"Unable to open file for reading: %s\0" as *const u8 as *const libc::c_char,
            b"/etc/hosts\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut linePtr: *const libc::c_char = 0 as *const libc::c_char;
    let mut hashPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineEntryPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savePtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domainPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        linePtr = fgets(
            line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            fp,
        );
        if linePtr.is_null() {
            break;
        }
        hashPtr = strchr(linePtr, '#' as i32);
        if !hashPtr.is_null() {
            *hashPtr = '\0' as i32 as libc::c_char;
        }
        lineEntryPtr = strcasestr(linePtr, hostname.as_mut_ptr());
        if lineEntryPtr.is_null() {
            continue;
        }
        strtok_r(
            lineEntryPtr,
            b" \t\n\r\0" as *const u8 as *const libc::c_char,
            &mut savePtr,
        );
        domainPtr = lineEntryPtr.offset(strlen(hostname.as_mut_ptr()) as isize);
        break;
    }
    fclose(fp);
    if !domainPtr.is_null() {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            domainPtr,
        )
    } else {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(none)\0" as *const u8 as *const libc::c_char,
        )
    };
}
