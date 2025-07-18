use ::libc;
extern "C" {
    fn printSuccess(message: *const libc::c_char);
    fn fatalErrorValue(message: *const libc::c_char, value: *const libc::c_char);
    fn snoopy_util_syslog_convertFacilityToInt(
        facilityStr: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_syslog_convertFacilityToStr(
        facilityInt: libc::c_int,
    ) -> *const libc::c_char;
    fn snoopy_util_syslog_convertLevelToInt(
        levelStr: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_syslog_convertLevelToStr(
        levelInt: libc::c_int,
    ) -> *const libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_syslog_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `misc` :: Subunit `syslog`\n\nDescription:\n    Mocks src/unit/syslog.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit util syslog\n    snoopy-test unit util syslog --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_syslog(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize);
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int == strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char)
    {
        snoopyTestCli_action_unit_util_syslog_showHelp();
        return 0 as libc::c_int;
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"AUTH\0" as *const u8 as *const libc::c_char,
    ) != (4 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"AUTH\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
    ) != (10 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"CRON\0" as *const u8 as *const libc::c_char,
    ) != (9 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"CRON\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"DAEMON\0" as *const u8 as *const libc::c_char,
    ) != (3 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"DAEMON\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"FTP\0" as *const u8 as *const libc::c_char,
    ) != (11 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"FTP\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"KERN\0" as *const u8 as *const libc::c_char,
    ) != (0 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"KERN\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL0\0" as *const u8 as *const libc::c_char,
    ) != (16 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL0\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL1\0" as *const u8 as *const libc::c_char,
    ) != (17 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL1\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL2\0" as *const u8 as *const libc::c_char,
    ) != (18 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL2\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL3\0" as *const u8 as *const libc::c_char,
    ) != (19 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL3\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL4\0" as *const u8 as *const libc::c_char,
    ) != (20 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL4\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL5\0" as *const u8 as *const libc::c_char,
    ) != (21 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL5\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL6\0" as *const u8 as *const libc::c_char,
    ) != (22 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL6\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL7\0" as *const u8 as *const libc::c_char,
    ) != (23 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL7\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LPR\0" as *const u8 as *const libc::c_char,
    ) != (6 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LPR\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"MAIL\0" as *const u8 as *const libc::c_char,
    ) != (2 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"MAIL\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"NEWS\0" as *const u8 as *const libc::c_char,
    ) != (7 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"NEWS\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"SYSLOG\0" as *const u8 as *const libc::c_char,
    ) != (5 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"SYSLOG\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"USER\0" as *const u8 as *const libc::c_char,
    ) != (1 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"USER\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"UUCP\0" as *const u8 as *const libc::c_char,
    ) != (8 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"UUCP\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_AUTH\0" as *const u8 as *const libc::c_char,
    ) != (4 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_AUTH\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_AUTHPRIV\0" as *const u8 as *const libc::c_char,
    ) != (10 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_AUTHPRIV\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_CRON\0" as *const u8 as *const libc::c_char,
    ) != (9 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_CRON\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_DAEMON\0" as *const u8 as *const libc::c_char,
    ) != (3 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_DAEMON\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_FTP\0" as *const u8 as *const libc::c_char,
    ) != (11 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_FTP\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_KERN\0" as *const u8 as *const libc::c_char,
    ) != (0 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_KERN\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL0\0" as *const u8 as *const libc::c_char,
    ) != (16 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL0\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL1\0" as *const u8 as *const libc::c_char,
    ) != (17 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL1\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL2\0" as *const u8 as *const libc::c_char,
    ) != (18 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL2\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL3\0" as *const u8 as *const libc::c_char,
    ) != (19 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL3\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL4\0" as *const u8 as *const libc::c_char,
    ) != (20 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL4\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL5\0" as *const u8 as *const libc::c_char,
    ) != (21 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL5\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL6\0" as *const u8 as *const libc::c_char,
    ) != (22 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL6\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL7\0" as *const u8 as *const libc::c_char,
    ) != (23 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL7\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LPR\0" as *const u8 as *const libc::c_char,
    ) != (6 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LPR\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_MAIL\0" as *const u8 as *const libc::c_char,
    ) != (2 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_MAIL\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_NEWS\0" as *const u8 as *const libc::c_char,
    ) != (7 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_NEWS\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_SYSLOG\0" as *const u8 as *const libc::c_char,
    ) != (5 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_SYSLOG\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_USER\0" as *const u8 as *const libc::c_char,
    ) != (1 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_USER\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertFacilityToInt(
        b"LOG_UUCP\0" as *const u8 as *const libc::c_char,
    ) != (8 as libc::c_int) << 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_UUCP\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (4 as libc::c_int) << 3 as libc::c_int,
            ),
            b"AUTH\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"AUTH\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (10 as libc::c_int) << 3 as libc::c_int,
            ),
            b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (9 as libc::c_int) << 3 as libc::c_int,
            ),
            b"CRON\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"CRON\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (3 as libc::c_int) << 3 as libc::c_int,
            ),
            b"DAEMON\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"DAEMON\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (11 as libc::c_int) << 3 as libc::c_int,
            ),
            b"FTP\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"FTP\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (0 as libc::c_int) << 3 as libc::c_int,
            ),
            b"KERN\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"KERN\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (16 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL0\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL0\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (17 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL1\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL1\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (18 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL2\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL2\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (19 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL3\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL3\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (20 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL4\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL4\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (21 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL5\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL5\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (22 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL6\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL6\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (23 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LOCAL7\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL7\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (6 as libc::c_int) << 3 as libc::c_int,
            ),
            b"LPR\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LPR\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (2 as libc::c_int) << 3 as libc::c_int,
            ),
            b"MAIL\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"MAIL\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (7 as libc::c_int) << 3 as libc::c_int,
            ),
            b"NEWS\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"NEWS\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (5 as libc::c_int) << 3 as libc::c_int,
            ),
            b"SYSLOG\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"SYSLOG\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (1 as libc::c_int) << 3 as libc::c_int,
            ),
            b"USER\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"USER\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertFacilityToStr(
                (8 as libc::c_int) << 3 as libc::c_int,
            ),
            b"UUCP\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"UUCP\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"EMERG\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"EMERG\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"ALERT\0" as *const u8 as *const libc::c_char,
    ) != 1 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ALERT\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"CRIT\0" as *const u8 as *const libc::c_char,
    ) != 2 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"CRIT\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(b"ERR\0" as *const u8 as *const libc::c_char)
        != 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ERR\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"WARNING\0" as *const u8 as *const libc::c_char,
    ) != 4 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"NOTICE\0" as *const u8 as *const libc::c_char,
    ) != 5 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"INFO\0" as *const u8 as *const libc::c_char,
    ) != 6 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"DEBUG\0" as *const u8 as *const libc::c_char,
    ) != 7 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_EMERG\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"EMERG\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_ALERT\0" as *const u8 as *const libc::c_char,
    ) != 1 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ALERT\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_CRIT\0" as *const u8 as *const libc::c_char,
    ) != 2 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"CRIT\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_ERR\0" as *const u8 as *const libc::c_char,
    ) != 3 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ERR\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_WARNING\0" as *const u8 as *const libc::c_char,
    ) != 4 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_NOTICE\0" as *const u8 as *const libc::c_char,
    ) != 5 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_INFO\0" as *const u8 as *const libc::c_char,
    ) != 6 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
    }
    if snoopy_util_syslog_convertLevelToInt(
        b"LOG_DEBUG\0" as *const u8 as *const libc::c_char,
    ) != 7 as libc::c_int
    {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(0 as libc::c_int),
            b"EMERG\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"EMERG\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(1 as libc::c_int),
            b"ALERT\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"ALERT\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(2 as libc::c_int),
            b"CRIT\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"CRIT\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(3 as libc::c_int),
            b"ERR\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"ERR\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(4 as libc::c_int),
            b"WARNING\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(5 as libc::c_int),
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(6 as libc::c_int),
            b"INFO\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
    }
    if 0 as libc::c_int
        != strcmp(
            snoopy_util_syslog_convertLevelToStr(7 as libc::c_int),
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        )
    {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        );
    }
    printSuccess(b"Mocking src/misc.c complete.\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
