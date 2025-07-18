use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
pub static mut g_etcLdSoPreloadPath: *const libc::c_char = 0 as *const libc::c_char;
pub static mut g_libsnoopySoPath: *const libc::c_char = 0 as *const libc::c_char;
pub static mut g_libsnoopySoHandle: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
pub unsafe extern "C" fn printMessage(message: *const libc::c_char) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printDiag(message: *const libc::c_char) {
    printf(b"[DIAG] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printDiagValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printf(b"[DIAG] %s: %s\n\0" as *const u8 as *const libc::c_char, message, value);
}
pub unsafe extern "C" fn printInfo(message: *const libc::c_char) {
    printf(b"[INFO] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printInfoValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printf(b"[INFO] %s: %s\n\0" as *const u8 as *const libc::c_char, message, value);
}
pub unsafe extern "C" fn printNotice(message: *const libc::c_char) {
    printf(b"[NOTICE] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printNoticeValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printf(b"[NOTICE] %s: %s\n\0" as *const u8 as *const libc::c_char, message, value);
}
pub unsafe extern "C" fn printSuccess(message: *const libc::c_char) {
    printf(b"[SUCCESS] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printWarning(message: *const libc::c_char) {
    fprintf(stderr, b"[WARNING] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printWarningValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    fprintf(
        stderr,
        b"[WARNING] %s: %s\n\0" as *const u8 as *const libc::c_char,
        message,
        value,
    );
}
pub unsafe extern "C" fn printError(message: *const libc::c_char) {
    fprintf(stderr, b"[ERROR] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printErrorValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    fprintf(
        stderr,
        b"[ERROR] %s: %s\n\0" as *const u8 as *const libc::c_char,
        message,
        value,
    );
}
pub unsafe extern "C" fn fatalError(message: *const libc::c_char) {
    printError(message);
    exit(127 as libc::c_int);
}
pub unsafe extern "C" fn fatalErrorValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printErrorValue(message, value);
    exit(127 as libc::c_int);
}
pub unsafe extern "C" fn libsnoopySo_getFilePath() -> *mut libc::c_char {
    let mut filePath: *mut libc::c_char = 0 as *mut libc::c_char;
    filePath = libsnoopySo_getFilePathNoCheck();
    if access(filePath, 4 as libc::c_int) != 0 as libc::c_int {
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        fatalError(
            b"Unable to access libsnoopy.so file\0" as *const u8 as *const libc::c_char,
        );
    }
    return filePath;
}
pub unsafe extern "C" fn libsnoopySo_getFilePathNoCheck() -> *mut libc::c_char {
    let mut filePath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envValue: *mut libc::c_char = 0 as *mut libc::c_char;
    envValue = getenv(
        b"SNOOPY_TEST_LIBSNOOPY_SO_PATH\0" as *const u8 as *const libc::c_char,
    );
    if !envValue.is_null() {
        printWarningValue(
            b"Using non-default path to libsnoopy.so\0" as *const u8
                as *const libc::c_char,
            envValue,
        );
        filePath = envValue;
    } else {
        filePath = b"/usr/local/lib/libsnoopy.so\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    return filePath;
}
pub unsafe extern "C" fn libsnoopySo_load() {
    g_libsnoopySoPath = libsnoopySo_getFilePath();
    g_libsnoopySoHandle = dlopen(g_libsnoopySoPath, 0x1 as libc::c_int);
    if g_libsnoopySoHandle.is_null() {
        printErrorValue(
            b"Dynamic linker error message\0" as *const u8 as *const libc::c_char,
            dlerror(),
        );
        fatalError(
            b"Unable to load shared library\0" as *const u8 as *const libc::c_char,
        );
    }
    dlerror();
}
pub unsafe extern "C" fn libsnoopySo_dlsym(
    functionName: *const libc::c_char,
) -> *mut libc::c_void {
    let mut functionPtr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    functionPtr = dlsym(g_libsnoopySoHandle, functionName);
    error = dlerror();
    if !error.is_null() {
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            g_libsnoopySoPath,
        );
        fatalError(error);
    }
    return functionPtr;
}
pub unsafe extern "C" fn etcLdSoPreload_getFilePath() -> *const libc::c_char {
    let mut envValue: *const libc::c_char = 0 as *const libc::c_char;
    envValue = getenv(
        b"SNOOPY_TEST_LD_SO_PRELOAD_PATH\0" as *const u8 as *const libc::c_char,
    );
    if !envValue.is_null() {
        printWarningValue(
            b"Using non-standard path to ld.so.preload\0" as *const u8
                as *const libc::c_char,
            envValue,
        );
        return envValue;
    } else {
        return b"/etc/ld.so.preload\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn etcLdSoPreload_readFile() -> *mut libc::c_char {
    let mut filePath: *const libc::c_char = 0 as *const libc::c_char;
    let mut fileContentLen: libc::c_int = 0;
    let mut fileContentBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    filePath = etcLdSoPreload_getFilePath();
    let mut fileHandle: *mut FILE = fopen(
        filePath,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fileHandle.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            fileContentBuf = malloc(1 as libc::c_int as libc::c_ulong)
                as *mut libc::c_char;
            *fileContentBuf
                .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            return fileContentBuf;
        } else {
            printDiagValue(
                b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
                filePath,
            );
            printDiagValue(
                b"Error message\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            fatalError(
                b"Unable to read ld.so.preload file.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if fseek(fileHandle, 0 as libc::c_int as libc::c_long, 2 as libc::c_int)
        != 0 as libc::c_int
    {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"function called\0" as *const u8 as *const libc::c_char,
            b"fseek(fh, 0, SEEK_END)\0" as *const u8 as *const libc::c_char,
        );
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fatalError(
            b"Unable to determine the size of the ld.so.preload file.\0" as *const u8
                as *const libc::c_char,
        );
    }
    fileContentLen = ftell(fileHandle) as libc::c_int;
    if fileContentLen == -(1 as libc::c_int) {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"function called\0" as *const u8 as *const libc::c_char,
            b"ftell(fh)\0" as *const u8 as *const libc::c_char,
        );
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fatalError(
            b"Unable to determine the size of the ld.so.preload file.\0" as *const u8
                as *const libc::c_char,
        );
    }
    fseek(fileHandle, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    fileContentBuf = malloc((fileContentLen + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if fileContentBuf.is_null() {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fatalError(
            b"Unable to malloc() for reading the file content.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if fileContentLen == 0 as libc::c_int {
        *fileContentBuf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        let mut freadRes: libc::c_long = fread(
            fileContentBuf as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            fileContentLen as libc::c_ulong,
            fileHandle,
        ) as libc::c_long;
        if freadRes < fileContentLen as libc::c_long {
            *fileContentBuf.offset(freadRes as isize) = '\0' as i32 as libc::c_char;
            printDiagValue(
                b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
                filePath,
            );
            fatalError(
                b"Unable to read the whole content of the file.\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            *fileContentBuf
                .offset(fileContentLen as isize) = '\0' as i32 as libc::c_char;
        }
    }
    fclose(fileHandle);
    g_etcLdSoPreloadPath = filePath;
    return fileContentBuf;
}
pub unsafe extern "C" fn etcLdSoPreload_writeFile(mut newContent: *mut libc::c_char) {
    let mut filePath: *const libc::c_char = 0 as *const libc::c_char;
    filePath = etcLdSoPreload_getFilePath();
    let mut fileHandle: *mut FILE = fopen(
        filePath,
        b"w+\0" as *const u8 as *const libc::c_char,
    );
    if fileHandle.is_null() {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fatalError(
            b"Unable to open file for writing (missing sudo, maybe?).\0" as *const u8
                as *const libc::c_char,
        );
    }
    if fprintf(fileHandle, b"%s\0" as *const u8 as *const libc::c_char, newContent)
        < 0 as libc::c_int
    {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fatalError(b"Unable to write to file.\0" as *const u8 as *const libc::c_char);
    }
    fclose(fileHandle);
}
pub unsafe extern "C" fn etcLdSoPreload_findEntry(
    mut content: *const libc::c_char,
    mut entry: *const libc::c_char,
) -> *const libc::c_char {
    let mut contentPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut entryPos: *const libc::c_char = 0 as *const libc::c_char;
    contentPos = content;
    loop {
        entryPos = strstr(contentPos, entry);
        if entryPos.is_null() {
            break;
        }
        if (entryPos == content
            || entryPos > content
                && *entryPos.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '\n' as i32)
            && (*entryPos.offset(strlen(entry) as isize) as libc::c_int == '\0' as i32
                || *entryPos.offset(strlen(entry) as isize) as libc::c_int == '\n' as i32
                || *entryPos.offset(strlen(entry) as isize) as libc::c_int == '#' as i32
                || *entryPos.offset(strlen(entry) as isize) as libc::c_int == ' ' as i32
                || *entryPos.offset(strlen(entry) as isize) as libc::c_int
                    == '\t' as i32)
        {
            return entryPos;
        }
        contentPos = entryPos.offset(strlen(entry) as isize);
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn etcLdSoPreload_findNonCommentLineContainingString(
    mut content: *const libc::c_char,
    mut searchString: *const libc::c_char,
) -> *const libc::c_char {
    let mut contentPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut foundStringPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut lineStartPtr: *const libc::c_char = 0 as *const libc::c_char;
    contentPos = content;
    loop {
        foundStringPos = strstr(contentPos, searchString);
        if foundStringPos.is_null() {
            break;
        }
        lineStartPtr = foundStringPos;
        while lineStartPtr > contentPos && *lineStartPtr as libc::c_int != '\n' as i32 {
            lineStartPtr = lineStartPtr.offset(-1);
            lineStartPtr;
        }
        if *lineStartPtr as libc::c_int == '\n' as i32 {
            lineStartPtr = lineStartPtr.offset(1);
            lineStartPtr;
        }
        if *lineStartPtr as libc::c_int != '#' as i32 {
            return lineStartPtr;
        }
        contentPos = foundStringPos.offset(strlen(searchString) as isize);
    }
    return 0 as *const libc::c_char;
}
