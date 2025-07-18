use ::libc;
extern "C" {
    fn printMessage(message: *const libc::c_char);
    fn printDiagValue(message: *const libc::c_char, value: *const libc::c_char);
    fn printNotice(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn libsnoopySo_getFilePath() -> *mut libc::c_char;
    fn etcLdSoPreload_readFile() -> *mut libc::c_char;
    fn etcLdSoPreload_writeFile(newContent: *mut libc::c_char);
    fn etcLdSoPreload_findNonCommentLineContainingString(
        content: *const libc::c_char,
        searchString: *const libc::c_char,
    ) -> *const libc::c_char;
    fn etcLdSoPreload_findEntry(
        content: *const libc::c_char,
        entry: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut g_etcLdSoPreloadPath: *const libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_cli_action_enable() -> libc::c_int {
    let mut libsnoopySoPath: *const libc::c_char = 0 as *const libc::c_char;
    let mut curEtcLdSoPreloadContent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newEtcLdSoPreloadContent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newEtcLdSoPreloadContentLength: size_t = 0;
    let mut newEtcLdSoPreloadContentLengthBuf: size_t = 0;
    let mut strPosPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    libsnoopySoPath = libsnoopySo_getFilePath();
    if access(libsnoopySoPath, 0 as libc::c_int) != 0 as libc::c_int {
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            libsnoopySoPath,
        );
        fatalError(b"File not found\0" as *const u8 as *const libc::c_char);
    }
    if access(libsnoopySoPath, 4 as libc::c_int) != 0 as libc::c_int {
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            libsnoopySoPath,
        );
        fatalError(b"File not readable\0" as *const u8 as *const libc::c_char);
    }
    curEtcLdSoPreloadContent = etcLdSoPreload_readFile();
    if !(etcLdSoPreload_findEntry(curEtcLdSoPreloadContent, libsnoopySoPath)).is_null() {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            g_etcLdSoPreloadPath,
        );
        printDiagValue(
            b"Search string\0" as *const u8 as *const libc::c_char,
            libsnoopySoPath,
        );
        printNotice(
            b"Snoopy is already enabled in /etc/ld.so.preload.\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if !(etcLdSoPreload_findNonCommentLineContainingString(
        curEtcLdSoPreloadContent,
        b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            g_etcLdSoPreloadPath,
        );
        printDiagValue(
            b"Search string\0" as *const u8 as *const libc::c_char,
            b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
        );
        fatalError(
            b"Another Snoopy instance encountered.\0" as *const u8 as *const libc::c_char,
        );
    }
    newEtcLdSoPreloadContentLength = (strlen(curEtcLdSoPreloadContent))
        .wrapping_add(strlen(libsnoopySoPath))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    newEtcLdSoPreloadContentLengthBuf = newEtcLdSoPreloadContentLength
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    newEtcLdSoPreloadContent = malloc(newEtcLdSoPreloadContentLengthBuf)
        as *mut libc::c_char;
    *newEtcLdSoPreloadContent
        .offset(newEtcLdSoPreloadContentLength as isize) = '\0' as i32 as libc::c_char;
    if strlen(curEtcLdSoPreloadContent) == 0 as libc::c_int as libc::c_ulong {
        strPosPtr = newEtcLdSoPreloadContent;
    } else {
        strncpy(
            newEtcLdSoPreloadContent,
            curEtcLdSoPreloadContent,
            newEtcLdSoPreloadContentLengthBuf,
        );
        strPosPtr = newEtcLdSoPreloadContent
            .offset(strlen(curEtcLdSoPreloadContent) as isize)
            .offset(-(1 as libc::c_int as isize));
        if *strPosPtr as libc::c_int != '\n' as i32 {
            strPosPtr = strPosPtr.offset(1);
            strPosPtr;
            *strPosPtr = '\n' as i32 as libc::c_char;
        }
        strPosPtr = strPosPtr.offset(1);
        strPosPtr;
    }
    strncpy(
        strPosPtr,
        libsnoopySoPath,
        newEtcLdSoPreloadContentLengthBuf
            .wrapping_sub(
                strPosPtr.offset_from(newEtcLdSoPreloadContent) as libc::c_long
                    as libc::c_ulong,
            ),
    );
    strPosPtr = strPosPtr.offset(strlen(libsnoopySoPath) as isize);
    *strPosPtr = '\n' as i32 as libc::c_char;
    strPosPtr = strPosPtr.offset(1);
    strPosPtr;
    *strPosPtr = '\0' as i32 as libc::c_char;
    etcLdSoPreload_writeFile(newEtcLdSoPreloadContent);
    printDiagValue(
        b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
        g_etcLdSoPreloadPath,
    );
    printDiagValue(
        b"Snoopy library path\0" as *const u8 as *const libc::c_char,
        libsnoopySoPath,
    );
    printMessage(
        b"SUCCESS: Snoopy has been enabled.\0" as *const u8 as *const libc::c_char,
    );
    free(curEtcLdSoPreloadContent as *mut libc::c_void);
    free(newEtcLdSoPreloadContent as *mut libc::c_void);
    return 0 as libc::c_int;
}
