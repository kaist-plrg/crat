use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn brubeck_statsd_msg_parse(
        msg: *mut brubeck_statsd_msg,
        buffer: *mut libc::c_char,
        end: *mut libc::c_char,
        _: libc::c_double,
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut __sput: sput;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type value_t = libc::c_double;
pub type brubeck_metric_mod_t = libc::c_uint;
pub const BRUBECK_MOD_RELATIVE_VALUE: brubeck_metric_mod_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_statsd_msg {
    pub key: *mut libc::c_char,
    pub key_len: uint16_t,
    pub type_0: uint16_t,
    pub value: value_t,
    pub sample_freq: value_t,
    pub modifiers: uint8_t,
}
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
unsafe extern "C" fn must_parse(
    mut msg_text: *const libc::c_char,
    mut value: libc::c_double,
    mut sample: libc::c_double,
    mut modifiers: uint8_t,
) {
    let mut msg: brubeck_statsd_msg = brubeck_statsd_msg {
        key: 0 as *mut libc::c_char,
        key_len: 0,
        type_0: 0,
        value: 0.,
        sample_freq: 0.,
        modifiers: 0,
    };
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = strlen(msg_text);
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        msg_text as *const libc::c_void,
        len,
    );
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
        .name = if !msg_text.is_null() {
        msg_text
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 15 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"brubeck_statsd_msg_parse(&msg, buffer, buffer + len, 0.001) == 0\0"
        as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(brubeck_statsd_msg_parse(
        &mut msg,
        buffer.as_mut_ptr(),
        buffer.as_mut_ptr().offset(len as isize),
        0.001f64,
    ) == 0 as libc::c_int)
    {
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
    }
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
        .name = if !(b"msg.value == expected\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"msg.value == expected\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 16 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"value == msg.value\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(value == msg.value) {
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
    }
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
        .name = if !(b"msg.sample_rate == expected\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"msg.sample_rate == expected\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 17 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample == msg.sample_freq\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample == msg.sample_freq) {
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
    }
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
        .name = if !(b"msg.modifiers == expected\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"msg.modifiers == expected\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 18 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"modifiers == msg.modifiers\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(modifiers as libc::c_int == msg.modifiers as libc::c_int) {
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
unsafe extern "C" fn must_not_parse(mut msg_text: *const libc::c_char) {
    let mut msg: brubeck_statsd_msg = brubeck_statsd_msg {
        key: 0 as *mut libc::c_char,
        key_len: 0,
        type_0: 0,
        value: 0.,
        sample_freq: 0.,
        modifiers: 0,
    };
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = strlen(msg_text);
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        msg_text as *const libc::c_void,
        len,
    );
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
        .name = if !msg_text.is_null() {
        msg_text
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 29 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"brubeck_statsd_msg_parse(&msg, buffer, buffer + len, 0.001) < 0\0"
        as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(brubeck_statsd_msg_parse(
        &mut msg,
        buffer.as_mut_ptr(),
        buffer.as_mut_ptr().offset(len as isize),
        0.001f64,
    ) < 0 as libc::c_int)
    {
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
pub unsafe extern "C" fn test_statsd_msg__parse_strings() {
    must_parse(
        b"github.auth.fingerprint.sha1:1|c\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_double,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"github.auth.fingerprint.sha1:1|c|@0.1\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_double,
        10.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"github.auth.fingerprint.sha1:1|g\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_double,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"lol:2|ms\0" as *const u8 as *const libc::c_char,
        0.002f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.is.sparta:199812|C\0" as *const u8 as *const libc::c_char,
        199812 as libc::c_int as libc::c_double,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.is.sparta:0012|h\0" as *const u8 as *const libc::c_char,
        12 as libc::c_int as libc::c_double,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.is.sparta:23.23|g\0" as *const u8 as *const libc::c_char,
        23.23f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.is.sparta:0.232030|g\0" as *const u8 as *const libc::c_char,
        0.23203f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@0.025\0" as *const u8
            as *const libc::c_char,
        1234567.89f64,
        40.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@0.25\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        4.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@0.01\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        100.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@000.0100\0" as *const u8
            as *const libc::c_char,
        1234567.89f64,
        100.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@1.0\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@1\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g|@1.\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:|g\0" as *const u8 as *const libc::c_char,
        0.0f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"this.are.some.floats:1234567.89|g\0" as *const u8 as *const libc::c_char,
        1234567.89f64,
        1.0f64,
        0 as libc::c_int as uint8_t,
    );
    must_parse(
        b"gauge.increment:+1|g\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_double,
        1.0f64,
        BRUBECK_MOD_RELATIVE_VALUE as libc::c_int as uint8_t,
    );
    must_parse(
        b"gauge.decrement:-1|g\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as libc::c_double,
        1.0f64,
        BRUBECK_MOD_RELATIVE_VALUE as libc::c_int as uint8_t,
    );
    must_not_parse(
        b"this.are.some.floats:12.89.23|g\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:12.89|a\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:12.89|msdos\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:12.89g|g\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(b"this.are.some.floats:12.89|\0" as *const u8 as *const libc::c_char);
    must_not_parse(b"this.are.some.floats:12.89\0" as *const u8 as *const libc::c_char);
    must_not_parse(
        b"this.are.some.floats:12.89 |g\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(b"this.are.some.floats|g\0" as *const u8 as *const libc::c_char);
    must_not_parse(
        b"this.are.some.floats:1.0|g|1.0\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|0.1\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@0.1.1\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@0.1@\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@0.1125.2\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@0.1125.2\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@1.23\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@3.0\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@-3.0\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@-1.0\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@-0.23\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@0.0\0" as *const u8 as *const libc::c_char,
    );
    must_not_parse(
        b"this.are.some.floats:1.0|g|@0\0" as *const u8 as *const libc::c_char,
    );
}
