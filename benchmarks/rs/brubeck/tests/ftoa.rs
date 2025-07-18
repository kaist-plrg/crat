use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn brubeck_ftoa(outbuf: *mut libc::c_char, f: libc::c_float) -> libc::c_int;
    static mut __sput: sput;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput {
    pub out: *mut FILE,
    pub initialized: libc::c_char,
    pub overall: sput_overall,
    pub suite: sput_suite,
    pub test: sput_test,
    pub check: sput_check,
    pub time: sput_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_time {
    pub start: time_t,
    pub end: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_check {
    pub name: *const libc::c_char,
    pub cond: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub line: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_test {
    pub name: *const libc::c_char,
    pub nr: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_suite {
    pub name: *const libc::c_char,
    pub nr: libc::c_ulong,
    pub checks: libc::c_ulong,
    pub ok: libc::c_ulong,
    pub nok: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_overall {
    pub checks: libc::c_ulong,
    pub suites: libc::c_ulong,
    pub ok: libc::c_ulong,
    pub nok: libc::c_ulong,
}
unsafe extern "C" fn check_eq(mut f: libc::c_float, mut str: *const libc::c_char) {
    let mut buf: [libc::c_char; 16] = [0; 16];
    brubeck_ftoa(buf.as_mut_ptr(), f);
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !str.is_null() {
        str
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 7 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"strcmp(str, buf) == 0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(strcmp(str, buf.as_mut_ptr()) == 0 as libc::c_int) {
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    };
}
pub unsafe extern "C" fn test_ftoa() {
    check_eq(0.0f64 as libc::c_float, b"0\0" as *const u8 as *const libc::c_char);
    check_eq(15.0f64 as libc::c_float, b"15\0" as *const u8 as *const libc::c_char);
    check_eq(15.5f64 as libc::c_float, b"15.5\0" as *const u8 as *const libc::c_char);
    check_eq(
        15.505f64 as libc::c_float,
        b"15.505\0" as *const u8 as *const libc::c_char,
    );
    check_eq(0.125f64 as libc::c_float, b"0.125\0" as *const u8 as *const libc::c_char);
    check_eq(
        1234.567f64 as libc::c_float,
        b"1234.567\0" as *const u8 as *const libc::c_char,
    );
    check_eq(
        99999.999f64 as libc::c_float,
        b"100000\0" as *const u8 as *const libc::c_char,
    );
    check_eq(0.999f64 as libc::c_float, b"0.999\0" as *const u8 as *const libc::c_char);
}
