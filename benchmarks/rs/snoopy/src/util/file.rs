use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub unsafe extern "C" fn snoopy_util_file_getSmallTextFileContent(
    filePath: *const libc::c_char,
    mut contentPtrAddr: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut fileHandle: *mut FILE = 0 as *mut FILE;
    let mut contentPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errorMsgBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    contentPtr = malloc(10240 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if contentPtr.is_null() {
        contentPtr = malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        snprintf(
            contentPtr,
            1024 as libc::c_int as libc::c_ulong,
            b"Unable to malloc() %d bytes\0" as *const u8 as *const libc::c_char,
            10240 as libc::c_int,
        );
        *contentPtr
            .offset(
                (1024 as libc::c_int - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        *contentPtrAddr = contentPtr;
        return -(1 as libc::c_int);
    }
    *contentPtr.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    fileHandle = fopen(filePath, b"r\0" as *const u8 as *const libc::c_char);
    if fileHandle.is_null() {
        free(contentPtr as *mut libc::c_void);
        contentPtr = malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        errorMsgBuf = malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        *errorMsgBuf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        strerror_r(*__errno_location(), errorMsgBuf, 1024 as libc::c_int as size_t);
        *errorMsgBuf
            .offset(
                (1024 as libc::c_int - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        snprintf(
            contentPtr,
            1024 as libc::c_int as libc::c_ulong,
            b"Unable to open file %s for reading, reason: %s\0" as *const u8
                as *const libc::c_char,
            filePath,
            errorMsgBuf,
        );
        *contentPtr
            .offset(
                (1024 as libc::c_int - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        *contentPtrAddr = contentPtr;
        free(errorMsgBuf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    let mut bytesReadTotal: size_t = 0 as libc::c_int as size_t;
    while bytesReadTotal < 10240 as libc::c_int as libc::c_ulong {
        let mut bytesReadNow: size_t = 0;
        bytesReadNow = fread(
            contentPtr.offset(bytesReadTotal as isize) as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1024 as libc::c_int as libc::c_ulong,
            fileHandle,
        );
        bytesReadTotal = (bytesReadTotal as libc::c_ulong).wrapping_add(bytesReadNow)
            as size_t as size_t;
        if ferror(fileHandle) != 0 {
            free(contentPtr as *mut libc::c_void);
            contentPtr = malloc(1024 as libc::c_int as libc::c_ulong)
                as *mut libc::c_char;
            errorMsgBuf = malloc(1024 as libc::c_int as libc::c_ulong)
                as *mut libc::c_char;
            *errorMsgBuf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            strerror_r(*__errno_location(), errorMsgBuf, 1024 as libc::c_int as size_t);
            *errorMsgBuf
                .offset(
                    (1024 as libc::c_int - 1 as libc::c_int) as isize,
                ) = '\0' as i32 as libc::c_char;
            snprintf(
                contentPtr,
                1024 as libc::c_int as libc::c_ulong,
                b"Error reading file: %s\0" as *const u8 as *const libc::c_char,
                errorMsgBuf,
            );
            *contentPtr
                .offset(
                    (1024 as libc::c_int - 1 as libc::c_int) as isize,
                ) = '\0' as i32 as libc::c_char;
            *contentPtrAddr = contentPtr;
            clearerr(fileHandle);
            fclose(fileHandle);
            free(errorMsgBuf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        if feof(fileHandle) != 0 || bytesReadNow < 1024 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    if bytesReadTotal >= 10240 as libc::c_int as libc::c_ulong {
        free(contentPtr as *mut libc::c_void);
        contentPtr = malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        snprintf(
            contentPtr,
            1024 as libc::c_int as libc::c_ulong,
            b"INTERNAL ERROR: File too large for getSmallTextFileContent()\0"
                as *const u8 as *const libc::c_char,
        );
        *contentPtr
            .offset(
                (1024 as libc::c_int - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        *contentPtrAddr = contentPtr;
        fclose(fileHandle);
        return -(1 as libc::c_int);
    }
    if bytesReadTotal < (10240 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        *contentPtr.offset(bytesReadTotal as isize) = '\0' as i32 as libc::c_char;
    } else {
        *contentPtr
            .offset(
                (10240 as libc::c_int - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    fclose(fileHandle);
    *contentPtrAddr = contentPtr;
    return bytesReadTotal as libc::c_int;
}
