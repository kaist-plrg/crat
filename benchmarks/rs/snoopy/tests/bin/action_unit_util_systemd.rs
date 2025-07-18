use ::libc;
extern "C" {
    fn printDiagValue(message: *const libc::c_char, value: *const libc::c_char);
    fn printSuccess(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn snoopy_util_systemd_convertCgroupEntryToUnitName(
        cgroupEntry: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_systemd_showHelp() {
    let mut helpContent: *mut libc::c_char = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `util` :: Subunit 'systemd'\n\nDescription:\n    Mocks src/action/systemd.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit util systemd\n    snoopy-test unit util systemd --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_systemd(
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
        snoopyTestCli_action_unit_util_systemd_showHelp();
        return 0 as libc::c_int;
    }
    let mut cgroupEntry1: [libc::c_char; 27] = *::std::mem::transmute::<
        &[u8; 27],
        &mut [libc::c_char; 27],
    >(b"1:name=systemd:/init.scope\0");
    let mut expectedUnitName1: *const libc::c_char = b"init\0" as *const u8
        as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry1.as_mut_ptr(), expectedUnitName1);
    let mut cgroupEntry2: [libc::c_char; 42] = *::std::mem::transmute::<
        &[u8; 42],
        &mut [libc::c_char; 42],
    >(b"1:name=systemd:/system.slice/dbus.service\0");
    let mut expectedUnitName2: *const libc::c_char = b"dbus\0" as *const u8
        as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry2.as_mut_ptr(), expectedUnitName2);
    let mut cgroupEntry3: [libc::c_char; 59] = *::std::mem::transmute::<
        &[u8; 59],
        &mut [libc::c_char; 59],
    >(b"1:name=systemd:/user.slice/user-0.slice/session-3177.scope\0");
    let mut expectedUnitName3: *const libc::c_char = b"root\0" as *const u8
        as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry3.as_mut_ptr(), expectedUnitName3);
    let mut cgroupEntry4: [libc::c_char; 63] = *::std::mem::transmute::<
        &[u8; 63],
        &mut [libc::c_char; 63],
    >(b"1:name=systemd:/user.slice/user-57123.slice/session-3177.scope\0");
    let mut expectedUnitName4: *const libc::c_char = b"user-57123\0" as *const u8
        as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry4.as_mut_ptr(), expectedUnitName4);
    let mut cgroupEntry5: [libc::c_char; 17] = *::std::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b"1:name=systemd:/\0");
    let mut expectedUnitName5: *const libc::c_char = b"-\0" as *const u8
        as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry5.as_mut_ptr(), expectedUnitName5);
    let mut cgroupEntry6: [libc::c_char; 13] = *::std::mem::transmute::<
        &[u8; 13],
        &mut [libc::c_char; 13],
    >(b"0::/whatever\0");
    let mut expectedUnitName6: *const libc::c_char = 0 as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry6.as_mut_ptr(), expectedUnitName6);
    let mut cgroupEntry7: [libc::c_char; 93] = *::std::mem::transmute::<
        &[u8; 93],
        &mut [libc::c_char; 93],
    >(
        b"1:name=systemd:/actions_job/44e2bfe9d6f3ece192a9f762d91c9514131b8cfbbd0469b1722fd8637ef6feac\0",
    );
    let mut expectedUnitName7: *const libc::c_char = 0 as *const libc::c_char;
    mockCgroupEntryConversion(cgroupEntry7.as_mut_ptr(), expectedUnitName7);
    printSuccess(
        b"Mocking src/unit/systemd.c complete.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mockCgroupEntryConversion(
    cgroupEntry: *const libc::c_char,
    expectedUnitName: *const libc::c_char,
) {
    let mut unitName: *mut libc::c_char = snoopy_util_systemd_convertCgroupEntryToUnitName(
        cgroupEntry,
    );
    if unitName.is_null() && expectedUnitName.is_null() {
        return;
    }
    if unitName.is_null() && !expectedUnitName.is_null() {
        printDiagValue(
            b"cgroupEntry\0" as *const u8 as *const libc::c_char,
            cgroupEntry,
        );
        printDiagValue(
            b"Expected unit name\0" as *const u8 as *const libc::c_char,
            expectedUnitName,
        );
        fatalError(
            b"Unable to convert cgroup entry to Systemd unit name\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !unitName.is_null() && expectedUnitName.is_null() {
        printDiagValue(
            b"cgroupEntry\0" as *const u8 as *const libc::c_char,
            cgroupEntry,
        );
        printDiagValue(
            b"Converted unit name\0" as *const u8 as *const libc::c_char,
            unitName,
        );
        fatalError(
            b"Cgroup entry not converted to NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if 0 as libc::c_int != strcmp(expectedUnitName, unitName) {
        printDiagValue(
            b"cgroupEntry\0" as *const u8 as *const libc::c_char,
            cgroupEntry,
        );
        printDiagValue(
            b"Expected  unit name\0" as *const u8 as *const libc::c_char,
            expectedUnitName,
        );
        printDiagValue(
            b"Converted unit name\0" as *const u8 as *const libc::c_char,
            unitName,
        );
        fatalError(
            b"Unexpected unit name conversion\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !unitName.is_null() {
        free(unitName as *mut libc::c_void);
    }
}
