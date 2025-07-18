use ::libc;
extern "C" {
    fn snoopy_error_handler(errorMsg: *const libc::c_char);
    fn snoopy_datasourceregistry_doesNameExist(
        datasourceName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasourceregistry_callByName(
        datasourceName: *const libc::c_char,
        result: *mut libc::c_char,
        datasourceArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_string_append(
        destString: *mut libc::c_char,
        destStringBufSize: size_t,
        appendThis: *const libc::c_char,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_message_generateFromFormat(
    mut logMessage: *mut libc::c_char,
    mut logMessageBufSize: size_t,
    mut logMessageFormat: *mut libc::c_char,
) {
    let mut fmtPos_cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmtPos_nextFormatTag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmtPos_nextFormatTagClose: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retVal: libc::c_int = 0;
    fmtPos_cur = logMessageFormat;
    fmtPos_nextFormatTag = logMessageFormat;
    while strlen(fmtPos_nextFormatTag) > 0 as libc::c_int as libc::c_ulong {
        let mut lengthToCopy: libc::c_int = 0;
        let mut fmtStaticText: [libc::c_char; 2048] = [0; 2048];
        let mut dataSourceTag: [libc::c_char; 100] = [0; 100];
        let mut dataSourceTagLength: libc::c_int = 0;
        let mut fmtPos_dataSourceTagArg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dataSourceNamePtr: *const libc::c_char = 0 as *const libc::c_char;
        let mut dataSourceArgPtr: *const libc::c_char = 0 as *const libc::c_char;
        let mut dataSourceArg: [libc::c_char; 1024] = [0; 1024];
        let mut dataSourceMsg: [libc::c_char; 2048] = [0; 2048];
        fmtPos_nextFormatTag = strstr(
            fmtPos_cur,
            b"%{\0" as *const u8 as *const libc::c_char,
        );
        if fmtPos_nextFormatTag.is_null() {
            snoopy_message_append(logMessage, logMessageBufSize, fmtPos_cur);
            return;
        }
        lengthToCopy = (fmtPos_nextFormatTag.offset_from(fmtPos_cur) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_int;
        if lengthToCopy
            > logMessageBufSize.wrapping_sub(strlen(logMessage)) as libc::c_int
        {
            lengthToCopy = logMessageBufSize.wrapping_sub(strlen(logMessage))
                as libc::c_int;
        }
        fmtStaticText[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        snprintf(
            fmtStaticText.as_mut_ptr(),
            lengthToCopy as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            fmtPos_cur,
        );
        snoopy_message_append(logMessage, logMessageBufSize, fmtStaticText.as_mut_ptr());
        fmtPos_nextFormatTagClose = strstr(
            fmtPos_nextFormatTag,
            b"}\0" as *const u8 as *const libc::c_char,
        );
        if fmtPos_nextFormatTagClose.is_null() {
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                b"[ERROR: Closing data source tag ('}') not found.]\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        dataSourceTag[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        dataSourceTagLength = (fmtPos_nextFormatTagClose
            .offset(-(1 as libc::c_int as isize))
            .offset_from(fmtPos_nextFormatTag.offset(2 as libc::c_int as isize))
            as libc::c_long + 2 as libc::c_int as libc::c_long) as libc::c_int;
        snprintf(
            dataSourceTag.as_mut_ptr(),
            dataSourceTagLength as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            fmtPos_nextFormatTag.offset(2 as libc::c_int as isize),
        );
        fmtPos_dataSourceTagArg = strstr(
            dataSourceTag.as_mut_ptr(),
            b":\0" as *const u8 as *const libc::c_char,
        );
        if fmtPos_dataSourceTagArg.is_null() {
            dataSourceNamePtr = dataSourceTag.as_mut_ptr();
            dataSourceArg[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            dataSourceArgPtr = dataSourceArg.as_mut_ptr();
        } else {
            *fmtPos_dataSourceTagArg
                .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            dataSourceNamePtr = dataSourceTag.as_mut_ptr();
            dataSourceArgPtr = fmtPos_dataSourceTagArg.offset(1 as libc::c_int as isize);
        }
        if snoopy_datasourceregistry_doesNameExist(dataSourceNamePtr) == 0 {
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                b"[ERROR: Data source '\0" as *const u8 as *const libc::c_char,
            );
            snoopy_message_append(logMessage, logMessageBufSize, dataSourceNamePtr);
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                b"' not found.]\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        dataSourceMsg[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        retVal = snoopy_datasourceregistry_callByName(
            dataSourceNamePtr,
            dataSourceMsg.as_mut_ptr(),
            dataSourceArgPtr,
        );
        if retVal < 0 as libc::c_int {
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                b"[ERROR: Data source '\0" as *const u8 as *const libc::c_char,
            );
            snoopy_message_append(logMessage, logMessageBufSize, dataSourceNamePtr);
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                b"' failed with the following error message: '\0" as *const u8
                    as *const libc::c_char,
            );
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                dataSourceMsg.as_mut_ptr(),
            );
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                b"']\0" as *const u8 as *const libc::c_char,
            );
        } else {
            snoopy_message_append(
                logMessage,
                logMessageBufSize,
                dataSourceMsg.as_mut_ptr(),
            );
        }
        fmtPos_cur = fmtPos_nextFormatTagClose.offset(1 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn snoopy_message_append(
    mut logMessage: *mut libc::c_char,
    mut logMessageBufSize: size_t,
    mut appendThis: *const libc::c_char,
) {
    if -(1 as libc::c_int)
        == snoopy_util_string_append(logMessage, logMessageBufSize, appendThis)
    {
        snoopy_error_handler(
            b"Maximum destination string size exceeded\0" as *const u8
                as *const libc::c_char,
        );
    }
}
