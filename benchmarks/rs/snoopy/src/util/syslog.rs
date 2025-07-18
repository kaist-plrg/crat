use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopy_util_syslog_convertFacilityToInt(
    mut facilityStr: *const libc::c_char,
) -> libc::c_int {
    let mut facilityStrAdj: *const libc::c_char = 0 as *const libc::c_char;
    let mut facilityInt: libc::c_int = 0;
    facilityStrAdj = facilityStr;
    if '_' as i32 == *facilityStr.offset(3 as libc::c_int as isize) as libc::c_int {
        facilityStrAdj = &*facilityStr.offset(4 as libc::c_int as isize)
            as *const libc::c_char;
    }
    if strcmp(facilityStrAdj, b"AUTH\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (4 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"AUTHPRIV\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (10 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"CRON\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (9 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"DAEMON\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (3 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"FTP\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (11 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"KERN\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (0 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL0\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (16 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL1\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (17 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL2\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (18 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL3\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (19 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL4\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (20 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL5\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (21 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL6\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (22 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LOCAL7\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (23 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"LPR\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (6 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"MAIL\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (2 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"NEWS\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (7 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"SYSLOG\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (5 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"USER\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (1 as libc::c_int) << 3 as libc::c_int;
    } else if strcmp(facilityStrAdj, b"UUCP\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        facilityInt = (8 as libc::c_int) << 3 as libc::c_int;
    } else {
        facilityInt = -(1 as libc::c_int);
    }
    return facilityInt;
}
pub unsafe extern "C" fn snoopy_util_syslog_convertFacilityToStr(
    mut facilityInt: libc::c_int,
) -> *const libc::c_char {
    let mut facilityStr: *const libc::c_char = 0 as *const libc::c_char;
    if (4 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"AUTH\0" as *const u8 as *const libc::c_char;
    } else if (10 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"AUTHPRIV\0" as *const u8 as *const libc::c_char;
    } else if (9 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"CRON\0" as *const u8 as *const libc::c_char;
    } else if (3 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"DAEMON\0" as *const u8 as *const libc::c_char;
    } else if (11 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"FTP\0" as *const u8 as *const libc::c_char;
    } else if (0 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"KERN\0" as *const u8 as *const libc::c_char;
    } else if (16 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL0\0" as *const u8 as *const libc::c_char;
    } else if (17 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL1\0" as *const u8 as *const libc::c_char;
    } else if (18 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL2\0" as *const u8 as *const libc::c_char;
    } else if (19 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL3\0" as *const u8 as *const libc::c_char;
    } else if (20 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL4\0" as *const u8 as *const libc::c_char;
    } else if (21 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL5\0" as *const u8 as *const libc::c_char;
    } else if (22 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL6\0" as *const u8 as *const libc::c_char;
    } else if (23 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LOCAL7\0" as *const u8 as *const libc::c_char;
    } else if (6 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"LPR\0" as *const u8 as *const libc::c_char;
    } else if (2 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"MAIL\0" as *const u8 as *const libc::c_char;
    } else if (7 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"NEWS\0" as *const u8 as *const libc::c_char;
    } else if (5 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"SYSLOG\0" as *const u8 as *const libc::c_char;
    } else if (1 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"USER\0" as *const u8 as *const libc::c_char;
    } else if (8 as libc::c_int) << 3 as libc::c_int == facilityInt {
        facilityStr = b"UUCP\0" as *const u8 as *const libc::c_char;
    } else {
        facilityStr = b"(invalid)\0" as *const u8 as *const libc::c_char;
    }
    return facilityStr;
}
pub unsafe extern "C" fn snoopy_util_syslog_convertLevelToInt(
    mut levelStr: *const libc::c_char,
) -> libc::c_int {
    let mut levelStrAdj: *const libc::c_char = 0 as *const libc::c_char;
    let mut levelInt: libc::c_int = 0;
    levelStrAdj = levelStr;
    if '_' as i32 == *levelStr.offset(3 as libc::c_int as isize) as libc::c_int {
        levelStrAdj = &*levelStr.offset(4 as libc::c_int as isize)
            as *const libc::c_char;
    }
    if strcmp(levelStrAdj, b"EMERG\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 0 as libc::c_int;
    } else if strcmp(levelStrAdj, b"ALERT\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 1 as libc::c_int;
    } else if strcmp(levelStrAdj, b"CRIT\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 2 as libc::c_int;
    } else if strcmp(levelStrAdj, b"ERR\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 3 as libc::c_int;
    } else if strcmp(levelStrAdj, b"WARNING\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 4 as libc::c_int;
    } else if strcmp(levelStrAdj, b"NOTICE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 5 as libc::c_int;
    } else if strcmp(levelStrAdj, b"INFO\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 6 as libc::c_int;
    } else if strcmp(levelStrAdj, b"DEBUG\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        levelInt = 7 as libc::c_int;
    } else {
        levelInt = 6 as libc::c_int;
    }
    return levelInt;
}
pub unsafe extern "C" fn snoopy_util_syslog_convertLevelToStr(
    mut levelInt: libc::c_int,
) -> *const libc::c_char {
    let mut levelStr: *const libc::c_char = 0 as *const libc::c_char;
    if 0 as libc::c_int == levelInt {
        levelStr = b"EMERG\0" as *const u8 as *const libc::c_char;
    } else if 1 as libc::c_int == levelInt {
        levelStr = b"ALERT\0" as *const u8 as *const libc::c_char;
    } else if 2 as libc::c_int == levelInt {
        levelStr = b"CRIT\0" as *const u8 as *const libc::c_char;
    } else if 3 as libc::c_int == levelInt {
        levelStr = b"ERR\0" as *const u8 as *const libc::c_char;
    } else if 4 as libc::c_int == levelInt {
        levelStr = b"WARNING\0" as *const u8 as *const libc::c_char;
    } else if 5 as libc::c_int == levelInt {
        levelStr = b"NOTICE\0" as *const u8 as *const libc::c_char;
    } else if 6 as libc::c_int == levelInt {
        levelStr = b"INFO\0" as *const u8 as *const libc::c_char;
    } else if 7 as libc::c_int == levelInt {
        levelStr = b"DEBUG\0" as *const u8 as *const libc::c_char;
    } else {
        levelStr = b"(invalid)\0" as *const u8 as *const libc::c_char;
    }
    return levelStr;
}
