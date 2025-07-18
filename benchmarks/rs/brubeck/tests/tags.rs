use ::libc;
extern "C" {
    pub type brubeck_tags_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn brubeck_get_tag_set(
        _: *mut brubeck_tags_t,
        key_str: *const libc::c_char,
        key_len: uint16_t,
    ) -> *const brubeck_tag_set;
    fn brubeck_parse_tags(_: *mut libc::c_char, _: uint16_t) -> *mut brubeck_tag_set;
    fn brubeck_tag_offset(_: *const libc::c_char) -> uint16_t;
    fn brubeck_tags_create(size: uint64_t) -> *mut brubeck_tags_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut __sput: sput;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag_set {
    pub index: uint32_t,
    pub tag_len: uint16_t,
    pub num_tags: uint16_t,
    pub tags: [brubeck_tag; 0],
}
pub type size_t = libc::c_ulong;
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
pub type time_t = __time_t;
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
unsafe extern "C" fn check_tags_equal(
    mut x: *const brubeck_tag_set,
    mut y: *const brubeck_tag_set,
    mut description: *const libc::c_char,
) {
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
        .name = if !description.is_null() {
        description
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 7 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"y->num_tags == x->num_tags\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !((*y).num_tags as libc::c_int == (*x).num_tags as libc::c_int) {
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
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*y).num_tags as libc::c_int {
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
            .name = if !description.is_null() {
            description
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 9 as libc::c_int as libc::c_ulong;
        __sput
            .check
            .cond = b"strcmp(x->tags[i].key, y->tags[i].key) == 0\0" as *const u8
            as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(strcmp(
            (*((*x).tags).as_ptr().offset(i as isize)).key,
            (*((*y).tags).as_ptr().offset(i as isize)).key,
        ) == 0 as libc::c_int)
        {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
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
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
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
            .name = if !description.is_null() {
            description
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 11 as libc::c_int as libc::c_ulong;
        __sput
            .check
            .cond = b"strcmp(x->tags[i].value, y->tags[i].value) == 0\0" as *const u8
            as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(strcmp(
            (*((*x).tags).as_ptr().offset(i as isize)).value,
            (*((*y).tags).as_ptr().offset(i as isize)).value,
        ) == 0 as libc::c_int)
        {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
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
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn check_equal(
    mut x: *const brubeck_tag_set,
    mut y: *const brubeck_tag_set,
    mut description: *const libc::c_char,
) {
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
        .name = if !description.is_null() {
        description
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 18 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"(x && !y) || (y && !x)\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-if\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !x.is_null() && y.is_null() || !y.is_null() && x.is_null() {
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
        .name = if !description.is_null() {
        description
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 19 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"y->tag_len == x->tag_len\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !((*y).tag_len as libc::c_int == (*x).tag_len as libc::c_int) {
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
    check_tags_equal(x, y, description);
}
pub unsafe extern "C" fn parse(
    mut source: *const libc::c_char,
) -> *const brubeck_tag_set {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: uint16_t = strlen(source) as uint16_t;
    str = strdup(source);
    return brubeck_parse_tags(str, len);
}
unsafe extern "C" fn check_parse(
    mut source: *const libc::c_char,
    mut expected_without_tags: *mut brubeck_tag_set,
    mut tag_arr: *const brubeck_tag,
) {
    let mut expected: *mut brubeck_tag_set = malloc(
        (::std::mem::size_of::<brubeck_tag_set>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<brubeck_tag>() as libc::c_ulong)
                    .wrapping_mul((*expected_without_tags).num_tags as libc::c_ulong),
            ),
    ) as *mut brubeck_tag_set;
    memcpy(
        expected as *mut libc::c_void,
        expected_without_tags as *const libc::c_void,
        ::std::mem::size_of::<brubeck_tag_set>() as libc::c_ulong,
    );
    memcpy(
        ((*expected).tags).as_mut_ptr() as *mut libc::c_void,
        tag_arr as *const libc::c_void,
        (::std::mem::size_of::<brubeck_tag>() as libc::c_ulong)
            .wrapping_mul((*expected_without_tags).num_tags as libc::c_ulong),
    );
    let mut tags: *const brubeck_tag_set = parse(source);
    check_equal(expected, tags, source);
}
pub unsafe extern "C" fn test_tag_parsing() {
    check_parse(
        b"\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 0 as libc::c_int as uint16_t,
                num_tags: 0 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [].as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b"foo\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 3 as libc::c_int as uint16_t,
                num_tags: 0 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [].as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b"foo=bar\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 7 as libc::c_int as uint16_t,
                num_tags: 1 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b",foo=bar\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 8 as libc::c_int as uint16_t,
                num_tags: 1 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b",foo=bar,\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 9 as libc::c_int as uint16_t,
                num_tags: 1 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b"s,foo=bar,baz=42\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 16 as libc::c_int as uint16_t,
                num_tags: 2 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = brubeck_tag {
                    key: b"baz\0" as *const u8 as *const libc::c_char,
                    value: b"42\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b",foo=bar,,baz=42,\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 17 as libc::c_int as uint16_t,
                num_tags: 2 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = brubeck_tag {
                    key: b"baz\0" as *const u8 as *const libc::c_char,
                    value: b"42\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b"s,junk,=,junk=,=junk\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 20 as libc::c_int as uint16_t,
                num_tags: 0 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [].as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b"s,foo===bar\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 11 as libc::c_int as uint16_t,
                num_tags: 1 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
    check_parse(
        b"foo=bar,\0" as *const u8 as *const libc::c_char,
        &mut {
            let mut init = brubeck_tag_set {
                index: 0,
                tag_len: 8 as libc::c_int as uint16_t,
                num_tags: 1 as libc::c_int as uint16_t,
                tags: [],
            };
            init
        },
        [
            {
                let mut init = brubeck_tag {
                    key: b"foo\0" as *const u8 as *const libc::c_char,
                    value: b"bar\0" as *const u8 as *const libc::c_char,
                };
                init
            },
        ]
            .as_mut_ptr() as *const brubeck_tag,
    );
}
pub unsafe extern "C" fn get_tag_set(
    mut tags: *mut brubeck_tags_t,
    mut tag_str: *const libc::c_char,
) -> *const brubeck_tag_set {
    let mut tag_str_len: uint16_t = strlen(tag_str) as uint16_t;
    return brubeck_get_tag_set(tags, tag_str, tag_str_len);
}
pub unsafe extern "C" fn test_tag_storage() {
    let mut tags: *mut brubeck_tags_t = brubeck_tags_create(
        1024 as libc::c_int as uint64_t,
    );
    let mut t1: *const brubeck_tag_set = 0 as *const brubeck_tag_set;
    let mut t2: *const brubeck_tag_set = 0 as *const brubeck_tag_set;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    str = b"s,foo=bar\0" as *const u8 as *const libc::c_char;
    t1 = get_tag_set(tags, str);
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
        .name = if !(b"not null\0" as *const u8 as *const libc::c_char).is_null() {
        b"not null\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 91 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"t1 == NULL\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-if\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if t1.is_null() {
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
    check_equal(t1, parse(b",foo=bar\0" as *const u8 as *const libc::c_char), str);
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
        .name = if !(b"index\0" as *const u8 as *const libc::c_char).is_null() {
        b"index\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 93 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"t1->index == 1\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !((*t1).index == 1 as libc::c_int as libc::c_uint) {
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
    t2 = get_tag_set(tags, str);
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
        .name = if !(b"not null\0" as *const u8 as *const libc::c_char).is_null() {
        b"not null\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 96 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"t2 == NULL\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-if\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if t2.is_null() {
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
    check_equal(t2, parse(b",foo=bar\0" as *const u8 as *const libc::c_char), str);
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
        .name = if !(b"caching\0" as *const u8 as *const libc::c_char).is_null() {
        b"caching\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 98 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"t1 == t2\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(t1 == t2) {
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
    t2 = get_tag_set(tags, b"s,foo=bar,\0" as *const u8 as *const libc::c_char);
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
        .name = if !(b"index\0" as *const u8 as *const libc::c_char).is_null() {
        b"index\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 104 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"t2->index == 2\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !((*t2).index == 2 as libc::c_int as libc::c_uint) {
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
    check_tags_equal(
        t1,
        t2,
        b"equivalent except index\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn check_tag_offset(
    mut str: *const libc::c_char,
    offset: uint16_t,
) {
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
    __sput.check.line = 109 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"brubeck_tag_offset(str) == offset\0" as *const u8
        as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(brubeck_tag_offset(str) as libc::c_int == offset as libc::c_int) {
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
pub unsafe extern "C" fn test_tag_offset() {
    check_tag_offset(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as uint16_t,
    );
    check_tag_offset(
        b"no.tags\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as uint16_t,
    );
    check_tag_offset(
        b"has.tags,foo=bar,baz=42\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as uint16_t,
    );
    check_tag_offset(
        b"has.tags#foo=bar,baz=42\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as uint16_t,
    );
}
