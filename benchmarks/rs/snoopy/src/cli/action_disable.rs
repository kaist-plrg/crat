use ::libc;
extern "C" {
    fn printMessage(message: *const libc::c_char);
    fn printDiagValue(message: *const libc::c_char, value: *const libc::c_char);
    fn printInfo(message: *const libc::c_char);
    fn printNotice(message: *const libc::c_char);
    fn printWarning(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn libsnoopySo_getFilePathNoCheck() -> *mut libc::c_char;
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
    fn snoopy_util_string_copyLineFromContent(
        lineStartPtr: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn snoopy_util_string_getLineLength(
        lineStartPtr: *const libc::c_char,
    ) -> libc::c_int;
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
pub unsafe extern "C" fn snoopy_cli_action_disable() -> libc::c_int {
    let mut libsnoopySoPath: *const libc::c_char = 0 as *const libc::c_char;
    let mut curEtcLdSoPreloadContent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newEtcLdSoPreloadContent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newEtcLdSoPreloadContentLengthMax: size_t = 0;
    let mut copyLength: libc::c_uint = 0;
    let mut entryPtr: *const libc::c_char = 0 as *const libc::c_char;
    let mut entryLine: *const libc::c_char = 0 as *const libc::c_char;
    let mut srcPosPtr: *const libc::c_char = 0 as *const libc::c_char;
    let mut destPosPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut foundStringPos1: *const libc::c_char = 0 as *const libc::c_char;
    let mut foundStringPos2: *const libc::c_char = 0 as *const libc::c_char;
    libsnoopySoPath = libsnoopySo_getFilePathNoCheck();
    curEtcLdSoPreloadContent = etcLdSoPreload_readFile();
    foundStringPos1 = etcLdSoPreload_findNonCommentLineContainingString(
        curEtcLdSoPreloadContent,
        b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
    );
    if !foundStringPos1.is_null() {
        foundStringPos2 = etcLdSoPreload_findNonCommentLineContainingString(
            foundStringPos1
                .offset(snoopy_util_string_getLineLength(foundStringPos1) as isize),
            b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
        );
        if !foundStringPos2.is_null() {
            printDiagValue(
                b"Search string\0" as *const u8 as *const libc::c_char,
                b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
            );
            printDiagValue(
                b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
                g_etcLdSoPreloadPath,
            );
            fatalError(
                b"Duplicate libsnoopy.so entry encountered\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    entryPtr = etcLdSoPreload_findEntry(curEtcLdSoPreloadContent, libsnoopySoPath);
    if entryPtr.is_null() {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            g_etcLdSoPreloadPath,
        );
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            libsnoopySoPath,
        );
        printNotice(
            b"Snoopy library is already absent from the ld.so.preload file.\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    newEtcLdSoPreloadContentLengthMax = strlen(curEtcLdSoPreloadContent);
    newEtcLdSoPreloadContent = malloc(newEtcLdSoPreloadContentLengthMax)
        as *mut libc::c_char;
    *newEtcLdSoPreloadContent
        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    destPosPtr = newEtcLdSoPreloadContent;
    srcPosPtr = curEtcLdSoPreloadContent;
    copyLength = entryPtr.offset_from(srcPosPtr) as libc::c_long as libc::c_uint;
    strncpy(destPosPtr, srcPosPtr, copyLength as libc::c_ulong);
    destPosPtr = newEtcLdSoPreloadContent.offset(copyLength as isize);
    entryLine = snoopy_util_string_copyLineFromContent(entryPtr);
    srcPosPtr = entryPtr.offset(strlen(entryLine) as isize);
    copyLength = (strlen(curEtcLdSoPreloadContent))
        .wrapping_sub(
            entryPtr.offset_from(curEtcLdSoPreloadContent) as libc::c_long
                as libc::c_ulong,
        )
        .wrapping_sub(strlen(entryLine)) as libc::c_uint;
    if *srcPosPtr as libc::c_int == '\n' as i32 {
        srcPosPtr = srcPosPtr.offset(1);
        srcPosPtr;
        copyLength = copyLength.wrapping_sub(1);
        copyLength;
    }
    strncpy(destPosPtr, srcPosPtr, copyLength as libc::c_ulong);
    destPosPtr = destPosPtr.offset(copyLength as isize);
    *destPosPtr = '\0' as i32 as libc::c_char;
    etcLdSoPreload_writeFile(newEtcLdSoPreloadContent);
    printMessage(
        b"[SUCCESS] Snoopy has been removed from /etc/ld.so.preload.\0" as *const u8
            as *const libc::c_char,
    );
    printInfo(
        b"Existing processes may still have Snoopy enabled until they are restarted.\0"
            as *const u8 as *const libc::c_char,
    );
    if !(etcLdSoPreload_findNonCommentLineContainingString(
        newEtcLdSoPreloadContent,
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
        printWarning(
            b"Another Snoopy instance found in ld.so.preload file.\0" as *const u8
                as *const libc::c_char,
        );
    }
    free(curEtcLdSoPreloadContent as *mut libc::c_void);
    free(newEtcLdSoPreloadContent as *mut libc::c_void);
    return 0 as libc::c_int;
}
